use sifis_api::Sifis;

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    // Creation of SIFIS-HOME API context
    let sifis = Sifis::new().await?;
    // Look for every lamps in an environment
    let lamps = sifis.lamps().await?;

    for lamp in lamps {
        // Get lamp state, whether it is on or off
        let on_off: &str = if lamp.get_on_off().await? {
            "On"
        } else {
            "Off"
        };
        // Get brightness
        let brightness: u8 = lamp.get_brightness().await?;
        // Print lamp identifier, on/off state, and brightness
        println!("{:<15} {:<7} {:<5} ", lamp.id, on_off, brightness);
        // Turn on the lamp
        lamp.turn_on().await?;
    }

    Ok(())
}
