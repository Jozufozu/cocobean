use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Pos {
    /// A position relative to the the target's head.
    ///
    /// "caret" coordinates.
    Local(f64, f64, f64),
    /// A position with mixed relative and absolute coordinates.
    World(Coord, Coord, Coord),
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pos::Local(x, y, z) => write!(f, "^{} ^{} ^{}", x, y, z),
            Pos::World(x, y, z) => write!(f, "{} {} {}", x, y, z),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rotation {
    pub pitch: Coord,
    pub yaw: Coord,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.pitch, self.yaw)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coord {
    pub mode: Mode,
    pub coord: f64,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.mode {
            Mode::Absolute => write!(f, "{}", self.coord),
            Mode::Relative => write!(f, "~{}", self.coord),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mode {
    Absolute,
    Relative,
}

pub trait IntoCoord {
    fn abs(self) -> Coord;

    fn rel(self) -> Coord;
}

pub trait IntoPos {
    fn abs(self) -> Pos;

    fn rel(self) -> Pos;

    fn loc(self) -> Pos;
}

impl Coord {
    pub fn relative<T: IntoCoord>(coord: T) -> Self {
        coord.rel()
    }

    pub fn absolute<T: IntoCoord>(coord: T) -> Self {
        coord.abs()
    }
}

impl Pos {
    pub fn of(x: Coord, y: Coord, z: Coord) -> Self {
        Pos::World(x, y, z)
    }
}

impl<T: Into<f64>> IntoCoord for T {
    fn abs(self) -> Coord {
        Coord {
            mode: Mode::Absolute,
            coord: self.into(),
        }
    }

    fn rel(self) -> Coord {
        Coord {
            mode: Mode::Relative,
            coord: self.into(),
        }
    }
}

impl<X, Y, Z> IntoPos for (X, Y, Z)
where
    X: Into<f64>,
    Y: Into<f64>,
    Z: Into<f64>,
{
    fn abs(self) -> Pos {
        Pos::World(self.0.abs(), self.1.abs(), self.2.abs())
    }

    fn rel(self) -> Pos {
        Pos::World(self.0.rel(), self.1.rel(), self.2.rel())
    }

    fn loc(self) -> Pos {
        Pos::Local(self.0.into(), self.1.into(), self.2.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_into_relative() {
        let coord = Coord {
            mode: Mode::Relative,
            coord: 2.0,
        };

        assert_eq!(2.0.rel(), coord);
        assert_eq!(2u32.rel(), coord);
        assert_eq!(2.rel(), coord);
    }

    #[test]
    fn test_coord_into_absolute() {
        let coord = Coord {
            mode: Mode::Absolute,
            coord: 2.0,
        };

        assert_eq!(2.0.abs(), coord);
        assert_eq!(2u32.abs(), coord);
        assert_eq!(2.abs(), coord);
    }

    #[test]
    fn test_pos_into() {
        let pos_abs = Pos::World(
            Coord {
                mode: Mode::Absolute,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 200.0,
            },
        );
        let pos_rel = Pos::World(
            Coord {
                mode: Mode::Relative,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 200.0,
            },
        );
        let pos_loc = Pos::Local(2.0, 20.0, 200.0);

        assert_eq!(pos_abs, (2, 20, 200).abs());
        assert_eq!(pos_rel, (2, 20, 200).rel());
        assert_eq!(pos_loc, (2, 20, 200).loc());
    }

    #[test]
    fn test_pos_mixed_type_into() {
        let pos_abs = Pos::World(
            Coord {
                mode: Mode::Absolute,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 200.0,
            },
        );
        let pos_rel = Pos::World(
            Coord {
                mode: Mode::Relative,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 200.0,
            },
        );
        let pos_loc = Pos::Local(2.0, 20.0, 200.0);

        assert_eq!(pos_abs, (2u8, 20i16, 200f32).abs());
        assert_eq!(pos_rel, (2.0, 20u32, 200).rel());
        assert_eq!(pos_loc, (2f32, 20f64, 200u32).loc());
    }

    #[test]
    fn test_coord_display() {
        let coord = Coord {
            mode: Mode::Relative,
            coord: 2.0,
        };

        let abs = Coord {
            mode: Mode::Absolute,
            coord: 2.0,
        };

        assert_eq!("~2", coord.to_string());
        assert_eq!("2", abs.to_string());
    }

    #[test]
    fn test_pos_display() {
        let pos_abs = Pos::World(
            Coord {
                mode: Mode::Absolute,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Absolute,
                coord: 200.0,
            },
        );
        let pos_rel = Pos::World(
            Coord {
                mode: Mode::Relative,
                coord: 2.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 20.0,
            },
            Coord {
                mode: Mode::Relative,
                coord: 200.0,
            },
        );
        let pos_loc = Pos::Local(2.0, 20.0, 200.0);

        assert_eq!("2 20 200", pos_abs.to_string());
        assert_eq!("~2 ~20 ~200", pos_rel.to_string());
        assert_eq!("^2 ^20 ^200", pos_loc.to_string());
    }
}
