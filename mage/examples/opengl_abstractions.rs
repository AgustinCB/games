use nalgebra::Vector4;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use mage::rendering::opengl::{clear, draw, DrawingBuffer, DrawingMode, OpenGlType, set_clear_color};
use mage::rendering::opengl::buffer::{Buffer, BufferType, BufferUsage};
use mage::rendering::opengl::program::Program;
use mage::rendering::opengl::shader::{Shader, ShaderType};
use mage::rendering::opengl::texture::{Texture, TextureFormat, TextureParameter, TextureParameterValue, TextureType};
use mage::rendering::opengl::vertex_array::{DataType, VertexArray};

const VERTEX_SHADER: &'static str = "#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

void main()
{
    gl_Position = vec4(aPos, 1.0);
    ourColor = aColor;
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
const VERTICES: [f32; 32] = [
    0.5f32, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -0.5, -0.5,
    0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
];
const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

pub fn main() {
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
    let vertex_array = VertexArray::new();
    let array_buffer = Buffer::new(BufferType::Array);
    let element_buffer = Buffer::new(BufferType::ElementArray);
    vertex_array.bind();
    array_buffer.bind();
    array_buffer.set_data(&VERTICES, BufferUsage::StaticDraw);
    element_buffer.bind();
    element_buffer.set_data(&INDICES, BufferUsage::StaticDraw);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 0, 8, 3, 0, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 1, 8, 3, 3, false);
    VertexArray::set_vertex_attrib_with_padding::<f32>(DataType::Float, 2, 8, 2, 6, false);

    let texture = Texture::new(TextureType::Texture2D);
    texture.bind(0);
    texture.set_parameter(TextureParameter::TextureWrapS, TextureParameterValue::Repeat);
    texture.set_parameter(TextureParameter::TextureWrapT, TextureParameterValue::Repeat);
    texture.set_parameter(TextureParameter::TextureMinFilter, TextureParameterValue::LinearMipmapLinear);
    texture.set_parameter(TextureParameter::TextureMagFilter, TextureParameterValue::Linear);
    {
        let data = include_bytes!("./resources/container.raw");
        texture.set_image_2d(512, 512, data, TextureFormat::UnsignedByte);
        texture.generate_mipmap();
    }

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