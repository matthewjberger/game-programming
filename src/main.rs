extern crate sdl2;
extern crate gl;

mod core;
use core::Application;

fn main() {
    core::Application::run(handle_events, update, render);
}

fn handle_events(_event: sdl2::event::Event) {
}

fn update() {
}

fn render() {
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
