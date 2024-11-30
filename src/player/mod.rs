use crate::cards::{ActionCard, PointsCard};
use crate::errors::GameErrors;
use crate::spice::{SpiceAmount, SpiceAmountBuilder, SpiceCube};

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
    /// use libcsr::{spice::{SpiceCube::Turmeric, SpiceAmount, SpiceAmountBuilder}, player::Caravan};
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(3).build();
    /// let caravan_1st_player = Caravan::from_spice_amount(spice_amount).unwrap();
    /// assert_eq!(caravan_1st_player.current_capacity(), 3);
    /// assert_eq!(caravan_1st_player.get_spaces(), &[Some(Turmeric), Some(Turmeric), Some(Turmeric), None, None, None, None, None, None, None]);
    /// ```
    ///
    /// Create a caravan with 3 turmeric cubes, and 1 saffron cube (e.g., for the 5th player):
    ///
    /// ```
    /// use libcsr::{spice::{SpiceCube::{Turmeric, Saffron}, SpiceAmount, SpiceAmountBuilder}, player::Caravan};
    /// let spice_amount = SpiceAmountBuilder::new().turmeric(3).saffron(1).build();
    /// let caravan_5th_player = Caravan::from_spice_amount(spice_amount).unwrap();
    /// assert_eq!(caravan_5th_player.current_capacity(), 4);
    /// assert_eq!(caravan_5th_player.get_spaces(), &[Some(Turmeric), Some(Turmeric), Some(Turmeric), Some(Saffron), None, None, None, None, None, None]);
    /// ```
    /// # Errors
    ///
    /// This function will return an error if the total number of spice cubes exceeds [`MAX_CARAVAN_SIZE`].
    /// ```
    /// use libcsr::{player::{Caravan, MAX_CARAVAN_SIZE}, errors::GameErrors, spice::{SpiceAmount, SpiceAmountBuilder}};
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
    /// use libcsr::{player::Caravan, spice::{SpiceAmountBuilder, SpiceAmount}};  
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

enum PlayerAction {
    PlayCard(ActionCard),
    AcquireCard(ActionCard),
    Rest,
    Score(PointsCard),
}

struct Player {
    caravan: Caravan,
    player_order: u8,
    hand: Vec<ActionCard>,
    discard_pile: Vec<ActionCard>,
    score_pile: Vec<PointsCard>,
    play_history: Vec<PlayerAction>,
}
