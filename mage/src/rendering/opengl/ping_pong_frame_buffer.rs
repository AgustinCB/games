use crate::rendering::opengl::texture::{
    Texture, TextureDimension, TextureFormat, TextureParameter, TextureParameterValue,
};

#[derive(Debug)]
pub struct PingPongFrameBuffer {
    ping_buffer: gl::types::GLuint,
    pong_buffer: gl::types::GLuint,
    ping_texture: Texture,
    pong_texture: Texture,
}

impl PingPongFrameBuffer {
    pub fn new(width: usize, height: usize) -> PingPongFrameBuffer {
        PingPongFrameBuffer::new_with_format(width, height, TextureFormat::UnsignedByte)
    }

    pub fn new_with_format(
        width: usize,
        height: usize,
        format: TextureFormat,
    ) -> PingPongFrameBuffer {
        let mut ping_pong_fbs = vec![0, 0];
        let mut textures = vec![];
        gl_function!(GenFramebuffers(2, ping_pong_fbs.as_mut_ptr()));

        for &fb in ping_pong_fbs.iter().take(2) {
            let texture = Texture::new(TextureDimension::Texture2D);
            gl_function!(BindFramebuffer(gl::FRAMEBUFFER, fb));
            texture.just_bind();
            texture.allocate_space(width as u32, height as u32, format);
            texture.set_parameter(
                TextureParameter::TextureMinFilter,
                TextureParameterValue::Linear,
            );
            texture.set_parameter(
                TextureParameter::TextureMagFilter,
                TextureParameterValue::Linear,
            );
            texture.set_parameter(
                TextureParameter::TextureWrapS,
                TextureParameterValue::ClampToBorder,
            );
            texture.set_parameter(
                TextureParameter::TextureWrapT,
                TextureParameterValue::ClampToBorder,
            );
            gl_function!(FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                TextureDimension::Texture2D as _,
                texture.0,
                0
            ));
            textures.push(texture)
        }
        let mut textures = textures.into_iter();
        PingPongFrameBuffer {
            ping_buffer: ping_pong_fbs[0],
            pong_buffer: ping_pong_fbs[1],
            ping_texture: textures.next().unwrap(),
            pong_texture: textures.next().unwrap(),
        }
    }

    pub fn bind(&self, pong: bool, texture_index: u32) {
        let fb = if pong {
            self.pong_buffer
        } else {
            self.ping_buffer
        };
        let texture = if !pong {
            &self.pong_texture
        } else {
            &self.ping_texture
        };
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, fb));
        texture.bind(gl::TEXTURE0 + texture_index);
    }

    pub fn bind_texture(&self, ping: bool, texture_index: u32) {
        let texture = if ping {
            &self.ping_texture
        } else {
            &self.pong_texture
        };
        texture.bind(gl::TEXTURE0 + texture_index);
    }

    pub fn unbind() {
        gl_function!(BindFramebuffer(gl::FRAMEBUFFER, 0));
    }
}

impl Drop for PingPongFrameBuffer {
    fn drop(&mut self) {
        let fbs = vec![self.ping_buffer, self.pong_buffer];
        gl_function!(DeleteFramebuffers(2, fbs.as_ptr()));
    }
}
