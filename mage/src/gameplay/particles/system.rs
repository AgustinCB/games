use std::cell::RefCell;

use hecs::World;
use nalgebra::{Vector3, Vector4};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;

use crate::core::system::System;
use crate::gameplay::particles::ParticleGenerator;
use crate::MageError;
use crate::physics::Velocity;
use crate::rendering::Transform;

pub struct ParticlesSystem {
    rng: RefCell<ThreadRng>,
}

impl Default for ParticlesSystem {
    fn default() -> Self {
        ParticlesSystem::new()
    }
}

impl System for ParticlesSystem {
    fn name(&self) -> &str {
        "Particles System"
    }

    fn start(&self, _world: &mut World) -> Result<(), MageError> {
        Ok(())
    }

    fn early_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }

    fn update(&self, world: &mut World, delta_time: u64) -> Result<(), MageError> {
        for (_, (transform, velocity, particles)) in
        world.query_mut::<(&Transform, &Velocity, &mut ParticleGenerator)>()
        {
            self.update_generator(particles, delta_time as f32, transform.position, velocity.0);
        }
        Ok(())
    }

    fn late_update(&self, _world: &mut World, _delta_time: u64) -> Result<(), MageError> {
        Ok(())
    }
}

impl ParticlesSystem {
    pub fn new() -> ParticlesSystem {
        ParticlesSystem {
            rng: RefCell::new(thread_rng()),
        }
    }

    fn respawn_particle(
        &self,
        generator: &mut ParticleGenerator,
        particle: usize,
        position: Vector3<f32>,
        velocity: Vector3<f32>,
    ) {
        let random_position_offset = self.rng.borrow_mut().gen_range(
            generator.parameters.position_random_offset_range.0
                ..generator.parameters.position_random_offset_range.1,
        );
        let random_brightness = self.rng.borrow_mut().gen_range(
            generator.parameters.brightness_range.0..generator.parameters.brightness_range.1,
        );
        let mut particle = &mut generator.particles[particle];
        particle.position = Vector3::new(
            position.x + random_position_offset + generator.parameters.offset.x,
            position.y + random_position_offset + generator.parameters.offset.y,
            position.z + random_position_offset + generator.parameters.offset.z,
        );
        particle.color = Vector4::new(random_brightness, random_brightness, random_brightness, 1.0);
        particle.life = 1.0;
        particle.velocity = velocity * generator.parameters.mult_velocity;
    }

    fn update_generator(
        &self,
        generator: &mut ParticleGenerator,
        delta_time: f32,
        position: Vector3<f32>,
        velocity: Vector3<f32>,
    ) {
        for _ in 0..generator.parameters.new_particles_per_cycle {
            let index = first_unused_particle(generator);
            self.respawn_particle(generator, index, position, velocity)
        }
        for i in 0..generator.parameters.max_particles {
            let velocity = generator.particles[i as usize].velocity;
            let mut p = &mut generator.particles[i as usize];
            if p.life > 0.0 {
                p.life -= delta_time * generator.parameters.dt_mult_life;
                p.position -= velocity * delta_time;
                p.color.z -= delta_time * generator.parameters.dt_mult_alpha;
            }
        }
    }
}

fn first_unused_particle(generator: &mut ParticleGenerator) -> usize {
    for i in generator.last_used_particle..generator.parameters.max_particles as _ {
        if generator.particles[i].life <= 0.0 {
            generator.last_used_particle = i as usize;
            return i;
        }
    }
    for i in 0..generator.last_used_particle {
        if generator.particles[i].life <= 0.0 {
            generator.last_used_particle = i as usize;
            return i;
        }
    }
    generator.last_used_particle = 0;
    0
}
