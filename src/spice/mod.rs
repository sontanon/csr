use crate::errors::GameErrors;
use crate::spice_amount;

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
    /// use libcsr::spice::SpiceCube;
    /// let cube = SpiceCube::Turmeric;
    /// let upgraded_cube = cube.upgrade(1).unwrap();
    /// assert_eq!(upgraded_cube, SpiceCube::Saffron);
    /// ```
    ///
    /// Upgrade Saffron to Cinnamon by 2 steps (notice the `Result` type):
    ///
    /// ```
    /// use libcsr::spice::SpiceCube;
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
    /// use libcsr::{spice::SpiceCube, errors::GameErrors};
    /// let cube = SpiceCube::Saffron;
    /// let upgrade_result = cube.upgrade(0);
    /// assert_eq!(upgrade_result, Err(GameErrors::CannotUpgradeToSelf));
    /// ```
    ///
    /// Returns `GameErrors::CannotUpgradePastCinnamon` if the number of steps would upgrade the cube past Cinnamon.
    ///
    /// ```
    /// use libcsr::{spice::SpiceCube, errors::GameErrors};
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
/// Represents an amount of spices.
///
/// The vector field contains duplicate information but having the separate fields makes it easier to work with and build amounts to avoid indexing errors.
pub struct SpiceAmount {
    pub turmeric: u8,
    pub saffron: u8,
    pub cardamon: u8,
    pub cinnamon: u8,
    pub vector: [u8; 4],
}

impl SpiceAmount {
    /// Check if another `SpiceAmount` is contained within this `SpiceAmount`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libcsr::{spice_amount, spice::SpiceAmount};
    /// let spice_amount = spice_amount!(2, 2, 2, 2);
    /// let other_spice_amount = spice_amount!(1, 1, 1, 1);
    /// assert!(spice_amount.contains(&other_spice_amount));
    /// ```
    ///
    /// ```
    /// use libcsr::{spice_amount, spice::SpiceAmount};
    /// let spice_amount = spice_amount!(2, 2, 2, 2);
    /// let other_spice_amount = spice_amount!(3, 3, 3, 3);
    /// assert!(!spice_amount.contains(&other_spice_amount));
    /// ```
    pub fn contains(&self, other: &SpiceAmount) -> bool {
        self.turmeric >= other.turmeric
            && self.saffron >= other.saffron
            && self.cardamon >= other.cardamon
            && self.cinnamon >= other.cinnamon
    }
    
    /// Adds another `SpiceAmount` to this `SpiceAmount`.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use libcsr::{spice_amount, spice::SpiceAmount};
    /// let first_amount = spice_amount!(2, 1, 4, 3);
    /// let other_amount = spice_amount!(1, 1, 4, 1);
    /// let result = first_amount.add(&other_amount);
    /// let expected_result = spice_amount!(3, 2, 8, 4);
    /// assert_eq!(result, expected_result);
    /// ```
    pub fn add(&self, other: &SpiceAmount) -> SpiceAmount {
        spice_amount!(
            self.turmeric + other.turmeric,
            self.saffron + other.saffron,
            self.cardamon + other.cardamon,
            self.cinnamon + other.cinnamon
        )
    }

    /// Attempt to subtract another `SpiceAmount` from this `SpiceAmount`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libcsr::{spice_amount, spice::SpiceAmount};
    /// let first_amount = spice_amount!(2, 1, 4, 3);
    /// let other_amount = spice_amount!(1, 1, 4, 1);
    /// let result = first_amount.subtract(&other_amount).unwrap();
    /// let expected_result = spice_amount!(1, 0, 0, 2);
    /// assert_eq!(result, expected_result);
    /// ```
    ///
    /// # Errors
    ///
    /// ```
    /// use libcsr::{spice_amount, spice::SpiceAmount, errors::GameErrors};
    /// let first_amount = spice_amount!(2, 1, 4, 3);
    /// let other_amount = spice_amount!(3, 2, 4, 0);
    /// let missing_amount = spice_amount!(1, 1, 0, 0);
    /// let result = first_amount.subtract(&other_amount);
    /// assert_eq!(result, Err(GameErrors::CannotSubtractSpiceAmount(first_amount, missing_amount)));
    /// ```
    ///
    /// Returns `GameErrors::CannotSubtractSpiceAmount` if the other `SpiceAmount` is not contained within this `SpiceAmount`. Notice that the returned error contains the original `SpiceAmount`.
    ///
    ///
    pub fn subtract(self, other: &SpiceAmount) -> Result<Self, GameErrors> {
        if !self.contains(other) {
            let missing = spice_amount!(
                other.turmeric.saturating_sub(self.turmeric),
                other.saffron.saturating_sub(self.saffron),
                other.cardamon.saturating_sub(self.cardamon),
                other.cinnamon.saturating_sub(self.cinnamon)
            );
            return Err(GameErrors::CannotSubtractSpiceAmount(self, missing));
        }

        Ok(spice_amount!(
            self.turmeric - other.turmeric,
            self.saffron - other.saffron,
            self.cardamon - other.cardamon,
            self.cinnamon - other.cinnamon
        ))
    }
}

/// A builder for the `SpiceAmount` struct.
///
/// # Examples
///
/// ```
/// use libcsr::spice::{SpiceAmount, SpiceAmountBuilder};
/// let spice_amount = SpiceAmountBuilder::new().turmeric(1).cardamon(3).build();
/// let expected_spice_amount = SpiceAmount { turmeric: 1, saffron: 0, cardamon: 3, cinnamon: 0, vector: [1, 0, 3, 0] };
/// assert_eq!(spice_amount, expected_spice_amount);
/// ```
#[derive(Default)]
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

/// Create a `SpiceAmount` from a `[u8; 4]` array.
///
/// # Examples
///
/// ```
/// use libcsr::spice::SpiceAmount;
/// let spice_array = [1, 2, 3, 4];
/// let spice_amount = SpiceAmount::from(spice_array);
/// let expected_spice_amount = SpiceAmount { turmeric: 1, saffron: 2, cardamon: 3, cinnamon: 4, vector: [1, 2, 3, 4] };
/// assert_eq!(spice_amount, expected_spice_amount);
/// ```
///  
impl From<[u8; 4]> for SpiceAmount {
    fn from(spice_array: [u8; 4]) -> Self {
        Self {
            turmeric: spice_array[0],
            saffron: spice_array[1],
            cardamon: spice_array[2],
            cinnamon: spice_array[3],
            vector: spice_array,
        }
    }
}

/// Convert a `SpiceAmount` into a `[u8; 4]` array.
///
/// # Examples
///
/// ```
/// use libcsr::spice::SpiceAmount;
/// let spice_amount = SpiceAmount { turmeric: 1, saffron: 2, cardamon: 3, cinnamon: 4, vector: [1, 2, 3, 4] };
/// let spice_array: [u8; 4] = spice_amount.into();
/// let expected_spice_array = [1, 2, 3, 4];
/// assert_eq!(spice_array, expected_spice_array);
/// ```
impl From<SpiceAmount> for [u8; 4] {
    fn from(spice_amount: SpiceAmount) -> [u8; 4] {
        spice_amount.vector
    }
}