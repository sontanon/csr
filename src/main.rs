use libcsr::{GameErrors, SpiceCube};

fn main() -> Result<(), GameErrors> {
    println!("Initializing the game.");

    let mut sample_cube = SpiceCube::Saffron;
    let upgrade_steps: u8 = 2;

    println!("I have a {sample_cube:?}");

    if let Ok(upgraded_cube) = sample_cube.upgrade(upgrade_steps) {
        sample_cube = upgraded_cube;
        println!("Upgraded cube {upgrade_steps} step(s) to {sample_cube:?}.");
    } else {
        println!("Failed to upgrade cube {sample_cube:?} by {upgrade_steps} step(s).");
    }
    Ok(())
}
