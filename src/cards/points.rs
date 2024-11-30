use crate::cards::PointsCard;

/// A macro to create a `PointsCard`` with a given point value and spice cost.
///
/// # Arguments
///
/// * `$points` - The point value of the card.
/// * `[$turmeric, $saffron, $cardamon, $cinnamon]` - The cost of the card.
macro_rules! points_card {
    ($points:expr, [$turmeric:expr, $saffron:expr, $cardamon:expr, $cinnamon:expr]) => {
        PointsCard {
            points: $points,
            cost: crate::spice::SpiceAmount {
                turmeric: $turmeric,
                saffron: $saffron,
                cardamon: $cardamon,
                cinnamon: $cinnamon,
                vector: [$turmeric, $saffron, $cardamon, $cinnamon],
            },
        }
    };
}
pub const POINTS_CARDS: [PointsCard; 36] = [
    points_card!(6, [2, 2, 0, 0]),
    points_card!(7, [3, 2, 0, 0]),
    points_card!(8, [2, 3, 0, 0]),
    points_card!(8, [0, 4, 0, 0]),
    points_card!(8, [2, 0, 2, 0]),
    points_card!(9, [3, 0, 2, 0]),
    points_card!(9, [2, 1, 0, 1]),
    points_card!(10, [0, 5, 0, 0]),
    points_card!(10, [0, 2, 2, 0]),
    points_card!(10, [2, 0, 0, 2]),
    points_card!(11, [2, 0, 3, 0]),
    points_card!(11, [3, 0, 0, 2]),
    points_card!(12, [0, 2, 0, 2]),
    points_card!(12, [1, 1, 1, 1]),
    points_card!(12, [0, 2, 1, 1]),
    points_card!(12, [0, 0, 4, 0]),
    points_card!(12, [1, 0, 2, 1]),
    points_card!(12, [0, 3, 2, 0]),
    points_card!(13, [2, 2, 2, 0]),
    points_card!(13, [0, 2, 3, 0]),
    points_card!(14, [2, 0, 0, 3]),
    points_card!(14, [0, 3, 0, 2]),
    points_card!(14, [0, 0, 2, 2]),
    points_card!(14, [3, 1, 1, 1]),
    points_card!(15, [0, 0, 5, 0]),
    points_card!(15, [2, 2, 0, 2]),
    points_card!(16, [0, 0, 0, 4]),
    points_card!(16, [0, 2, 0, 3]),
    points_card!(16, [1, 3, 1, 1]),
    points_card!(17, [0, 0, 3, 2]),
    points_card!(17, [2, 0, 2, 2]),
    points_card!(18, [0, 0, 2, 3]),
    points_card!(18, [1, 1, 3, 1]),
    points_card!(19, [0, 2, 2, 2]),
    points_card!(20, [0, 0, 0, 5]),
    points_card!(20, [1, 1, 1, 3]),
];