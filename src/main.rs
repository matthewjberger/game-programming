extern crate sdl2;
extern crate gl;

mod app;
use app::{Application2D, Application3D};

use sdl2::{
    render::Canvas,
    video::Window,
};

fn main() {
    app::Application2D::run(handle_events, update, render2D);
    //app::Application3D::run(handle_events, update, render3D);
}

fn handle_events(_event: sdl2::event::Event) {
}

fn update() {
}

fn render2D(canvas: &mut Canvas<Window>) {
}

fn render3D(window: &mut Window) {
}
