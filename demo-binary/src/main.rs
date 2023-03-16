use sifis_api::Sifis;

#[tokio::main]
async fn main() -> Result<(), sifis_api::Error> {
    let sifis = Sifis::new().await?;
    let lamps = sifis.lamps().await?;

    for lamp in lamps {
        let on_off: &str = if lamp.get_on_off().await? {
            "On"
        } else {
            "Off"
        };
        let brightness: u8 = lamp.get_brightness().await?;
        println!("{:<15} {:<7} {:<5} ", lamp.id, on_off, brightness);
        lamp.turn_on().await?;
    }

    Ok(())
}
