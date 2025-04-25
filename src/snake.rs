use rhai::Engine;
use rhai::Scope;
use rhai::module_resolvers::FileModuleResolver;
use rhai_sdl3::{SDLContext, register_sdl_module};
use std::env;
use std::fs;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the SDL context
    let sdl_context = Arc::new(Mutex::new(SDLContext::new()?));

    // Create the Rhai engine
    let mut engine = Engine::new();

    // Set up the module resolver to look for scripts in the "scripts/" directory
    let resolver = FileModuleResolver::new_with_path("scripts/");
    engine.set_module_resolver(resolver);

    // Register SDL functions with the Rhai engine
    register_sdl_module(&mut engine, sdl_context.clone());

    // Determine the script name from command-line arguments or default to "main.rhai"
    // let script_name = env::args().nth(1).unwrap_or("main.rhai".to_string());
    // let script_path = format!("scripts/{}", script_name);

    // // Read the script from the file
    // let script = fs::read_to_string(&script_path)?;

    // // Evaluate the script
    // engine.eval::<()>(&script)?;

    // Create a scope (optional, for stateful execution)
    let mut scope = Scope::new();

    // Run the main script
    engine.run_file_with_scope(&mut scope, "scripts/snake.rhai".into())?;

    Ok(())
}
