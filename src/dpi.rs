use serde::Deserialize;

pub type ScaleFactor = f64;
pub type PixelCount = isize;

#[derive(Debug)]
pub enum Kind {
    Logical,
    Physical,
}

/// A size represented in logical pixels.
#[derive(Deserialize, Clone, Debug)]
pub struct LogicalSize {
    width: PixelCount,
    height: PixelCount,
}

impl LogicalSize {
    pub fn kind() -> Kind {
        Kind::Logical
    }

    pub fn width(&self) -> PixelCount {
        self.width
    }

    pub fn height(&self) -> PixelCount {
        self.height
    }
}

/// A size represented in physical pixels.
#[derive(Deserialize, Clone, Debug)]
pub struct PhysicalSize {
    width: PixelCount,
    height: PixelCount,
}

impl PhysicalSize {
    pub fn kind() -> Kind {
        Kind::Physical
    }

    pub fn width(&self) -> PixelCount {
        self.width
    }

    pub fn height(&self) -> PixelCount {
        self.height
    }

    /// Converts the physical size to a logical one.
    pub fn as_logical(&self, scale_factor: ScaleFactor) -> LogicalSize {
        LogicalSize {
            width: (self.width as f64 / scale_factor) as PixelCount,
            height: (self.height as f64 / scale_factor) as PixelCount,
        }
    }
}

///  A position represented in logical pixels.
#[derive(Deserialize, Clone, Debug)]
pub struct LogicalPosition {
    x: PixelCount,
    y: PixelCount,
}

impl LogicalPosition {
    pub fn kind() -> Kind {
        Kind::Logical
    }

    pub fn x(&self) -> PixelCount {
        self.x
    }

    pub fn y(&self) -> PixelCount {
        self.y
    }
}

///  A position represented in physical pixels.
#[derive(Deserialize, Clone, Debug)]
pub struct PhysicalPosition {
    x: PixelCount,
    y: PixelCount,
}

impl PhysicalPosition {
    pub fn kind() -> Kind {
        Kind::Physical
    }

    pub fn x(&self) -> PixelCount {
        self.x
    }

    pub fn y(&self) -> PixelCount {
        self.y
    }

    /// Converts the physical position to a logical one.
    pub fn as_logical(&self, scale_factor: ScaleFactor) -> LogicalPosition {
        LogicalPosition {
            x: (self.x as f64 / scale_factor) as PixelCount,
            y: (self.y as f64 / scale_factor) as PixelCount,
        }
    }
}
