# MAGE

My Awesome Game Engine (MAGE) is a learning project. I developed it as a learning project for game development. I will
not publish it as a crate nor provide support for it.

## Design Tenets

1. **Rendering library agnostic**: The public interface of the library does not include any library specific types.
   Although internally it will use OpenGL, the user of the library should not be aware of that. This allows for later
   support of other platforms. In particular, I'm interested in adding WebGPU and Metal.

2. **ECS**: The library will use the ECS paradigm with hecs under the hoods. It will package any game specific
   functionality as a system with a set of related components.

3. **Domain driven folder structure**: The library organizes subcrates per domain and not per type. I.e. instead of
   having
   `crate::components::{CameraControl, Mesh}` and `crate::systems::{FpsCamera, MeshRendering}`, it will
   have `crate::camera::{CameraControl, FpsCamera}` and
   `crate::rendering::{Mesh, MeshRendering}`.

4. **Functionality first**: I have an awful tendency to spend a lot of time optimizing upfront. Given that this is a
   learning project, and I do not know what to optimize yet, I will make a point to make the library's goal to function
   and not to be the best performant. If it can barely run at 60FPS, that is enough.

5. **Silent failure**: This is a game engine. Errors should not cause panic. Rather, the engine will log them and try to
   recover from them.

## Library Structure

- **`crate::core`**: This section contains the main functionality for a game. Particularly, the structures `Game`
  , `World` and
  `Window`.

- **`crate::rendering`**: This section contains all the rendering logic. It will contain the following substructure:
    - **`crate::rendering::opengl`**: All the wrappers around OpenGL specific concepts. This is the only section of the
      library that will contain OpenGL commands.
    - **`crate::rendering::{instanced,lights,model,bordered,normal_mapping,transparent,skybox}`**:
      Contains logics to render those specific type of game objects.
    - **`crate::rendering::forward`**: Forward rendering system.

- **`crate::resources`**: Resource logic loading. From shader program content, to full on models, passing through
  textures, fonts, and sounds.

- **`crate::physics`**: This section contains all the logics related with game physics, including the `PhysicsEngine`.

- **`crate::gameplay`**: This section contains all gameplay logics, including cameras and input management.
