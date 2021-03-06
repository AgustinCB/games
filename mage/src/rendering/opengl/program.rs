use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem::transmute;
use std::ptr;

use gl;
use log::warn;
use nalgebra::{Matrix4, Vector3, Vector4};
use thiserror::Error;

use crate::rendering::opengl::shader::Shader;
use crate::MageError;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Error creating program: {0}")]
    CreationFailed(String),
}

fn check_success(
    resource: gl::types::GLuint,
    success_type: gl::types::GLenum,
) -> Result<(), MageError> {
    let mut status = gl::FALSE as gl::types::GLint;
    gl_function!(GetProgramiv(resource, success_type, &mut status));

    if status != (gl::TRUE as gl::types::GLint) {
        let mut len = 0;
        gl_function!(GetProgramiv(resource, gl::INFO_LOG_LENGTH, &mut len));
        let mut buf = [0].repeat(len as usize - 1);
        gl_function!(GetProgramInfoLog(
            resource,
            len,
            ptr::null_mut(),
            buf.as_mut_ptr() as *mut gl::types::GLchar,
        ));
        let s = std::str::from_utf8(&buf)
            .expect("ProgramInfoLog not valid utf8")
            .to_string();
        log::error!("{}", &s);
        Err(ProgramError::CreationFailed(s).into())
    } else {
        Ok(())
    }
}

pub struct Program {
    resource: gl::types::GLuint,
    uniforms: RefCell<HashMap<String, gl::types::GLint>>,
}

impl Program {
    pub fn new(vertex_shader: Shader, fragment_shader: Shader) -> Result<Program, MageError> {
        let resource = gl_function!(CreateProgram());
        gl_function!(AttachShader(resource, vertex_shader.0));
        gl_function!(AttachShader(resource, fragment_shader.0));
        gl_function!(LinkProgram(resource));
        check_success(resource, gl::LINK_STATUS)?;
        Ok(Program {
            resource,
            uniforms: RefCell::new(HashMap::new()),
        })
    }

    pub fn with_geometry(
        vertex_shader: Shader,
        fragment_shader: Shader,
        geometry_shader: Shader,
    ) -> Result<Program, MageError> {
        let resource = gl_function!(CreateProgram());
        gl_function!(AttachShader(resource, vertex_shader.0));
        gl_function!(AttachShader(resource, geometry_shader.0));
        gl_function!(AttachShader(resource, fragment_shader.0));
        gl_function!(LinkProgram(resource));
        check_success(resource, gl::LINK_STATUS)?;
        Ok(Program {
            resource,
            uniforms: RefCell::new(HashMap::new()),
        })
    }

    pub fn use_program(&self) {
        gl_function!(UseProgram(self.resource));
    }

    pub fn set_uniform_f1(&self, uniform: &str, x: f32) {
        let location = self.find_uniform(uniform);
        gl_function!(Uniform1f(location, x));
    }

    pub fn set_uniform_v4(&self, uniform: &str, vector: Vector4<f32>) {
        let location = self.find_uniform(uniform);
        gl_function!(Uniform4f(
            location,
            *vector.get(0).unwrap(),
            *vector.get(1).unwrap(),
            *vector.get(2).unwrap(),
            *vector.get(3).unwrap()
        ));
    }

    pub fn set_uniform_v3(&self, uniform: &str, vector: Vector3<f32>) {
        let location = self.find_uniform(uniform);
        gl_function!(Uniform3f(
            location,
            vector.data.0[0][0],
            vector.data.0[0][1],
            vector.data.0[0][2]
        ));
    }

    pub fn set_uniform_i1(&self, uniform: &str, value: i32) {
        let location = self.find_uniform(uniform);
        gl_function!(Uniform1i(location, value));
    }

    pub fn set_uniform_matrix4(&self, uniform: &str, matrix: Matrix4<f32>) {
        let location = self.find_uniform(uniform);
        gl_function!(UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr()));
    }

    pub fn bind_uniform_block(&self, uniform: &str, binding_point: usize) {
        let c_string = CString::new(uniform).unwrap();
        let block_index = gl_function!(GetUniformBlockIndex(
            self.resource,
            transmute(c_string.as_ptr())
        ));
        gl_function!(UniformBlockBinding(
            self.resource,
            block_index,
            binding_point as u32
        ));
    }

    fn find_uniform(&self, uniform: &str) -> gl::types::GLint {
        let mut cache = self.uniforms.borrow_mut();
        match cache.get(uniform) {
            Some(location) if *location == -1 => {
                warn!("Uniform {} does not exist", uniform);
                *location
            }
            Some(uniform) => *uniform,
            None => {
                let c_str = CString::new(uniform).unwrap();
                let location =
                    gl_function!(GetUniformLocation(self.resource, transmute(c_str.as_ptr())));
                if location == -1 {
                    warn!("Uniform {} does not exist", uniform);
                }
                cache.insert(uniform.to_string(), location);
                location
            }
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl_function!(DeleteProgram(self.resource));
    }
}
