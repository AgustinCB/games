use nalgebra::Vector4;

#[macro_export]
macro_rules! gl_function {
    ($a:ident($($b:tt)*)) => {
        unsafe {
            let return_value = gl::$a($($b)*);
            #[cfg(debug_assertions)]
            {
                log::trace!("gl{}({})", stringify!($a), stringify!($($b)*));
                let error_code = gl::GetError();
                if error_code != gl::NO_ERROR {
                    log::error!("ERROR CODE {} on gl{}({})", error_code, stringify!($a), stringify!($($b)*));
                    std::process::exit(error_code as i32);
                }
            }
            return_value
        }
    };
}

pub mod buffer;
pub mod frame_buffer;
pub mod multiple_render_target;
pub mod ping_pong_frame_buffer;
pub mod program;
pub mod render_buffer;
pub mod shader;
pub mod texture;
pub mod vertex_array;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum OpenGlType {
    UnsignedByte = gl::UNSIGNED_BYTE,
    UnsignedInt = gl::UNSIGNED_INT,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DrawingMode {
    Triangles = gl::TRIANGLES,
    TriangleStrip = gl::TRIANGLE_STRIP,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Feature {
    Depth = gl::DEPTH_TEST,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DrawingBuffer {
    Color = gl::COLOR_BUFFER_BIT,
    Depth = gl::DEPTH_BUFFER_BIT,
}

pub fn set_clear_color(color: Vector4<f32>) {
    gl_function!(ClearColor(
        *color.get(0).unwrap(),
        *color.get(1).unwrap(),
        *color.get(2).unwrap(),
        *color.get(3).unwrap()
    ));
}

pub fn clear(buffers: &[DrawingBuffer]) {
    let buffers = buffers.iter().fold(0, |z, b| z | *b as u32);
    gl_function!(Clear(buffers));
}

pub fn draw_arrays(mode: DrawingMode, vertices: u32) {
    gl_function!(DrawArrays(mode as _, 0, vertices as _,));
}

pub fn draw_elements(mode: DrawingMode, vertices: u32, indices_type: OpenGlType) {
    gl_function!(DrawElements(
        mode as _,
        vertices as _,
        indices_type as _,
        std::ptr::null()
    ));
}

pub fn enable(feature: Feature) {
    gl_function!(Enable(feature as _));
}
