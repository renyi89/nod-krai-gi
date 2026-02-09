// disable lints on generated code
#![allow(clippy::all)]
#![allow(unused)]
#![allow(missing_docs)]
include!("../gen/server_only/_.rs");

impl Vector {
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < 0.0000001 && self.y.abs() < 0.0000001 && self.z.abs() < 0.0000001
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn is_valid_rot(&self) -> bool {
        const VALID_RANGE: std::ops::Range<f32> = 0.0..361.0;

        self.is_valid()
            && VALID_RANGE.contains(&self.x)
            && VALID_RANGE.contains(&self.y)
            && VALID_RANGE.contains(&self.z)
    }
}

impl From<(f32, f32, f32)> for Vector {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vector> for (f32, f32, f32) {
    fn from(value: Vector) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<crate::normal::Vector> for Vector {
    fn from(value: crate::normal::Vector) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector> for crate::normal::Vector {
    fn from(value: Vector) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3({},{},{})", self.x, self.y, self.z)
    }
}
