use std::collections::HashMap;
use std::sync::Arc;

use image::EncodableLayout;
use image::io::Reader;

use crate::MageError;
use crate::rendering::model::mesh::{TextureInfo, TextureSource};
use crate::rendering::opengl::texture::{Texture, TextureDimension, TextureFormat};

pub struct TextureLoader {
    textures: HashMap<TextureSource, Arc<Texture>>,
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures: HashMap::new(),
        }
    }

    pub fn load_texture_cubemap(&mut self, texture_info: &TextureInfo) -> Arc<Texture> {
        if let Some(source) = self.textures.get(&texture_info.source) {
            source.clone()
        } else {
            todo!()
        }
    }

    pub fn load_texture_2d(
        &mut self,
        texture_info: &TextureInfo,
    ) -> Result<Arc<Texture>, MageError> {
        if let Some(source) = self.textures.get(&texture_info.source) {
            Ok(source.clone())
        } else {
            let texture = Arc::new(Texture::new(TextureDimension::Texture2D));
            texture.bind(texture_info.id as _);
            match &texture_info.source {
                TextureSource::File(path) => {
                    let image = Reader::open(path)?.decode()?.flipv();
                    match TextureFormat::try_from(image.color()) {
                        Ok(format) => {
                            texture.set_image_2d(
                                image.width() as u32,
                                image.height() as u32,
                                image.as_bytes(),
                                format,
                            );
                        }
                        Err(_) => {
                            let image = image.to_rgba8();
                            texture.set_image_2d(
                                image.width() as u32,
                                image.height() as u32,
                                image.as_bytes(),
                                TextureFormat::UnsignedByteWithAlpha,
                            );
                        }
                    };
                }
                TextureSource::Color(_color) => unimplemented!(),
            };
            self.textures
                .insert(texture_info.source.clone(), texture.clone());
            for (&k, &v) in &texture_info.parameters {
                texture.set_parameter(k, v);
            }
            texture.generate_mipmap();
            Ok(texture)
        }
    }
}
