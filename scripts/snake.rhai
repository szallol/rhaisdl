// Initialize SDL window
create_window("Snake Game", 800, 600);
init_event_pump();

// Game constants
let GRID_SIZE = 20; // Size of each grid cell in pixels
let GRID_WIDTH = 40; // Number of cells in width (800 / 20)
let GRID_HEIGHT = 30; // Number of cells in height (600 / 20)

// Snake properties
let snake = [[5, 5]]; // Initial snake position as list of [x, y] segments
let direction = "right"; // Initial direction
let food = [10, 10]; // Initial food position
let score = 0;
let game_over = false;

// Main game loop
while poll_event() && !game_over {
    // Handle input
    if is_key_down("up") && direction != "down" {
        direction = "up";
    } else if is_key_down("down") && direction != "up" {
        direction = "down";
    } else if is_key_down("left") && direction != "right" {
        direction = "left";
    } else if is_key_down("right") && direction != "left" {
        direction = "right";
    }

    // Update snake position
    let head = snake[0]; // Get head position
    let new_head = if direction == "up" {
        [head[0], head[1] - 1]
    } else if direction == "down" {
        [head[0], head[1] + 1]
    } else if direction == "left" {
        [head[0] - 1, head[1]]
    } else if direction == "right" {
        [head[0] + 1, head[1]]
    } else {
        head // Fallback, should not happen
    };

    // Check for collisions with boundaries
    if new_head[0] < 0 || new_head[0] >= GRID_WIDTH || new_head[1] < 0 || new_head[1] >= GRID_HEIGHT {
        game_over = true;
        continue;
    }

    // Check for collisions with self
    for segment in snake {
        if new_head[0] == segment[0] && new_head[1] == segment[1] {
            game_over = true;
            break;
        }
    }

    // Move snake
    snake.insert(0, new_head); // Add new head at index 0
    if new_head[0] == food[0] && new_head[1] == food[1] {
        // Eat food, increase score, generate new food
        score += 1;
        food = [rand(0, GRID_WIDTH - 1), rand(0, GRID_HEIGHT - 1)];
    } else {
        // Remove tail if not eating
        snake.pop();
    }

    // Clear screen (black)
    set_draw_color(0, 0, 0);
    clear();

    // Draw snake (green)
    set_draw_color(0, 255, 0);
    for segment in snake {
        fill_rect(segment[0] * GRID_SIZE, segment[1] * GRID_SIZE, GRID_SIZE, GRID_SIZE);
    }

    // Draw food (red)
    set_draw_color(255, 0, 0);
    fill_rect(food[0] * GRID_SIZE, food[1] * GRID_SIZE, GRID_SIZE, GRID_SIZE);

    // Update screen
    present();

    // Control game speed
    delay(100); // 100ms delay for ~10 FPS
}

// Game over screen
set_draw_color(0, 0, 0);
clear();
set_draw_color(255, 255, 255);
// Note: SDL3 text rendering not implemented in provided bindings, so we skip displaying "Game Over"
present();
