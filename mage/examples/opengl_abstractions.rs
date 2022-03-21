use std::collections::HashMap;
use std::sync::Arc;

use nalgebra::Vector4;
use russimp::texture::TextureType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use mage::MageError;
use mage::rendering::model::mesh::{TextureInfo, TextureSource};
use mage::rendering::model::quad::quad;
use mage::rendering::opengl::{clear, draw, DrawingBuffer, DrawingMode, OpenGlType, set_clear_color};
use mage::rendering::opengl::buffer::{Buffer, BufferType, BufferUsage};
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::{Shader, ShaderType};
use mage::rendering::opengl::texture::{Texture, TextureParameter, TextureParameterValue};
use mage::rendering::opengl::vertex_array::{DataType, VertexArray};
use mage::resources::texture::TextureLoader;

const VERTEX_SHADER: &'static str = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

void main()
{
    vec3 position = aPos * 0.5;
    gl_Position = vec4(position, 1.0);
    ourColor = vec3(aTexCoord, 1.0);
    TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}";

const FRAGMENT_SHADER: &'static str = "#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;

void main()
{
    FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1);
}";

fn load_texture(texture_info: TextureInfo) -> Result<Arc<Texture>, MageError> {
    let mut loader = TextureLoader::new();
    loader.load_texture_2d(&texture_info)
}

pub fn main() {
    env_logger::init();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let attrs = video_subsystem.gl_attr();

    attrs.set_context_major_version(4);
    attrs.set_context_minor_version(1);
    attrs.set_context_profile(GLProfile::Core);
    #[cfg(target_os = "macos")]
        attrs.set_context_flags().forward_compatible().set();

    let window = video_subsystem.window("Opengl abstractions", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let _opengl = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl_context.event_pump().unwrap();

    let program = Program::new(
        Shader::new(ShaderType::Vertex, VERTEX_SHADER).unwrap(),
        Shader::new(ShaderType::Fragment, FRAGMENT_SHADER).unwrap(),
    ).unwrap();
    let quad = quad(vec![]);
    let vertex_array = VertexArray::new();
    let array_buffer = Buffer::new(BufferType::Array);
    let element_buffer = Buffer::new(BufferType::ElementArray);
    vertex_array.bind();
    array_buffer.bind();
    array_buffer.set_data(&quad.flattened_data(), BufferUsage::StaticDraw);
    element_buffer.bind();
    element_buffer.set_data(&quad.indices.clone().unwrap(), BufferUsage::StaticDraw);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 0, 8, 3, 0, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 1, 8, 3, 3, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 2, 8, 2, 6, false);

    let texture = load_texture(TextureInfo {
        id: 0,
        texture_type: TextureType::Diffuse,
        source: TextureSource::File(format!("{}/examples/resources/container.jpg", env!("CARGO_MANIFEST_DIR"))),
        parameters: HashMap::from([
            (TextureParameter::TextureWrapS, TextureParameterValue::Repeat),
            (TextureParameter::TextureWrapT, TextureParameterValue::Repeat),
            (TextureParameter::TextureMinFilter, TextureParameterValue::LinearMipmapLinear),
            (TextureParameter::TextureMagFilter, TextureParameterValue::Linear),
        ]),
    }).unwrap();

    set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'game_loop,
                _ => {}
            }
        }
        clear(&vec![DrawingBuffer::Color]);
        texture.bind(0);
        program.use_program();
        vertex_array.bind();
        draw(DrawingMode::Triangles, 6, OpenGlType::UnsignedInt);
        window.gl_swap_window();
    }
}