use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
/// This represents a single spice cube.
///
/// * Turmeric: Level 1 (yellow)
/// * Saffron: Level 2 (red)
/// * Cardamon: Level 3 (green)
/// * Cinnamon: Level 4 (brown)
pub enum SpiceCube {
    Turmeric = 1,
    Saffron = 2,
    Cardamon = 3,
    Cinnamon = 4,
}

impl SpiceCube {
    /// Upgrades a _single_ spice cube by the number of steps specified.
    ///
    /// The upgrade process follows the hierarchy defined by the `SpiceCube` enum:
    ///
    /// `Turmeric -> Saffron -> Cardamon -> Cinnamon`
    ///
    /// # Examples
    ///
    /// Upgrade Turmeric to Saffron by 1 step:
    ///
    /// ```
    /// use libcsr::SpiceCube;
    /// let cube = SpiceCube::Turmeric;
    /// let upgraded_cube = cube.upgrade(1).unwrap();
    /// assert_eq!(upgraded_cube, SpiceCube::Saffron);
    /// ```
    ///
    /// Upgrade Saffron to Cinnamon by 2 steps (notice the `Result` type):
    ///
    /// ```
    /// use libcsr::SpiceCube;
    /// let cube = SpiceCube::Saffron;
    /// let upgrade_result = cube.upgrade(2);
    /// assert_eq!(upgrade_result, Ok(SpiceCube::Cinnamon))
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `GameErrors::CannotUpgradeToSelf` if the number of steps is 0.
    ///
    /// ```
    /// use libcsr::{SpiceCube, GameErrors};
    /// let cube = SpiceCube::Saffron;
    /// let upgrade_result = cube.upgrade(0);
    /// assert_eq!(upgrade_result, Err(GameErrors::CannotUpgradeToSelf));
    /// ```
    ///
    /// Returns `GameErrors::CannotUpgradePastCinnamon` if the number of steps would upgrade the cube past Cinnamon.
    ///
    /// ```
    /// use libcsr::{SpiceCube, GameErrors};
    /// let cube = SpiceCube::Turmeric;
    /// let upgrade_result = cube.upgrade(4);
    /// assert_eq!(upgrade_result, Err(GameErrors::CannotUpgradePastCinnamon));
    /// ```
    ///
    pub fn upgrade(&self, steps: u8) -> Result<Self, GameErrors> {
        if steps == 0 {
            return Err(GameErrors::CannotUpgradeToSelf);
        }

        let current_level = *self as u8;
        let target_level = current_level + steps;

        match target_level {
            2 => Ok(Self::Saffron),
            3 => Ok(Self::Cardamon),
            4 => Ok(Self::Cinnamon),
            _ => Err(GameErrors::CannotUpgradePastCinnamon),
        }
    }
}

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub struct SpiceAmount {
    turmeric: u8,
    saffron: u8,
    cardamon: u8,
    cinnamon: u8,
    vector: [u8; 4],
}

pub struct SpiceAmountBuilder {
    spice_amount: SpiceAmount,
}

impl SpiceAmountBuilder {
    pub fn new() -> Self {
        Self {
            spice_amount: SpiceAmount::default(),
        }
    }

    pub fn turmeric(mut self, turmeric: u8) -> Self {
        self.spice_amount.turmeric = turmeric;
        self.spice_amount.vector[0] = turmeric;
        self
    }

    pub fn saffron(mut self, saffron: u8) -> Self {
        self.spice_amount.saffron = saffron;
        self.spice_amount.vector[1] = saffron;
        self
    }

    pub fn cardamon(mut self, cardamon: u8) -> Self {
        self.spice_amount.cardamon = cardamon;
        self.spice_amount.vector[2] = cardamon;
        self
    }

    pub fn cinnamon(mut self, cinnamon: u8) -> Self {
        self.spice_amount.cinnamon = cinnamon;
        self.spice_amount.vector[3] = cinnamon;
        self
    }

    pub fn build(self) -> SpiceAmount {
        self.spice_amount
    }
}

impl From<[u8; 4]> for SpiceAmount {
    fn from(spice_array: [u8; 4]) -> SpiceAmount {
        SpiceAmountBuilder::new()
            .turmeric(spice_array[0])
            .saffron(spice_array[1])
            .cardamon(spice_array[2])
            .cinnamon(spice_array[3])
            .build()
    }
}
impl Into<[u8; 4]> for SpiceAmount {
    fn into(self) -> [u8; 4] {
        self.vector
    }
}
/// Maximum number of spice cubes a caravan can hold.
pub const MAX_CARAVAN_SIZE: usize = 10;

#[derive(Debug, PartialEq)]
/// This represents a player's caravan, or their inventory.
///
/// A caravan can hold up to [`MAX_CARAVAN_SIZE`] spice cubes.
pub struct Caravan {
    spaces: [Option<SpiceCube>; MAX_CARAVAN_SIZE],
}

impl Caravan {
    /// Get a reference to the private `spaces` array.
    pub fn get_spaces(&self) -> &[Option<SpiceCube>; MAX_CARAVAN_SIZE] {
        &self.spaces
    }
    /// Creates a new `Caravan` from a spice amount.
    ///
    /// # Examples
    ///
    /// Create a caravan with 3 turmeric cubes (e.g., for the 1st player):
    ///
    /// ```
    /// use libcsr::{SpiceCube::Turmeric, SpiceAmount, SpiceAmountBuilder, Caravan};
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(3).build();
    /// let caravan_1st_player = Caravan::from_spice_amount(spice_amount).unwrap();
    /// assert_eq!(caravan_1st_player.current_capacity(), 3);
    /// assert_eq!(caravan_1st_player.get_spaces(), &[Some(Turmeric), Some(Turmeric), Some(Turmeric), None, None, None, None, None, None, None]);
    /// ```
    ///
    /// Create a caravan with 3 turmeric cubes, and 1 saffron cube (e.g., for the 5th player):
    ///
    /// ```
    /// use libcsr::{SpiceCube::{Turmeric, Saffron}, SpiceAmount, SpiceAmountBuilder, Caravan};
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(3).saffron(1).build();
    /// let caravan_5th_player = Caravan::from_spice_amount(spice_amount).unwrap();
    /// assert_eq!(caravan_5th_player.current_capacity(), 4);
    /// assert_eq!(caravan_5th_player.get_spaces(), &[Some(Turmeric), Some(Turmeric), Some(Turmeric), Some(Saffron), None, None, None, None, None, None]);
    /// ```
    /// # Errors
    ///
    /// This function will return an error if the total number of spice cubes exceeds [`MAX_CARAVAN_SIZE`].
    /// ```
    /// use libcsr::{Caravan, GameErrors, MAX_CARAVAN_SIZE, SpiceAmount, SpiceAmountBuilder};
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(MAX_CARAVAN_SIZE as u8).cinnamon(1).build();
    /// let result = Caravan::from_spice_amount(spice_amount);
    /// assert_eq!(result, Err(GameErrors::MaxSpiceCapacityReached));
    /// ```
    pub fn from_spice_amount(spice_amount: SpiceAmount) -> Result<Self, GameErrors> {
        // This copies, it does not move-out.
        let spice_vector: [u8; 4] = spice_amount.into();
        let [turmeric, saffron, cardamon, cinnamon] = spice_vector.map(|x| x as usize);
        let total = turmeric + saffron + cardamon + cinnamon;

        if total > MAX_CARAVAN_SIZE {
            return Err(GameErrors::MaxSpiceCapacityReached);
        }

        let mut spaces = [None; MAX_CARAVAN_SIZE];
        let mut idx = 0;

        // Use slice patterns to fill spaces
        let spices = [
            (turmeric, SpiceCube::Turmeric),
            (saffron, SpiceCube::Saffron),
            (cardamon, SpiceCube::Cardamon),
            (cinnamon, SpiceCube::Cinnamon),
        ];

        for (count, spice) in spices {
            for _ in 0..count {
                spaces[idx] = Some(spice);
                idx += 1;
            }
        }

        Ok(Self { spaces })
    }

    /// Get the total number of spices in the caravan.
    pub fn current_capacity(&self) -> u8 {
        self.spaces.iter().flatten().count() as u8
    }

    /// Get the `SpiceAmount` in the caravan.
    ///
    /// # Examples
    /// ```
    /// use libcsr::{Caravan, SpiceAmountBuilder, SpiceAmount};  
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(3).saffron(1).cardamon(2).cinnamon(4).build();
    /// let caravan = Caravan::from_spice_amount(spice_amount).unwrap();
    /// let expected_spice_amount = SpiceAmount::from([3, 1, 2, 4]);
    /// assert_eq!(caravan.get_spice_amount(), expected_spice_amount);
    /// ```
    pub fn get_spice_amount(&self) -> SpiceAmount {
        SpiceAmountBuilder::new()
            .turmeric(
                self.spaces
                    .iter()
                    .filter(|&&x| x == Some(SpiceCube::Turmeric))
                    .count() as u8,
            )
            .saffron(
                self.spaces
                    .iter()
                    .filter(|&&x| x == Some(SpiceCube::Saffron))
                    .count() as u8,
            )
            .cardamon(
                self.spaces
                    .iter()
                    .filter(|&&x| x == Some(SpiceCube::Cardamon))
                    .count() as u8,
            )
            .cinnamon(
                self.spaces
                    .iter()
                    .filter(|&&x| x == Some(SpiceCube::Cinnamon))
                    .count() as u8,
            )
            .build()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum GameErrors {
    #[error("Cannot upgrade a spice to itself")]
    CannotUpgradeToSelf,

    #[error("Cannot upgrade past Cinnamon (highest level)")]
    CannotUpgradePastCinnamon,

    #[error("Cannot have more than {MAX_CARAVAN_SIZE} spices in caravan")]
    MaxSpiceCapacityReached,

    #[error("Internal logic error occurred")]
    InternalLogicError,
}

enum PlayerAction {
    PlayCard,
    AcquireCard,
    Rest,
    Score,
}

enum ActionCard {
    SpiceCard([u8; 4]),
    ExchangeCard,
    UpgradeCard(u8),
}

struct PointsCard {
    points: u8,
    spices: [u8; 4],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_turmeric() {
        let cube = SpiceCube::Turmeric;

        let upgraded_cube = cube.upgrade(1).unwrap();
        assert_eq!(upgraded_cube, SpiceCube::Saffron);

        let upgraded_cube = cube.upgrade(2).unwrap();
        assert_eq!(upgraded_cube, SpiceCube::Cardamon);

        let upgraded_cube = cube.upgrade(3).unwrap();
        assert_eq!(upgraded_cube, SpiceCube::Cinnamon);
    }

    #[test]
    fn upgrade_turmeric_failure_over_upgrade() {
        let cube = SpiceCube::Turmeric;
        let upgrade_result = cube.upgrade(4);
        assert!(upgrade_result.is_err_and(|x| x == GameErrors::CannotUpgradePastCinnamon));
    }

    #[test]
    fn upgrade_turmeric_failure_under_upgrade() {
        let cube = SpiceCube::Turmeric;
        let upgrade_result = cube.upgrade(0);
        assert!(upgrade_result.is_err_and(|x| x == GameErrors::CannotUpgradeToSelf));
    }
}
