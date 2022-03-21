use std::ffi::CString;
use std::mem::transmute;
use std::ptr;

use gl;
use thiserror::Error;

use crate::MageError;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Vertex = gl::VERTEX_SHADER,
}

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error creating shader: {0}")]
    CreationFailed(String),
}

fn check_success(
    resource: gl::types::GLuint,
    success_type: gl::types::GLenum,
) -> Result<(), MageError> {
    let mut status = gl::FALSE as gl::types::GLint;
    gl_function!(GetShaderiv(resource, success_type, &mut status));

    if status != (gl::TRUE as gl::types::GLint) {
        let mut len = 0;
        gl_function!(GetShaderiv(resource, gl::INFO_LOG_LENGTH, &mut len));
        let mut buf = [0].repeat(len as _);
        gl_function!(GetShaderInfoLog(
            resource,
            len,
            transmute(&mut len),
            transmute(buf.as_mut_ptr()),
        ));
        let s = std::str::from_utf8(&buf)
            .ok()
            .expect("ShaderInfoLog not valid utf8")
            .to_string();
        log::error!("{}", &s);
        Err(ShaderError::CreationFailed(s).into())
    } else {
        Ok(())
    }
}

pub struct Shader(pub(crate) gl::types::GLuint);

impl Shader {
    pub fn new(shader_type: ShaderType, content: &str) -> Result<Shader, MageError> {
        let shader = gl_function!(CreateShader(shader_type as _));
        let c_str = CString::new(content.as_bytes()).unwrap();
        gl_function!(ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null()));
        gl_function!(CompileShader(shader));

        check_success(shader, gl::COMPILE_STATUS)?;
        Ok(Shader(shader))
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl_function!(DeleteShader(self.0));
    }
}

impl ! Sync for Shader {}
