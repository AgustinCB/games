use sdl2::video::{GLContext, GLProfile, Window as SdlWindow};
use sdl2::{EventPump, Sdl, TimerSubsystem};

use crate::MageError;

pub struct Window {
    last: u64,
    now: u64,
    sdl_context: Sdl,
    sdl_window: SdlWindow,
    timer: TimerSubsystem,
    _opengl: GLContext,
}

impl Window {
    pub fn new(name: &str, width: u32, height: u32) -> Result<Window, MageError> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let attrs = video_subsystem.gl_attr();

        attrs.set_context_major_version(4);
        attrs.set_context_minor_version(1);
        attrs.set_context_profile(GLProfile::Core);
        #[cfg(target_os = "macos")]
        attrs.set_context_flags().forward_compatible().set();

        let window = video_subsystem
            .window(name, width, height)
            .position_centered()
            .opengl()
            .build()?;
        let _opengl = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
        let sdl_timer = sdl_context.timer()?;
        Ok(Window {
            _opengl,
            sdl_context,
            last: 0,
            now: 0,
            sdl_window: window,
            timer: sdl_timer,
        })
    }

    pub fn swap_buffers(&self) {
        self.sdl_window.gl_swap_window();
    }

    pub fn get_pumper(&mut self) -> EventPump {
        self.sdl_context.event_pump().unwrap()
    }

    pub fn start_timer(&mut self) {
        self.now = self.timer.performance_counter() as _;
    }

    pub fn delta_time(&mut self) -> u64 {
        self.last = self.now;
        self.now = self.timer.performance_counter();
        (self.now - self.last) / 1000
    }
}
