extern crate sdl2;
extern crate gl;
extern crate core;

use core::{
    app::Application2D
};

use sdl2::{
    render::Canvas,
    video::Window,
};

fn main() {
   Application2D::run(handle_events, update, render);
}

fn handle_events(_event: sdl2::event::Event) {
}

fn update() {
}

fn render(canvas: &mut Canvas<Window>) {
    canvas.clear();
    canvas.present();
}
