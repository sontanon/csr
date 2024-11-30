use crate::errors::GameErrors;

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
pub struct SpiceAmount {
    pub turmeric: u8,
    pub saffron: u8,
    pub cardamon: u8,
    pub cinnamon: u8,
    pub vector: [u8; 4],
}

impl SpiceAmount {
    pub fn contains(&self, other: &SpiceAmount) -> bool {
        self.turmeric >= other.turmeric
            && self.saffron >= other.saffron
            && self.cardamon >= other.cardamon
            && self.cinnamon >= other.cinnamon
    }
    // pub fn subtract(self, other: &SpiceAmount) -> Result<Self, GameErrors> {
    //     if !self.contains(other) {
    //         return Err(GameErrors::CannotSubtractSpiceAmount(self));
    //     }

    //     Ok(spice_amount!(
    //         self.turmeric - other.turmeric,
    //         self.saffron - other.saffron,
    //         self.cardamon - other.cardamon,
    //         self.cinnamon - other.cinnamon
    //     ))
    // }
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
impl Into<[u8; 4]> for SpiceAmount {
    fn into(self) -> [u8; 4] {
        self.vector
    }
}

#[macro_export]
/// A macro to create a `SpiceAmount` struct with the given amounts of spices.
///
/// # Arguments
///
/// * `$turmeric` - The amount of turmeric.
/// * `$saffron` - The amount of saffron.
/// * `$cardamon` - The amount of cardamon.
/// * `$cinnamon` - The amount of cinnamon.
///
/// # Examples
/// ```
/// use libcsr::{spice_amount, spice::SpiceAmount};
/// let spice_amount = spice_amount!(1, 2, 3, 4);
/// assert_eq!(spice_amount, SpiceAmount { turmeric: 1, saffron: 2, cardamon: 3, cinnamon: 4, vector: [1, 2, 3, 4] });
/// ```
macro_rules! spice_amount {
    ($turmeric:expr, $saffron:expr, $cardamon:expr, $cinnamon:expr) => {
        SpiceAmount {
            turmeric: $turmeric,
            saffron: $saffron,
            cardamon: $cardamon,
            cinnamon: $cinnamon,
            vector: [$turmeric, $saffron, $cardamon, $cinnamon],
        }
    };
}
