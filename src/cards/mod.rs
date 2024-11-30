use crate::errors::GameErrors;
use crate::spice::SpiceAmount;

pub mod action;
pub mod points;

#[derive(Debug, PartialEq)]
pub enum ActionCard {
    Gain(SpiceAmount),
    Exchange(fn(SpiceAmount, u8) -> Result<SpiceAmount, GameErrors>),
    Upgrade(u8),
}

#[derive(Debug, PartialEq)]
pub struct PointsCard {
    pub points: u8,
    pub cost: SpiceAmount,
}

impl PointsCard {
    pub fn purchase(&self, spice_amount: &SpiceAmount) -> Result<(u8, SpiceAmount), GameErrors> {
        let subtracted_amount = spice_amount.subtract(&self.cost)?;
        Ok((self.points, subtracted_amount))
    }
}

#[cfg(test)]
mod tests {
    use crate::{cards::PointsCard, errors::GameErrors, spice_amount};

    #[test]
    fn test_purchase() {
        let card = PointsCard {
            points: 11,
            cost: spice_amount!(3, 0, 0, 2),
        };

        let spice_amount = spice_amount!(4, 2, 1, 2);
        let result = card.purchase(&spice_amount);
        assert_eq!(result, Ok((11, spice_amount!(1, 2, 1, 0))));
    }
}
