use crate::utils::TcpUtils;
use crate::MCType;
use std::fmt::Debug;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct MCAngle(pub i8);

impl MCAngle {
    /// From -360 to 360
    pub fn from_degrees(deegree: i16) -> Self {
        Self((deegree % 360 / 360 * 256) as i8)
    }
}

impl MCType for MCAngle {
    fn pack(&self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    fn unpack(src: &mut dyn Read) -> Self {
        Self(src.read_byte() as i8)
    }
}
