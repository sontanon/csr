pub mod cards;
pub mod errors;
pub mod player;
pub mod spice;

#[cfg(test)]
mod tests {
    use crate::errors::GameErrors;
    use crate::spice::SpiceCube;

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
