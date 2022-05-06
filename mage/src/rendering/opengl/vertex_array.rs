use std::mem::{size_of, transmute};
use std::ptr;

use gl;
use itertools::Itertools;

use crate::gl_function;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum DataType {
    Byte = gl::BYTE,
    UnsignedByte = gl::UNSIGNED_BYTE,
    Short = gl::SHORT,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Int = gl::INT,
    UnsignedInt = gl::UNSIGNED_INT,
    HalfFloat = gl::HALF_FLOAT,
    Float = gl::FLOAT,
    Double = gl::DOUBLE,
    Fixed = gl::FIXED,
    Int2101010Rev = gl::INT_2_10_10_10_REV,
    UnsignedInt2101010Rev = gl::UNSIGNED_INT_2_10_10_10_REV,
    UnsignedInt10F11F11FRev = gl::UNSIGNED_INT_10F_11F_11F_REV,
}

#[derive(Debug)]
pub struct VertexArray(gl::types::GLuint);

impl VertexArray {
    #[allow(clippy::new_without_default)]
    pub fn new() -> VertexArray {
        let mut vertex_array = 0u32;
        gl_function!(GenVertexArrays(1, &mut vertex_array));
        VertexArray(vertex_array)
    }

    pub fn multiple<const S: usize>() -> Vec<VertexArray> {
        let mut vertex_arrays = [0; S];
        gl_function!(GenVertexArrays(S as i32, vertex_arrays.as_mut_ptr()));
        vertex_arrays.into_iter().map(VertexArray).collect_vec()
    }

    pub fn bind(&self) {
        gl_function!(BindVertexArray(self.0));
    }

    pub fn set_vertex_attrib<T>(gl_type: DataType, attribute: u32, size: u32, normalized: bool) {
        let normalized = if normalized { gl::TRUE } else { gl::FALSE };
        gl_function!(EnableVertexAttribArray(attribute));
        gl_function!(VertexAttribPointer(
            attribute,
            size as _,
            gl_type as _,
            normalized,
            size as i32 * size_of::<T>() as i32,
            ptr::null()
        ));
    }

    pub fn set_vertex_attrib_with_padding<T>(
        gl_type: DataType,
        attribute: u32,
        size: u32,
        padding: u32,
        start: u32,
        normalized: bool,
    ) {
        let normalized = if normalized { gl::TRUE } else { gl::FALSE };
        gl_function!(EnableVertexAttribArray(attribute));
        gl_function!(VertexAttribPointer(
            attribute,
            padding as _,
            gl_type as _,
            normalized,
            size as i32 * size_of::<T>() as i32,
            transmute(start as usize * size_of::<T>())
        ));
    }

    pub fn unbind() {
        gl_function!(BindVertexArray(0));
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        gl_function!(DeleteVertexArrays(1, &self.0))
    }
}
