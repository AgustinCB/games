use std::mem::transmute;
use std::ptr;

use gl;
use itertools::Itertools;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureType {
    Texture1D = gl::TEXTURE_1D,
    Texture2D = gl::TEXTURE_2D,
    Texture3D = gl::TEXTURE_3D,
    CubeMap = gl::TEXTURE_CUBE_MAP,
    Texture2DMultisample = gl::TEXTURE_2D_MULTISAMPLE,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureParameter {
    DepthStencilTextureMode = gl::DEPTH_STENCIL_TEXTURE_MODE,
    TextureCompareFunc = gl::TEXTURE_COMPARE_FUNC,
    TextureCompareMode = gl::TEXTURE_COMPARE_MODE,
    TextureMinFilter = gl::TEXTURE_MIN_FILTER,
    TextureMagFilter = gl::TEXTURE_MAG_FILTER,
    TextureWrapS = gl::TEXTURE_WRAP_S,
    TextureWrapT = gl::TEXTURE_WRAP_T,
    TextureWrapR = gl::TEXTURE_WRAP_R,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum TextureParameterValue {
    DepthComponent = gl::DEPTH_COMPONENT,
    StencilIndex = gl::STENCIL_INDEX,
    LessEqual = gl::LEQUAL,
    GreaterEqual = gl::GEQUAL,
    Less = gl::LESS,
    Greater = gl::GREATER,
    Equal = gl::EQUAL,
    NotEqual = gl::NOTEQUAL,
    Always = gl::ALWAYS,
    Never = gl::NEVER,
    CompareRefToTexture = gl::COMPARE_REF_TO_TEXTURE,
    None = gl::NONE,
    Nearest = gl::NEAREST,
    Linear = gl::LINEAR,
    NearestMipmapNearest = gl::NEAREST_MIPMAP_NEAREST,
    LinearMipmapNearest = gl::LINEAR_MIPMAP_NEAREST,
    NearestMipmapLinear = gl::NEAREST_MIPMAP_LINEAR,
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    ClampToEdge = gl::CLAMP_TO_EDGE,
    ClampToBorder = gl::CLAMP_TO_BORDER,
    MirroredRepeat = gl::MIRRORED_REPEAT,
    Repeat = gl::REPEAT,
    MirrorClampToEdge = gl::MIRROR_CLAMP_TO_EDGE,
}

#[derive(Clone, Copy)]
pub enum TextureFormat {
    FloatingPoint,
    UnsignedByte,
    UnsignedByteWithAlpha,
    Grey,
    Depth,
}

#[derive(Debug)]
pub struct Texture(pub(crate) gl::types::GLuint, pub(crate) TextureType);

impl Texture {
    pub fn multiple(texture_types: Vec<TextureType>) -> Vec<Texture> {
        let mut texture_resources = [0].repeat(texture_types.len());
        gl_function!(GenTextures(texture_types.len() as i32, texture_resources.as_mut_ptr()));
        texture_resources.into_iter()
            .zip(texture_types)
            .map(|(r, t)| Texture(r, t))
            .collect_vec()
    }

    pub fn new(texture_type: TextureType) -> Texture {
        let mut texture = 0 as gl::types::GLuint;
        gl_function!(GenTextures(1, &mut texture));
        Texture(texture, texture_type)
    }

    pub fn unbind(&self) {
        gl_function!(BindTexture(self.1 as _, 0));
    }

    pub fn bind_as(&self, unit: gl::types::GLenum, texture_type: gl::types::GLenum) {
        gl_function!(ActiveTexture(unit));
        gl_function!(BindTexture(texture_type, self.0));
    }

    pub fn bind(&self, unit: gl::types::GLenum) {
        gl_function!(ActiveTexture(unit));
        self.just_bind();
    }

    pub fn just_bind(&self) {
        gl_function!(BindTexture(self.1 as _, self.0));
    }

    pub fn generate_mipmap(&self) {
        gl_function!(GenerateMipmap(self.1 as _));
    }

    pub fn alloc_depth_cube_map_face(&self, face: u32, width: usize, height: usize) {
        gl_function!(TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + face,
            0,
            gl::DEPTH_COMPONENT as _,
            width as _,
            height as _,
            0,
            gl::DEPTH_COMPONENT as _,
            gl::FLOAT,
            ptr::null(),
        ));
    }

    pub fn set_cube_map_face(&self, face: u32, width: usize, height: usize, data: &[u8]) {
        gl_function!(TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + face,
            0,
            gl::RGBA as _,
            width as _,
            height as _,
            0,
            gl::RGBA as _,
            gl::UNSIGNED_BYTE,
            transmute(&(data[0]) as *const u8)
        ));
    }

    pub fn set_image_2d<T>(&self, width: u32, height: u32, data: &[T], format: TextureFormat) {
        match (self.1, format) {
            (TextureType::Texture2D, TextureFormat::UnsignedByte) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGB as _,
                width as _,
                height as _,
                0,
                gl::RGB as _,
                gl::UNSIGNED_BYTE,
                transmute(&(data[0]) as *const T)
            )),
            (TextureType::Texture2D, TextureFormat::UnsignedByteWithAlpha) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGBA as _,
                width as _,
                height as _,
                0,
                gl::RGBA as _,
                gl::UNSIGNED_BYTE,
                transmute(&(data[0]) as *const T)
            )),
            (TextureType::Texture2D, TextureFormat::FloatingPoint) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGBA16F as _,
                width as _,
                height as _,
                0,
                gl::RGBA as _,
                gl::FLOAT,
                transmute(&(data[0]) as *const T)
            )),
            _ => unimplemented!(),
        }
    }

    pub fn allocate_space(&self, width: u32, height: u32, format: TextureFormat) {
        match (self.1, format) {
            (TextureType::Texture2D, TextureFormat::UnsignedByte) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGB as _,
                width as _,
                height as _,
                0,
                gl::RGB as _,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            )),
            (TextureType::Texture2D, TextureFormat::UnsignedByteWithAlpha) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGBA as _,
                width as _,
                height as _,
                0,
                gl::RGBA as _,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            )),
            (TextureType::Texture2D, TextureFormat::FloatingPoint) => gl_function!(TexImage2D(
                self.1 as _,
                0,
                gl::RGBA16F as _,
                width as _,
                height as _,
                0,
                gl::RGBA as _,
                gl::FLOAT,
                ptr::null(),
            )),
            (TextureType::Texture2D, TextureFormat::Grey) => gl_function!(TexImage2D(
                self.1 as _, 0, gl::RED as _, width as _, height as _, 0, gl::RED as _, gl::FLOAT, ptr::null(),
            )),
            (TextureType::Texture2D, TextureFormat::Depth) => gl_function!(TexImage2D(
                self.1 as _, 0, gl::DEPTH_COMPONENT as _, width as _, height as _, 0, gl::DEPTH_COMPONENT as _, gl::FLOAT, ptr::null(),
            )),
            _ => unimplemented!(),
        }
    }

    pub fn set_parameter(&self, parameter: TextureParameter, value: TextureParameterValue) {
        gl_function!(TexParameteri(self.1 as _, parameter as _, value as _));
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        gl_function!(DeleteTextures(1, &self.0));
    }
}