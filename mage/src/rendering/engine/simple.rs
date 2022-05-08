use crate::gameplay::camera::Camera;
use crate::rendering::engine::{Engine, SHADER_LIBRARY};
use crate::rendering::model::mesh::{Mesh, RenderingMesh};
use crate::rendering::opengl::buffer::{Buffer, BufferType};
use crate::rendering::opengl::program::Program;
use crate::rendering::opengl::shader::ShaderType;
use crate::rendering::opengl::{clear, enable, set_clear_color, DrawingBuffer, Feature};
use crate::rendering::Transform;
use crate::resources::shader::ShaderLoader;
use crate::MageError;
use hecs::World;
use nalgebra::{Matrix4, Vector3, Vector4};

const VERTEX_SHADER: &str = "simple-rendering-vertex.glsl";
const FRAGMENT_SHADER: &str = "simple-rendering-fragment.glsl";

pub struct SimpleEngine<C: Camera> {
    camera: C,
    clear_color: Vector3<f32>,
    program: Program,
    uniform_buffer: Buffer,
}

impl<C: Camera> SimpleEngine<C> {
    pub fn new(camera: C, clear_color: Vector3<f32>) -> Result<SimpleEngine<C>, MageError> {
        let shader_loader = ShaderLoader::new(&SHADER_LIBRARY)?;
        let program = Program::new(
            shader_loader.load(ShaderType::Vertex, VERTEX_SHADER)?,
            shader_loader.load(ShaderType::Fragment, FRAGMENT_SHADER)?,
        )?;
        let uniform_buffer = Buffer::new(BufferType::Uniform);
        let buffer_size = Matrix4::<f32>::identity().len() * 2;
        uniform_buffer.bind();
        uniform_buffer.allocate_data::<f32>(buffer_size);
        uniform_buffer.unbind();
        uniform_buffer.link_to_binding_point(0, 0, buffer_size);
        Ok(SimpleEngine {
            camera,
            clear_color,
            program,
            uniform_buffer,
        })
    }

    fn setup_globals(&self) {
        let projection = self.camera.projection();
        let view = self.camera.look_at_matrix();
        self.uniform_buffer.bind();
        self.uniform_buffer
            .set_sub_data(0, view.len(), view.as_slice());
        self.uniform_buffer
            .set_sub_data(view.len(), projection.len(), projection.as_slice());
        self.uniform_buffer.unbind();
        self.program
            .set_uniform_v3("viewPos", self.camera.position());
    }
}

impl<C: Camera> Engine for SimpleEngine<C> {
    fn setup(&self, world: &mut World) -> Result<(), MageError> {
        enable(Feature::Depth);
        let mut rendering_mesh = vec![];
        for (_e, (mesh, transform)) in world.query_mut::<(&Mesh, &Transform)>() {
            rendering_mesh.push((mesh.to_rendering_mesh()?, transform.clone()));
        }
        for (rendering_mesh, transform) in rendering_mesh {
            world.spawn((rendering_mesh, transform));
        }
        set_clear_color(Vector4::new(
            self.clear_color.x,
            self.clear_color.y,
            self.clear_color.z,
            1.0,
        ));
        Ok(())
    }

    fn render(&self, world: &mut World, _delta_time: f32) -> Result<(), MageError> {
        clear(&[DrawingBuffer::Color, DrawingBuffer::Depth]);
        self.program.use_program();
        self.setup_globals();
        for (_e, (mesh, transform)) in world.query_mut::<(&RenderingMesh, &Transform)>() {
            mesh.attach_to_program(&self.program);
            self.program
                .set_uniform_matrix4("model", transform.get_model_matrix());
            mesh.draw();
        }
        Ok(())
    }
}
