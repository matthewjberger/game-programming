extern crate core;
extern crate gl;
extern crate sdl2;

use sdl2::rect::Rect;
use core::app::{Application2D, Canvas, Color, Window};

fn main() {
    Application2D::run(handle_events, update, render);
}

fn handle_events(_event: sdl2::event::Event) {}

fn update() {}

fn render(canvas: &mut Canvas<Window>) {
    let paddle_1 = Rect::new(0, 0, 1024, 15);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.fill_rect(paddle_1).unwrap();
}
