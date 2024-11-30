use crate::errors::GameErrors;
use crate::spice::SpiceAmount;

pub mod action;
pub mod points;

#[derive(Debug)]
pub enum ActionCard {
    Gain(SpiceAmount),
    Exchange(fn(SpiceAmount, u8) -> Result<SpiceAmount, GameErrors>),
    Upgrade(u8),
}

#[derive(Debug)]
pub struct PointsCard {
    pub points: u8,
    pub cost: SpiceAmount,
}
