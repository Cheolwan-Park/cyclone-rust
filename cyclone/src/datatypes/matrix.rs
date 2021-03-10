use super::Vector3;

#[derive(Clone, Debug)]
pub struct Matrix4x4 {
    pub value: [[f32; 4]; 4]
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

  pub fn translate(delta: Vector3) -> Self {
    Self {
      value: [
        [1.0, 0.0, 0.0, delta.x],
        [0.0, 1.0, 0.0, delta.y],
        [0.0, 0.0, 1.0, delta.z],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn rotate_x(angle: f32) -> Self {
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

  pub fn rotate_y(angle: f32) -> Self {
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

  pub fn rotate_z(angle: f32) -> Self {
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

  pub fn rotate(angle: Vector3) -> Self {
    Self::rotate_z(angle.z) * Self::rotate_y(angle.y) * Self::rotate_x(angle.x)
  }

  pub fn scale(scale: Vector3) -> Self {
    Self {
      value: [
        [scale.x, 0.0, 0.0, 0.0],
        [0.0, scale.y, 0.0, 0.0],
        [0.0, 0.0, scale.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Self {
    let front = (target - eye).normalized();
    let side = Vector3::cross(&front, &up).normalized();
    let new_up = Vector3::cross(&side, &front);

    Self {
      value: [
        [side.x, side.y, side.z, -Vector3::dot(&eye, &side)],
        [new_up.x, new_up.y, new_up.z, -Vector3::dot(&eye, &new_up)],
        [-front.x, -front.y, -front.z, Vector3::dot(&eye, &front)],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
    Self {
      value: [
        [2.0/(right-left), 0.0, 0.0, (left+right)/(left-right)],
        [0.0, 2.0/(top-bottom), 0.0, (bottom+top)/(bottom-top)],
        [0.0, 0.0, 2.0/(near-far), (near+far)/(near-far)],
        [0.0, 0.0, 0.0, 1.0]
      ]
    }
  }

  pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
    Self {
      value: [
        [(2.0*near)/(right-left), 0.0, (right+left)/(right-left), 0.0],
        [0.0, (2.0*near)/(top-bottom), (top+bottom)/(top-bottom), 0.0],
        [0.0, 0.0, (near+far)/(near-far), (2.0*near*far)/(near-far)],
        [0.0, 0.0, -1.0, 0.0]
      ]
    }
  }

  pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
    let fovy_rad = fovy * std::f32::consts::PI / 180.0;
    let top = (fovy_rad/2.0).tan() * near;
    let bottom = -top;
    let left = bottom * aspect;
    let right = top * aspect;

    Self::frustum(left, right, bottom, top, near, far)
  }

  pub fn mul(a: &Self, b: &Self) -> Self {
    let mut result = Self::zero();

    for k in 0..4 {
      for i in 0..4 {
        let temp = a[i][k];
        for j in 0..4 {
          result[i][j] += temp * b[k][j];
        }
      }
    }
    
    result
  }

  pub fn transpose(&mut self) {
    let mut temp: f32;
    for i in 0..4 {
      for j in (i+1)..4 {
        temp = self[i][j];
        self[i][j] = self[j][i];
        self[j][i] = temp;
      }
    }
  }

  pub fn transposed(&self) -> Self {
    let mut result = Self::zero();

    for i in 0..4 {
      for j in 0..4 {
        result[i][j] = self[j][i];
      }
    }

    result
  }
}

impl std::ops::Index<usize> for Matrix4x4 {
  type Output = [f32; 4];

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

