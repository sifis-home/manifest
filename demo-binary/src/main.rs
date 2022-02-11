use sifis_api::{Context, Light, Percentage, Rgb};

pub fn turn_light_on() -> i32 {
    0
}

fn main() {
    let context = Context::default();
    let mut devices = context.find_all::<Light>().unwrap();
    for device in &mut devices {
        let brightness = Percentage::new(30);
        let color = Rgb::new(255, 0, 0);
        device.turn_light_on(brightness, color).unwrap();
    }
}
