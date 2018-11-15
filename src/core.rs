extern crate gl;

pub struct Application;

impl Application {
    pub fn run(handle_events: fn(sdl2::event::Event), update: fn(), render: fn()) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
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
            render();
            window.gl_swap_window();
        }
    }
}
