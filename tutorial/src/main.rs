use macroquad::prelude::*;

#[macroquad::main("Tutorial Game")]
async fn main() {
    println!("Hello, world!");

    let mut x = screen_width() / 2.0;
    let mut y = screen_height() /2.0;

    let velocity = 1.0;

    loop {
        clear_background(SKYBLUE);

        if is_key_down(KeyCode::Down) {
            y+=velocity;
        }

        if is_key_down(KeyCode::Up) {
            y-=velocity;
        }

        if is_key_down(KeyCode::Right) {
            x+=velocity
        }

        if is_key_down(KeyCode::Left) {
            x-=velocity;
        }

        draw_circle(x, y, 16.0, YELLOW);


        next_frame().await;
    }
}
