use nalgebra::{Vector3, Vector4};

pub struct ParticlesParametersBuilder {
    brightness_range: (f32, f32),
    visibility_seconds: f32,
    life_seconds: f32,
    mult_velocity: f32,
    max_particles: u32,
    new_particles_per_cycle: u32,
    offset: Vector3<f32>,
    position_random_offset_range: (f32, f32),
}

impl Default for ParticlesParametersBuilder {
    fn default() -> Self {
        ParticlesParametersBuilder::new()
    }
}

impl ParticlesParametersBuilder {
    pub fn new() -> ParticlesParametersBuilder {
        ParticlesParametersBuilder {
            brightness_range: (0.5, 1.5),
            visibility_seconds: 0.4,
            life_seconds: 1.0,
            mult_velocity: 0.1,
            max_particles: 500,
            new_particles_per_cycle: 2,
            offset: Vector3::zeros(),
            position_random_offset_range: (-5.0, 5.0),
        }
    }

    pub fn with_offset(self, offset: Vector3<f32>) -> Self {
        ParticlesParametersBuilder {
            offset,
            brightness_range: self.brightness_range,
            visibility_seconds: self.visibility_seconds,
            life_seconds: self.life_seconds,
            mult_velocity: self.mult_velocity,
            max_particles: self.max_particles,
            new_particles_per_cycle: self.new_particles_per_cycle,
            position_random_offset_range: self.position_random_offset_range,
        }
    }

    pub fn build(self) -> ParticlesParameters {
        ParticlesParameters {
            brightness_range: self.brightness_range,
            dt_mult_alpha: 1.0 / self.visibility_seconds,
            dt_mult_life: 1.0 / self.life_seconds,
            mult_velocity: self.mult_velocity,
            max_particles: self.max_particles,
            new_particles_per_cycle: self.new_particles_per_cycle,
            offset: self.offset,
            position_random_offset_range: self.position_random_offset_range,
        }
    }
}

pub struct ParticlesParameters {
    pub(crate) brightness_range: (f32, f32),
    pub(crate) dt_mult_alpha: f32,
    pub(crate) dt_mult_life: f32,
    pub(crate) mult_velocity: f32,
    pub(crate) max_particles: u32,
    pub(crate) new_particles_per_cycle: u32,
    pub(crate) offset: Vector3<f32>,
    pub(crate) position_random_offset_range: (f32, f32),
}

pub struct Particle {
    pub(crate) color: Vector4<f32>,
    pub(crate) life: f32,
    pub(crate) position: Vector3<f32>,
    pub(crate) velocity: Vector3<f32>,
}

pub struct ParticleGenerator {
    pub(crate) last_used_particle: usize,
    pub(crate) parameters: ParticlesParameters,
    pub(crate) particles: Vec<Particle>,
}

impl ParticleGenerator {
    pub fn new(parameters: ParticlesParameters) -> ParticleGenerator {
        let mut particles = Vec::with_capacity(parameters.max_particles as _);
        for _ in 0..parameters.max_particles {
            particles.push(Particle {
                color: Vector4::zeros(),
                life: 0.0,
                position: Vector3::zeros(),
                velocity: Vector3::zeros(),
            });
        }
        ParticleGenerator {
            last_used_particle: 0,
            parameters,
            particles,
        }
    }
}
