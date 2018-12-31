extern crate gl;
extern crate sdl2;

pub use crate::app::sdl2::{
    pixels::Color,
    render::Canvas,
    video::Window,
};

pub struct Application2D;
pub struct Application3D;

/// An SDL2 application that uses the SDL2 Renderer for 2D drawing
impl Application2D {
    pub fn run(handle_events: fn(sdl2::event::Event), update: fn(), render: fn(canvas: &mut Canvas<Window>)) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("Game", 900, 700)
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.clear();

        let mut event_pump = sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    _ => {},
                }
                handle_events(event);
            }

            update();
            render(&mut canvas);
            canvas.present();
        }
    }
}

/// An SDL2 application that sets up an OpenGL context for 3D rendering
impl Application3D {
    pub fn run(handle_events: fn(sdl2::event::Event), update: fn(), render: fn(window: &mut Window)) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let mut window = video_subsystem
            .window("Game", 900, 700)
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

        let mut event_pump = sdl.event_pump().unwrap();
        'main: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} => break 'main,
                    _ => {},
                }
                handle_events(event);
            }

            update();
            render(&mut window);
            window.gl_swap_window();
        }
    }
}
