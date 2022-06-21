use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use image::io::Reader;
use image::{DynamicImage, EncodableLayout, Pixel, Rgb, RgbImage, Rgba};

use crate::rendering::model::mesh::{TextureInfo, TextureSource};
use crate::rendering::opengl::texture::{Texture, TextureDimension, TextureFormat};
use crate::MageError;

fn load_image_to_texture(path: &str) -> Result<DynamicImage, MageError> {
    Ok(Reader::open(path)?.decode()?.flipv())
}

pub struct TextureLoader {
    textures: Mutex<HashMap<TextureSource, Arc<Texture>>>,
}

impl TextureLoader {
    pub fn new() -> TextureLoader {
        TextureLoader {
            textures: Mutex::new(HashMap::new()),
        }
    }

    pub fn load_texture_cubemap(&self, texture_info: &TextureInfo) -> Arc<Texture> {
        if let Some(source) = self.textures.lock().unwrap().get(&texture_info.source) {
            source.clone()
        } else {
            todo!()
        }
    }

    pub fn load_texture_2d(&self, texture_info: &TextureInfo) -> Result<Arc<Texture>, MageError> {
        let mut textures = self.textures.lock().unwrap();
        if let Some(source) = textures.get(&texture_info.source) {
            Ok(source.clone())
        } else {
            let texture = Arc::new(Texture::new(TextureDimension::Texture2D));
            texture.bind(texture_info.id as _);
            match &texture_info.source {
                TextureSource::File(path) => {
                    let image = load_image_to_texture(path)?;
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
                TextureSource::Color(color) => {
                    let mut image = RgbImage::new(1, 1);
                    image.put_pixel(0, 0, Rgb([color.x, color.y, color.z]));
                    texture.set_image_2d(
                        image.width() as u32,
                        image.height() as u32,
                        image.as_bytes(),
                        TextureFormat::UnsignedByte,
                    );
                }
                TextureSource::ColoredFile(path, color) => {
                    let mut image = load_image_to_texture(path)?.to_rgba8();
                    image.enumerate_pixels_mut().for_each(|(_, _, p)| {
                        #[allow(deprecated)]
                        let (r, g, b, a) = p.channels4();
                        *p = Rgba([r * color[0], g * color[1], b * color[2], a * color[3]]);
                    });
                    texture.set_image_2d(
                        image.width() as u32,
                        image.height() as u32,
                        image.as_bytes(),
                        TextureFormat::UnsignedByteWithAlpha,
                    );
                }
            };
            textures.insert(texture_info.source.clone(), texture.clone());
            for (&k, &v) in &texture_info.parameters {
                texture.set_parameter(k, v);
            }
            texture.generate_mipmap();
            Ok(texture)
        }
    }
}

impl Default for TextureLoader {
    fn default() -> TextureLoader {
        TextureLoader::new()
    }
}
