use sdl2::event::EventPollIterator;
use sdl2::EventPump;
use sdl2::video::{GLContext, GLProfile, Window as SdlWindow};

use crate::MageError;

pub struct Window {
    sdl_window: SdlWindow,
    event_pump: EventPump,
    _opengl: GLContext,
}

impl Window {
    pub fn new() -> Result<Window, MageError> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let attrs = video_subsystem.gl_attr();

        attrs.set_context_major_version(4);
        attrs.set_context_minor_version(1);
        attrs.set_context_profile(GLProfile::Core);
        #[cfg(target_os = "macos")]
            attrs.set_context_flags().forward_compatible().set();

        let window = video_subsystem
            .window("Opengl abstractions", 800, 600)
            .position_centered()
            .opengl()
            .build()?;
        let _opengl = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let event_pump = sdl_context.event_pump()?;
        Ok(Window {
            _opengl,
            event_pump,
            sdl_window: window,
        })
    }

    pub fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn poll_events(&mut self) -> EventPollIterator {
        self.event_pump.poll_iter()
    }
}
