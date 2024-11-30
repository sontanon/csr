use crate::player::MAX_CARAVAN_SIZE;
use crate::spice::SpiceAmount;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum GameErrors {
    #[error("Cannot upgrade a spice to itself")]
    CannotUpgradeToSelf,

    #[error("Cannot upgrade past Cinnamon (highest level)")]
    CannotUpgradePastCinnamon,

    #[error("Cannot have more than {MAX_CARAVAN_SIZE} spices in caravan")]
    MaxSpiceCapacityReached,

    #[error("Cannot subtract spice amount from another spice amount")]
    CannotSubtractSpiceAmount(SpiceAmount),

    #[error("Internal logic error occurred")]
    InternalLogicError,
}
