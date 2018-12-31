extern crate sdl2;
extern crate gl;

mod app;

use app::{Application2D, Application3D};

fn main() {
    app::Application2D::run(handle_events, update, render);
}

fn handle_events(_event: sdl2::event::Event) {
}

fn update() {
}

fn render() {
}
