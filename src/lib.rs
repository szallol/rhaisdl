use rand::Rng;
use rhai::{Dynamic, Engine, EvalAltResult};
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Scancode;
use sdl3::mouse::MouseButton;
use sdl3::pixels::Color;
use sdl3::rect::{Point, Rect};
use sdl3::video::Window;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// SDL3 context wrapper to be shared with Rhai
pub struct SDLContext {
    sdl: sdl3::Sdl,
    window: Option<Window>,
    canvas: Option<sdl3::render::Canvas<Window>>,
    event_pump: Option<EventPump>,
}

impl SDLContext {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl3::init().map_err(|e| e.to_string())?;
        Ok(SDLContext {
            sdl,
            window: None,
            canvas: None,
            event_pump: None,
        })
    }

    fn create_window(&mut self, title: &str, width: i32, height: i32) -> Result<(), String> {
        let video = self.sdl.video().map_err(|e| e.to_string())?;
        let window = video
            .window(title, width as u32, height as u32)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas();
        self.window = Some(canvas.window().to_owned());
        self.canvas = Some(canvas);
        Ok(())
    }

    fn set_draw_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas.set_draw_color(Color::RGB(r, g, b));
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn clear(&mut self) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas.clear();
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas
                .draw_rect(Rect::new(x, y, w as u32, h as u32).into())
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas
                .fill_rect(Rect::new(x, y, w as u32, h as u32))
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn draw_point(&mut self, x: i32, y: i32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas
                .draw_point(Point::new(x, y))
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas
                .draw_line(Point::new(x1, y1), Point::new(x2, y2))
                .map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn present(&mut self) -> Result<(), String> {
        if let Some(canvas) = &mut self.canvas {
            canvas.present();
            Ok(())
        } else {
            Err("Canvas not initialized".to_string())
        }
    }

    fn init_event_pump(&mut self) -> Result<(), String> {
        self.event_pump = Some(self.sdl.event_pump().map_err(|e| e.to_string())?);
        Ok(())
    }

    fn poll_event(&mut self) -> Result<bool, String> {
        if let Some(event_pump) = &mut self.event_pump {
            match event_pump.poll_event() {
                Some(Event::Quit { .. }) => Ok(false),
                Some(_) => Ok(true),
                None => Ok(true),
            }
        } else {
            Err("Event pump not initialized".to_string())
        }
    }

    fn is_key_down(&mut self, key: &str) -> Result<bool, String> {
        if let Some(event_pump) = &mut self.event_pump {
            let scancode = match key.to_lowercase().as_str() {
                "space" => Scancode::Space,
                "left" => Scancode::Left,
                "right" => Scancode::Right,
                "up" => Scancode::Up,
                "down" => Scancode::Down,
                _ => return Err(format!("Unsupported key: {}", key)),
            };
            let keyboard_state = event_pump.keyboard_state();
            Ok(keyboard_state.pressed_scancodes().any(|s| s == scancode))
        } else {
            Err("Event pump not initialized".to_string())
        }
    }

    fn is_mouse_button_down(&mut self, button: &str) -> Result<bool, String> {
        if let Some(event_pump) = &mut self.event_pump {
            let mouse_state = event_pump.mouse_state();
            let button_mask = match button.to_lowercase().as_str() {
                "left" => MouseButton::Left,
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => return Err(format!("Unsupported mouse button: {}", button)),
            };
            Ok(mouse_state.is_mouse_button_pressed(button_mask))
        } else {
            Err("Event pump not initialized".to_string())
        }
    }

    fn get_mouse_position(&mut self) -> Result<(i64, i64), String> {
        if let Some(event_pump) = &mut self.event_pump {
            let mouse_state = event_pump.mouse_state();
            Ok((mouse_state.x() as i64, mouse_state.y() as i64))
        } else {
            Err("Event pump not initialized".to_string())
        }
    }

    fn delay(&self, ms: u32) -> Result<(), String> {
        std::thread::sleep(Duration::from_millis(ms as u64));
        Ok(())
    }
}

// Rhai module to register SDL functions
pub fn register_sdl_module(engine: &mut Engine, sdl_context: Arc<Mutex<SDLContext>>) {
    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "create_window",
        move |title: &str, width: i64, height: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .create_window(title, width as i32, height as i32)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "set_draw_color",
        move |r: i64, g: i64, b: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .set_draw_color(r as u8, g as u8, b as u8)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn("clear", move || -> Result<(), Box<EvalAltResult>> {
        sdl_context_clone
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e.to_string()),
                    Default::default(),
                ))
            })?
            .clear()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e),
                    Default::default(),
                ))
            })
    });

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "draw_rect",
        move |x: i64, y: i64, w: i64, h: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .draw_rect(x as i32, y as i32, w as i32, h as i32)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "fill_rect",
        move |x: i64, y: i64, w: i64, h: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .fill_rect(x as i32, y as i32, w as i32, h as i32)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "draw_point",
        move |x: i64, y: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .draw_point(x as i32, y as i32)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "draw_line",
        move |x1: i64, y1: i64, x2: i64, y2: i64| -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .draw_line(x1 as i32, y1 as i32, x2 as i32, y2 as i32)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn("present", move || -> Result<(), Box<EvalAltResult>> {
        sdl_context_clone
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e.to_string()),
                    Default::default(),
                ))
            })?
            .present()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e),
                    Default::default(),
                ))
            })
    });

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "init_event_pump",
        move || -> Result<(), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .init_event_pump()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn("poll_event", move || -> Result<bool, Box<EvalAltResult>> {
        sdl_context_clone
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e.to_string()),
                    Default::default(),
                ))
            })?
            .poll_event()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e),
                    Default::default(),
                ))
            })
    });

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "is_key_down",
        move |key: &str| -> Result<bool, Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .is_key_down(key)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "is_mouse_button_down",
        move |button: &str| -> Result<bool, Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .is_mouse_button_down(button)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn(
        "get_mouse_position",
        move || -> Result<(i64, i64), Box<EvalAltResult>> {
            sdl_context_clone
                .lock()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Default::default(),
                    ))
                })?
                .get_mouse_position()
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e),
                        Default::default(),
                    ))
                })
        },
    );

    let sdl_context_clone = sdl_context.clone();
    engine.register_fn("delay", move |ms: i64| -> Result<(), Box<EvalAltResult>> {
        sdl_context_clone
            .lock()
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e.to_string()),
                    Default::default(),
                ))
            })?
            .delay(ms as u32)
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    Dynamic::from(e),
                    Default::default(),
                ))
            })
    });

    // Register random number generator
    engine.register_fn("rand", |min: i64, max: i64| -> i64 {
        let mut rng = rand::rng();
        rng.gen_range(min..=max)
    });
}
