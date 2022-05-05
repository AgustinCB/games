use crate::rendering::opengl::shader::{Shader, ShaderType};
use crate::MageError;
use include_dir::Dir;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderLoaderError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("Empty File: {0}")]
    EmptyFile(String),
}

pub struct ShaderLoader {
    regex: Regex,
    shaders: &'static Dir<'static>,
}

const INCLUDE_STR: &str = "#include \"";

impl ShaderLoader {
    pub fn new(shaders: &'static Dir) -> Result<ShaderLoader, MageError> {
        let regex = Regex::new(r#"include "(.+)""#).map_err(Box::new)?;
        Ok(ShaderLoader { regex, shaders })
    }

    pub fn load(&self, shader_type: ShaderType, glsl: &'static str) -> Result<Shader, MageError> {
        Shader::new(shader_type, &self.load_file(glsl)?)
    }

    fn load_file(&self, glsl: &str) -> Result<String, MageError> {
        let mut content = self
            .shaders
            .get_file(glsl)
            .ok_or_else(|| Box::new(ShaderLoaderError::FileNotFound(glsl.to_owned())))
            .map(|f| f.contents_utf8())?
            .map(|s| s.to_string())
            .ok_or_else(|| Box::new(ShaderLoaderError::EmptyFile(glsl.to_owned())))?;
        for cap in self.regex.captures_iter(&(content.clone())) {
            content = content.replace(
                &(INCLUDE_STR.to_owned() + &cap[1] + r#"""#),
                &self.load_file(&cap[1])?,
            );
        }
        Ok(content)
    }
}
