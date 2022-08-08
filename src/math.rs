use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3D {
    #[inline]
    pub fn normalized(self) -> Vec3D {
        return self / self.length();
    }

    #[inline]
    pub fn dot(self, other: Vec3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl Div for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        let recip: f64 = rhs.recip();
        self * recip
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Mul for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3D {
    type Output = Vec3D;

    #[inline]
    fn sub(self, rhs: f64) -> Self::Output {
        Vec3D {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl PartialEq for Vec3D {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Vec3D;

    #[test]
    fn test_normalized() {
        assert_eq!(
            Vec3D {
                x: 4.0,
                y: 3.0,
                z: 5.0
            }
            .normalized(),
            Vec3D {
                x: 0.565685424949238,
                y: 0.4242640687119285,
                z: 0.7071067811865475
            }
        );
    }
    #[test]
    fn test_length() {
        assert_eq!(
            Vec3D {
                x: 4.0,
                y: 3.0,
                z: 0.0
            }
            .length(),
            5.0
        );
        assert_eq!(
            Vec3D {
                x: 0.0,
                y: 4.0,
                z: 3.0
            }
            .length(),
            5.0
        );
    }
    #[test]
    fn supports_add_vec_vec() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } + Vec3D {
                x: 3.0,
                y: 2.0,
                z: 1.0
            },
            Vec3D {
                x: 4.0,
                y: 4.0,
                z: 4.0
            }
        );
    }
    #[test]
    fn supports_add_vec_f64() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } + 1.0,
            Vec3D {
                x: 2.0,
                y: 3.0,
                z: 4.0
            }
        );
    }
    #[test]
    fn supports_mul_vec_vec() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } * Vec3D {
                x: 3.0,
                y: 2.0,
                z: 3.0
            },
            Vec3D {
                x: 3.0,
                y: 4.0,
                z: 9.0
            }
        );
    }
    #[test]
    fn supports_mul_vec_f64() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } * 5.0,
            Vec3D {
                x: 5.0,
                y: 10.0,
                z: 15.0
            }
        );
    }
    #[test]
    fn supports_sub_vec_vec() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } - Vec3D {
                x: 3.0,
                y: 2.0,
                z: 1.0
            },
            Vec3D {
                x: -2.0,
                y: 0.0,
                z: 2.0
            }
        );
    }
    #[test]
    fn supports_sub_vec_f64() {
        assert_eq!(
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            } - 5.0,
            Vec3D {
                x: -4.0,
                y: -3.0,
                z: -2.0
            }
        );
    }
    #[test]
    fn supports_div_vec_vec() {
        assert_eq!(
            Vec3D {
                x: 3.0,
                y: 4.0,
                z: 9.0
            } / Vec3D {
                x: 3.0,
                y: 2.0,
                z: 3.0
            },
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }
    #[test]
    fn supports_div_vec_f64() {
        assert_eq!(
            Vec3D {
                x: 3.0,
                y: 6.0,
                z: 9.0
            } / 3.0,
            Vec3D {
                x: 1.0,
                y: 2.0,
                z: 3.0
            }
        );
    }
}
