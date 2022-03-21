use itertools::Itertools;

#[derive(Debug)]
pub struct RenderBuffer(pub(crate) gl::types::GLuint);

impl RenderBuffer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RenderBuffer {
        let mut buffer = 0u32;
        gl_function!(GenRenderbuffers(1, &mut buffer));
        RenderBuffer(buffer)
    }

    pub fn multiple<const S: usize>() -> Vec<RenderBuffer> {
        let mut render_buffers = [0; S];
        gl_function!(GenRenderbuffers(S as i32, render_buffers.as_mut_ptr()));
        render_buffers
            .into_iter()
            .map(RenderBuffer)
            .collect_vec()
    }

    pub fn bind(&self) {
        gl_function!(BindRenderbuffer(gl::RENDERBUFFER, self.0));
    }

    pub fn unbind() {
        gl_function!(BindRenderbuffer(gl::RENDERBUFFER, 0));
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        gl_function!(DeleteRenderbuffers(1, &self.0));
    }
}

impl ! Sync for RenderBuffer {}
