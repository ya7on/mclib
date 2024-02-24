use crate::types::base::MCType;
use crate::utils::TcpUtils;
use std::io::Read;

pub(crate) mod data_array;
pub(crate) mod palleted_container;
pub(crate) mod section;

pub use data_array::DataArray;
pub use palleted_container::PalletedContainer;
pub use section::ChunkSection;

#[derive(Debug, Clone)]
pub struct ChunkData(pub Vec<ChunkSection>);

impl MCType for ChunkData {
    fn pack(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for section in self.0.iter() {
            result.extend(section.pack());
        }
        result
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let mut result = Vec::new();
        // todo use error handling
        let world_height = 384;
        for _ in 0..world_height / 16 - 1 {
            result.push(ChunkSection::unpack(src));
        }
        src.read_byte();

        Self(result)
    }
}
