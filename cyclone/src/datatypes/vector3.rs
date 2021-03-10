#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Vector3 {
  pub fn new_zero() -> Self {
    Self { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn invert(&mut self) {
    self.x = -self.x;
    self.y = -self.y;
    self.z = -self.z;
  }

  pub fn inverted(&self) -> Self {
    Self { x: -self.x, y: -self.y, z: -self.z }
  }

  pub fn square_length(&self) -> f32 {
    self.x*self.x + self.y*self.y + self.z*self.z
  }

  pub fn length(&self) -> f32 {
    self.square_length().sqrt()
  }

  pub fn normalize(&mut self) {
    let length = self.length();
    self.x /= length;
    self.y /= length;
    self.z /= length;
  }

  pub fn normalized(&self) -> Self {
    let length = self.length();
    Self {
      x: self.x / length,
      y: self.y / length,
      z: self.z / length
    }
  }

  pub fn dot(a: &Self, b: &Self) -> f32 {
    a.x*b.x + a.y*b.y + a.z*b.z
  }

  pub fn cross(a: &Self, b: &Self) -> Self {
    Self {
      x: a.y*b.z - a.z*b.y,
      y: a.z*b.x - a.x*b.z,
      z: a.x*b.y - a.y*b.x
    }
  }
}

impl std::ops::Add for Vector3 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z
    }
  }
}

impl std::ops::AddAssign for Vector3 {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
    self.z += other.z;
  }
}

impl std::ops::Sub for Vector3 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z
    }
  }
}

impl std::ops::SubAssign for Vector3 {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
    self.z -= other.z;
  }
}

impl std::ops::Div for Vector3 {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    if other.x == 0.0 || other.y == 0.0 || other.z == 0.0 {
      panic!("cannot divide by zero");
    }

    Self {
      x: self.x / other.x,
      y: self.y / other.y,
      z: self.z / other.z
    }
  }
}

impl std::ops::DivAssign for Vector3 {
  fn div_assign(&mut self, other: Self) {
    self.x /= other.x;
    self.y /= other.y;
    self.z /= other.z;
  }
}

impl std::ops::Div<f32> for Vector3 {
  type Output = Self;

  fn div(self, rhs: f32) -> Self::Output {
    if rhs == 0.0 {
      panic!("cannot divide by zero");
    }

    Self {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs
    }
  }
}

impl std::ops::DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, rhs: f32) {
    self.x /= rhs;
    self.y /= rhs;
    self.z /= rhs;
  }
}

impl std::ops::Mul for Vector3 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z
    }
  }
}

impl std::ops::MulAssign for Vector3 {
  fn mul_assign(&mut self, other: Vector3) {
    self.x *= other.x;
    self.y *= other.y;
    self.z *= other.z;
  }
}

impl std::ops::Mul<f32> for Vector3 {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self::Output {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs
    }
  }
}

impl std::ops::MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, rhs: f32) {
    self.x *= rhs;
    self.y *= rhs;
    self.z *= rhs;
  }
}

impl std::ops::Mul<Vector3> for f32 {
  type Output = Vector3;

  fn mul(self, rhs: Vector3) -> Self::Output {
    Vector3 {
      x: self * rhs.x,
      y: self * rhs.y,
      z: self * rhs.z
    }
  }
}

impl std::fmt::Display for Vector3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}