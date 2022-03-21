use itertools::Itertools;
use log::error;

use crate::rendering::opengl::render_buffer::RenderBuffer;
use crate::rendering::opengl::texture::{Texture, TextureDimension, TextureFormat, TextureParameter, TextureParameterValue};

fn textures_with_formats(width: u32, height: u32, formats: &[TextureFormat]) -> Vec<Texture> {
    let textures = Texture::multiple([TextureDimension::Texture2D].repeat(formats.len()));
    for (i, (format, texture)) in formats.iter().zip(&textures).enumerate() {
        texture.just_bind();
        texture.allocate_space(width, height, *format);
        texture.set_parameter(TextureParameter::TextureMinFilter, TextureParameterValue::Nearest);
        texture.set_parameter(TextureParameter::TextureMagFilter, TextureParameterValue::Nearest);
        texture.set_parameter(TextureParameter::TextureWrapS, TextureParameterValue::ClampToBorder);
        texture.set_parameter(TextureParameter::TextureWrapT, TextureParameterValue::ClampToBorder);
        texture.unbind();
        gl_function!(FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0 + i as u32, TextureDimension::Texture2D as u32, texture.0, 0));
    }
    textures
}

#[derive(Debug)]
pub struct MultipleRenderTarget {
    _render_buffer: Option<RenderBuffer>,
    pub resource: gl::types::GLuint,
    pub textures: Vec<Texture>,
}

impl MultipleRenderTarget {
    pub fn new(width: u32, height: u32, targets: usize) -> MultipleRenderTarget {
        MultipleRenderTarget::new_with_format(width, height, targets, TextureFormat::UnsignedByte)
    }

    pub fn new_with_format(width: u32, height: u32, targets: usize, format: TextureFormat) -> MultipleRenderTarget {
        MultipleRenderTarget::new_with_formats(
            width, height, &std::iter::repeat(format).take(targets).collect_vec(),
        )
    }

    pub fn new_with_formats(width: u32, height: u32, formats: &[TextureFormat]) -> MultipleRenderTarget {
        let mut frame_buffer = 0 as gl::types::GLuint;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));

        let textures = textures_with_formats(width, height, formats);

        let render_buffer = RenderBuffer::new();
        render_buffer.bind();
        gl_function!(RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, width as _, height as _));
        RenderBuffer::unbind();
        gl_function!(FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, render_buffer.0));

        let status = gl_function!(CheckFramebufferStatus(gl::FRAMEBUFFER));
        if status != gl::FRAMEBUFFER_COMPLETE {
            error!("Error creating frame buffer, status code {}", status);
        }
        MultipleRenderTarget::unbind();
        MultipleRenderTarget {
            textures,
            _render_buffer: Some(render_buffer),
            resource: frame_buffer,
        }
    }

    pub fn set_draw_buffers(&self) {
        let attachments = (0..self.textures.len()).into_iter()
            .map(|i| gl::COLOR_ATTACHMENT0 + i as u32)
            .collect_vec();
        gl_function!(DrawBuffers(self.textures.len() as _, attachments.as_ptr()));
    }

    pub fn bind(&self) {
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, self.resource));
    }

    pub fn unbind() {
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, 0));
    }
}

impl Drop for MultipleRenderTarget {
    fn drop(&mut self) {
        gl_function!(DeleteFramebuffers(1, &self.resource));
    }
}

impl ! Sync for MultipleRenderTarget {}