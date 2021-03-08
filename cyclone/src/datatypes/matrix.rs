use super::Vector3;

#[derive(Clone, Debug)]
pub struct Matrix4x4 {
    pub value: [[f64; 4]; 4]
}

impl Matrix4x4 {
  pub fn zero() -> Self {
    Self {
      value: [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0]
      ]
    }
  }

  pub fn identity() -> Self {
    Self {
      value: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn translate(delta: &Vector3) -> Self {
    Self {
      value: [
        [1.0, 0.0, 0.0, delta.x],
        [0.0, 1.0, 0.0, delta.y],
        [0.0, 0.0, 1.0, delta.z],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn rotate_x(angle: f64) -> Self {
    let cos = angle.cos();
    let sin = angle.sin();
    Self {
      value: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos, sin, 0.0],
        [0.0, -sin, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn rotate_y(angle: f64) -> Self {
    let cos = angle.cos();
    let sin = angle.sin();
    Self {
      value: [
        [cos, 0.0, -sin, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [sin, 0.0, cos, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn rotate_z(angle: f64) -> Self {
    let cos = angle.cos();
    let sin = angle.sin();
    Self {
      value: [
        [cos, -sin, 0.0, 0.0],
        [sin, cos, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn scale(scale: &Vector3) -> Self {
    Self {
      value: [
        [scale.x, 0.0, 0.0, 0.0],
        [0.0, scale.y, 0.0, 0.0],
        [0.0, 0.0, scale.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn mul(a: &Self, b: &Self) -> Self {
    let mut result = Self::zero();

    for k in 0..5 {
      for i in 0..5 {
        let temp = a[i][k];
        for j in 0..5 {
          result[i][j] += temp * b[k][j];
        }
      }
    }
    
    result
  }
}

impl std::ops::Index<usize> for Matrix4x4 {
  type Output = [f64; 4];

  fn index(&self, idx: usize) -> &Self::Output {
    &self.value[idx]
  }
}

impl std::ops::IndexMut<usize> for Matrix4x4 {
  fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
    &mut self.value[idx] 
  }
}

impl std::ops::Mul for Matrix4x4 {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self::mul(&self, &other)
  }
}

impl std::ops::MulAssign for Matrix4x4 {
  fn mul_assign(&mut self, other: Self) {
    *self = Self::mul(self, &other);
  }
}

