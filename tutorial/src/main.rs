use macroquad::prelude::*;

#[macroquad::main("Tutorial Game")]
async fn main() {
    println!("Hello, world!");
    loop {
        clear_background(SKYBLUE);
        next_frame().await
    }
}
