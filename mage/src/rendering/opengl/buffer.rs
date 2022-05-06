use std::mem::{size_of, transmute};
use std::ptr;

use gl::types::{GLenum, GLuint};
use itertools::Itertools;

#[derive(Debug)]
pub struct Buffer(GLuint, GLenum);

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum BufferUsage {
    StreamDraw = gl::STREAM_DRAW,
    StreamRead = gl::STREAM_READ,
    StreamCopy = gl::STREAM_COPY,
    StaticDraw = gl::STATIC_DRAW,
    StaticRead = gl::STATIC_READ,
    StaticCopy = gl::STATIC_COPY,
    DynamicDraw = gl::DYNAMIC_DRAW,
    DynamicRead = gl::DYNAMIC_READ,
    DynamicCopy = gl::DYNAMIC_COPY,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum BufferType {
    Array = gl::ARRAY_BUFFER,
    AtomicCounter = gl::ATOMIC_COUNTER_BUFFER,
    CopyRead = gl::COPY_READ_BUFFER,
    CopyWrite = gl::COPY_WRITE_BUFFER,
    DispatchIndirect = gl::DISPATCH_INDIRECT_BUFFER,
    DrawIndirect = gl::DRAW_INDIRECT_BUFFER,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER,
    PixelPack = gl::PIXEL_PACK_BUFFER,
    PixelUnpack = gl::PIXEL_UNPACK_BUFFER,
    Query = gl::QUERY_BUFFER,
    ShaderStorage = gl::SHADER_STORAGE_BUFFER,
    Texture = gl::TEXTURE_BUFFER,
    TransformFeedback = gl::TRANSFORM_FEEDBACK_BUFFER,
    Uniform = gl::UNIFORM_BUFFER,
}

impl Buffer {
    pub fn new(buffer_type: BufferType) -> Buffer {
        let mut buffer = 0u32;
        gl_function!(GenBuffers(1, &mut buffer));
        Buffer(buffer, buffer_type as _)
    }

    pub fn multiple(buffer_types: Vec<BufferType>) -> Vec<Buffer> {
        let mut buffers = (0..buffer_types.len())
            .into_iter()
            .map(|_| 0u32)
            .collect_vec();
        gl_function!(GenBuffers(buffer_types.len() as i32, buffers.as_mut_ptr()));
        buffers
            .into_iter()
            .zip(&buffer_types)
            .map(|(b, t)| Buffer(b, *t as _))
            .collect_vec()
    }

    pub fn allocate_data<T>(&self, size: usize) {
        gl_function!(BufferData(
            self.1,
            (size_of::<T>() * size) as isize,
            ptr::null(),
            gl::STATIC_DRAW
        ))
    }

    pub fn set_sub_data<T>(&self, from: usize, to: usize, data: &[T]) {
        gl_function!(BufferSubData(
            self.1,
            (size_of::<T>() * from) as isize,
            (size_of::<T>() * to) as isize,
            transmute(&data[0]),
        ));
    }

    pub fn set_data<T>(&self, data: &[T], drawing_mode: BufferUsage) {
        gl_function!(BufferData(
            self.1,
            (size_of::<T>() * data.len()) as isize,
            transmute(&data[0]),
            drawing_mode as u32
        ));
    }

    pub fn link_to_binding_point(&self, binding_point: usize, from: usize, to: usize) {
        gl_function!(BindBufferRange(
            self.1,
            binding_point as _,
            self.0,
            from as _,
            to as _
        ));
    }

    pub fn bind(&self) {
        gl_function!(BindBuffer(self.1, self.0));
    }

    pub fn unbind(&self) {
        gl_function!(BindBuffer(self.1, 0));
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        gl_function!(DeleteBuffers(1, &self.0));
    }
}
