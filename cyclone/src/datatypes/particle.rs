use super::Vector3;
use super::super::integrator;

pub struct Particle {
  pub position: Vector3,
  pub inv_mass: f64,
  pub damping: f64,
  velocity: Vector3,
  accel: Vector3,
  accumulated_force: Vector3 
}

impl Particle {
  pub fn new(position: Vector3, inv_mass: f64, damping: f64) -> Self {
    Self {
      position,
      inv_mass: inv_mass,
      damping: damping,
      velocity: Vector3::new_zero(),
      accel: Vector3::new_zero(),
      accumulated_force: Vector3::new_zero()
    }
  }

  pub fn velocity(&self) -> Vector3 {
    self.velocity
  }

  pub fn accel(&self) -> Vector3 {
    self.accel
  }

  pub fn accumulated_force(&self) -> Vector3 {
    self.accumulated_force
  }

  pub fn add_force(&mut self, force: &Vector3) {
    self.accumulated_force += *force;
  }
}

impl integrator::Integratable for Particle {
  fn integrate(&mut self, duration: f64) {
    if self.inv_mass <= 0.0 {
      return;
    }
    
    self.position += self.velocity * duration;
    
    self.accel = self.accumulated_force * self.inv_mass;

    self.velocity += self.accel * duration;
    self.velocity *= self.damping.powf(duration);

    self.accumulated_force = Vector3::new_zero();
  }
}

