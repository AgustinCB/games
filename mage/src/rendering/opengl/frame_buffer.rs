use log::error;

use crate::rendering::opengl::render_buffer::RenderBuffer;
use crate::rendering::opengl::texture::{
    Texture, TextureDimension, TextureFormat, TextureParameter, TextureParameterValue,
};

#[derive(Debug)]
pub struct FrameBuffer {
    _render_buffer: Option<RenderBuffer>,
    resource: gl::types::GLuint,
    pub texture: Texture,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> FrameBuffer {
        FrameBuffer::new_with_format(width, height, TextureFormat::UnsignedByte)
    }

    pub fn new_with_format(width: u32, height: u32, format: TextureFormat) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));

        let texture = Texture::new(TextureDimension::Texture2D);
        texture.just_bind();
        texture.allocate_space(width, height, format);
        texture.set_parameter(
            TextureParameter::TextureMinFilter,
            TextureParameterValue::Linear,
        );
        texture.set_parameter(
            TextureParameter::TextureMagFilter,
            TextureParameterValue::Linear,
        );
        texture.unbind();

        gl_function!(FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            TextureDimension::Texture2D as u32,
            texture.0,
            0
        ));

        let render_buffer = RenderBuffer::new();
        render_buffer.bind();
        gl_function!(RenderbufferStorage(
            gl::RENDERBUFFER,
            gl::DEPTH24_STENCIL8,
            width as _,
            height as _
        ));
        RenderBuffer::unbind();
        gl_function!(FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            render_buffer.0
        ));

        let status = gl_function!(CheckFramebufferStatus(gl::FRAMEBUFFER));
        if status != gl::FRAMEBUFFER_COMPLETE {
            error!("Error creating frame buffer, status code {}", status);
        }
        FrameBuffer::unbind();
        FrameBuffer {
            texture,
            _render_buffer: Some(render_buffer),
            resource: frame_buffer,
        }
    }

    pub fn intermediate_with_format(width: u32, height: u32, format: TextureFormat) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));

        let texture = Texture::new(TextureDimension::Texture2D);
        texture.just_bind();
        texture.allocate_space(width, height, format);
        texture.set_parameter(
            TextureParameter::TextureMinFilter,
            TextureParameterValue::Nearest,
        );
        texture.set_parameter(
            TextureParameter::TextureMagFilter,
            TextureParameterValue::Nearest,
        );
        gl_function!(FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            TextureDimension::Texture2D as u32,
            texture.0,
            0
        ));

        let status = gl_function!(CheckFramebufferStatus(gl::FRAMEBUFFER));
        if status != gl::FRAMEBUFFER_COMPLETE {
            error!("Error creating frame buffer, status code {}", status);
        }
        FrameBuffer::unbind();
        FrameBuffer {
            texture,
            _render_buffer: None,
            resource: frame_buffer,
        }
    }

    pub fn intermediate(width: u32, height: u32) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));

        let texture = Texture::new(TextureDimension::Texture2D);
        texture.just_bind();
        texture.allocate_space(width, height, TextureFormat::UnsignedByte);
        texture.set_parameter(
            TextureParameter::TextureMinFilter,
            TextureParameterValue::Linear,
        );
        texture.set_parameter(
            TextureParameter::TextureMagFilter,
            TextureParameterValue::Linear,
        );
        gl_function!(FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            TextureDimension::Texture2D as u32,
            texture.0,
            0
        ));

        let status = gl_function!(CheckFramebufferStatus(gl::FRAMEBUFFER));
        if status != gl::FRAMEBUFFER_COMPLETE {
            error!("Error creating frame buffer, status code {}", status);
        }
        FrameBuffer::unbind();
        FrameBuffer {
            texture,
            _render_buffer: None,
            resource: frame_buffer,
        }
    }

    pub fn multisample(width: u32, height: u32) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));

        let texture = Texture::new(TextureDimension::Texture2DMultisample);
        texture.just_bind();
        gl_function!(TexImage2DMultisample(
            TextureDimension::Texture2DMultisample as _,
            4,
            gl::RGB,
            width as i32,
            height as i32,
            gl::TRUE
        ));
        texture.unbind();

        gl_function!(FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            TextureDimension::Texture2DMultisample as u32,
            texture.0,
            0
        ));

        let render_buffer = RenderBuffer::new();
        render_buffer.bind();
        gl_function!(RenderbufferStorageMultisample(
            gl::RENDERBUFFER,
            4,
            gl::DEPTH24_STENCIL8,
            width as _,
            height as _
        ));
        RenderBuffer::unbind();
        gl_function!(FramebufferRenderbuffer(
            gl::FRAMEBUFFER,
            gl::DEPTH_STENCIL_ATTACHMENT,
            gl::RENDERBUFFER,
            render_buffer.0
        ));

        let status = gl_function!(CheckFramebufferStatus(gl::FRAMEBUFFER));
        if status != gl::FRAMEBUFFER_COMPLETE {
            error!("Error creating frame buffer, status code {}", status);
        }
        FrameBuffer::unbind();
        FrameBuffer {
            texture,
            _render_buffer: Some(render_buffer),
            resource: frame_buffer,
        }
    }

    pub fn depth_buffer(width: u32, height: u32) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));

        let texture = Texture::new(TextureDimension::Texture2D);
        texture.just_bind();
        texture.allocate_space(width, height, TextureFormat::Depth);
        texture.set_parameter(
            TextureParameter::TextureMinFilter,
            TextureParameterValue::Nearest,
        );
        texture.set_parameter(
            TextureParameter::TextureMagFilter,
            TextureParameterValue::Nearest,
        );
        texture.set_parameter(
            TextureParameter::TextureWrapS,
            TextureParameterValue::ClampToBorder,
        );
        texture.set_parameter(
            TextureParameter::TextureWrapT,
            TextureParameterValue::ClampToBorder,
        );
        gl_function!(TexParameterfv(
            gl::TEXTURE_2D,
            gl::TEXTURE_BORDER_COLOR,
            [1f32, 1f32, 1f32, 1f32].as_ptr()
        ));

        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));
        gl_function!(FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            texture.1 as _,
            texture.0,
            0
        ));
        gl_function!(DrawBuffer(gl::NONE));
        gl_function!(ReadBuffer(gl::NONE));
        FrameBuffer::unbind();

        FrameBuffer {
            texture,
            _render_buffer: None,
            resource: frame_buffer,
        }
    }

    pub fn depth_cubemap_with_texture(texture: Texture) -> FrameBuffer {
        let mut frame_buffer = 0u32;
        gl_function!(GenFramebuffers(1, &mut frame_buffer));

        texture.just_bind();
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, frame_buffer));
        gl_function!(FramebufferTexture(
            gl::FRAMEBUFFER,
            gl::DEPTH_ATTACHMENT,
            texture.0,
            0
        ));
        gl_function!(DrawBuffer(gl::NONE));
        gl_function!(ReadBuffer(gl::NONE));
        FrameBuffer::unbind();

        FrameBuffer {
            texture,
            _render_buffer: None,
            resource: frame_buffer,
        }
    }

    pub fn draw_bind(&self) {
        gl_function!(BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.resource));
    }

    pub fn read_bind(&self) {
        gl_function!(BindFramebuffer(gl::READ_FRAMEBUFFER, self.resource));
    }

    pub fn bind(&self) {
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, self.resource));
    }

    pub fn unbind() {
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, 0));
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        gl_function!(DeleteFramebuffers(1, &self.resource));
    }
}
