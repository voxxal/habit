use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StreakBitSet {
    pub _bit_capacity: u32,
    pub _buffer: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct TileData {
    pub id: String,
    pub name: String,
    pub streak: StreakBitSet,
    pub startTime: String,
    pub lastCheck: String,
}

#[derive(Deserialize, Debug)]
pub struct State {
    pub userId: String,
    pub experience: f64,
    pub level: i16,
    pub tiles: Vec<TileData>,
}