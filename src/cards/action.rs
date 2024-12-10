use super::ActionCard;
use crate::spice_amount;

pub const STARTING_ACTION_CARDS: [ActionCard; 2] = [
    // Starting cards.
    ActionCard::Gain(spice_amount!(2, 0, 0, 0)),
    ActionCard::Upgrade(2),
];

pub const PURCHASABLE_ACTION_CARDS: [ActionCard; 9] = [
    // Spice cards.
    ActionCard::Gain(spice_amount!(3, 0, 0, 0)),
    ActionCard::Gain(spice_amount!(4, 0, 0, 0)),
    ActionCard::Gain(spice_amount!(1, 1, 0, 0)),
    ActionCard::Gain(spice_amount!(2, 1, 0, 0)),
    ActionCard::Gain(spice_amount!(0, 2, 0, 0)),
    ActionCard::Gain(spice_amount!(0, 0, 1, 0)),
    ActionCard::Gain(spice_amount!(1, 0, 1, 0)),
    ActionCard::Gain(spice_amount!(0, 0, 0, 1)),
    // Single upgrade card in the deck.
    ActionCard::Upgrade(3),
    // Exchange cards.
];
