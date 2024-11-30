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
///
/// ```
/// use libcsr::spice_amount;
/// let amount = spice_amount!(1, 2, 3, 4);
/// assert_eq!(amount.turmeric, 1);
/// assert_eq!(amount.saffron, 2);
/// assert_eq!(amount.cardamon, 3);
/// assert_eq!(amount.cinnamon, 4);
/// assert_eq!(amount.vector, [1, 2, 3, 4]);
/// ```
macro_rules! spice_amount {
    ($turmeric:expr, $saffron:expr, $cardamon:expr, $cinnamon:expr) => {
        $crate::spice::SpiceAmount {
            turmeric: $turmeric,
            saffron: $saffron,
            cardamon: $cardamon,
            cinnamon: $cinnamon,
            vector: [$turmeric, $saffron, $cardamon, $cinnamon],
        }
    };
}

#[macro_export]
/// A macro to create a `PointsCard`` with a given point value and spice cost.
///
/// # Arguments
///
/// * `$points` - The point value of the card.
/// * `[$turmeric, $saffron, $cardamon, $cinnamon]` - The cost of the card.
///
/// # Examples
///
/// ```
/// use libcsr::points_card;
/// let card = points_card!(6, [2, 2, 0, 0]);
/// assert_eq!(card.points, 6);
/// assert_eq!(card.cost.turmeric, 2);
/// assert_eq!(card.cost.saffron, 2);
/// assert_eq!(card.cost.cardamon, 0);
/// assert_eq!(card.cost.cinnamon, 0);
/// ```
macro_rules! points_card {
    ($points:expr, [$turmeric:expr, $saffron:expr, $cardamon:expr, $cinnamon:expr]) => {
        $crate::cards::PointsCard {
            points: $points,
            cost: $crate::spice::SpiceAmount {
                turmeric: $turmeric,
                saffron: $saffron,
                cardamon: $cardamon,
                cinnamon: $cinnamon,
                vector: [$turmeric, $saffron, $cardamon, $cinnamon],
            },
        }
    };
}
#[cfg(test)]
mod tests {
    use crate::{cards::PointsCard, points_card, spice::SpiceAmount, spice_amount};

    #[test]
    fn test_spice_amount_macro() {
        let amount = spice_amount!(1, 2, 3, 4);
        let expected_amount = SpiceAmount {
            turmeric: 1,
            saffron: 2,
            cardamon: 3,
            cinnamon: 4,
            vector: [1, 2, 3, 4],
        };
        assert_eq!(amount, expected_amount);
    }

    #[test]
    fn test_points_card_macro() {
        let card = points_card!(6, [2, 2, 0, 0]);
        let expected_card = PointsCard {
            points: 6,
            cost: spice_amount!(2, 2, 0, 0),
        };
        assert_eq!(card, expected_card);
    }
}
