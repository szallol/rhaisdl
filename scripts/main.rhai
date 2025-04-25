import "utils" as utils;

utils::greet("World");

create_window("Example", 800, 600);
init_event_pump();
while poll_event() {
    set_draw_color(255, 0, 0);
    clear();
    set_draw_color(0, 255, 0);
    draw_rect(100, 100, 200, 200);
    present();
}
