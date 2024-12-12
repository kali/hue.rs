use crate::CommandLight;
use regex::Regex;

pub fn parse_command(args: Vec<String>) -> CommandLight {
    let re_triplet = Regex::new("([0-9]{0,3}):([0-9]{0,5}):([0-9]{0,3})").unwrap();
    let re_mired = Regex::new("([0-9]{0,4})MK:([0-9]{0,5})").unwrap();
    let re_kelvin = Regex::new("([0-9]{4,4})K:([0-9]{0,5})").unwrap();
    let re_xy = Regex::new("(0\\.[0-9]+),(0\\.[0-9]+)(:([0-9]{0,5}))?").unwrap();
    let re_rrggbb = Regex::new("([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();

    let command = &args[3];
    let mut parsed = match &command[..] {
        "on" => CommandLight::default().on(),
        "off" => CommandLight::default().off(),
        _ if re_triplet.is_match(command) => {
            let caps = re_triplet.captures(command).unwrap();
            let mut command = CommandLight::default().on();
            command.bri = caps.get(1).and_then(|s| s.as_str().parse::<u8>().ok());
            command.hue = caps.get(2).and_then(|s| s.as_str().parse::<u16>().ok());
            command.sat = caps.get(3).and_then(|s| s.as_str().parse::<u8>().ok());
            command
        }
        _ if re_mired.is_match(command) => {
            let caps = re_mired.captures(command).unwrap();
            let mut command = CommandLight::default().on();
            command.ct = caps.get(1).and_then(|s| s.as_str().parse::<u16>().ok());
            command.bri = caps.get(2).and_then(|s| s.as_str().parse::<u8>().ok());
            command.sat = Some(254);
            command
        }
        _ if re_kelvin.is_match(command) => {
            let caps = re_kelvin.captures(command).unwrap();
            let mut command = CommandLight::default().on();
            command.ct = caps.get(1).and_then(|s| {
                s.as_str()
                    .parse::<u32>()
                    .ok()
                    .map(|k| (1000000u32 / k) as u16)
            });
            command.bri = caps.get(2).and_then(|s| s.as_str().parse::<u8>().ok());
            command.sat = Some(254);
            command
        }
        _ if re_rrggbb.is_match(command) => {
            let caps = re_rrggbb.captures(command).unwrap();
            let mut command = CommandLight::default().on();
            let rgb: Vec<u8> = [caps.get(1), caps.get(2), caps.get(3)]
                .iter()
                .map(|s| u8::from_str_radix(s.unwrap().as_str(), 16).unwrap())
                .collect();
            let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
            println!("{:?}", hsv);
            command.hue = Some((hsv.0 * 65535f64) as u16);
            command.sat = Some((hsv.1 * 255f64) as u8);
            command.bri = Some((hsv.2 * 255f64) as u8);
            command
        }
        _ if re_xy.is_match(command) => {
            let caps = re_xy.captures(command).unwrap();
            dbg!(&caps);
            let mut command = CommandLight::default().on();
            let x = caps.get(1).unwrap().as_str().parse::<f32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<f32>().unwrap();
            command.xy = Some((x, y));
            if let Some(bri) = caps.get(4) {
                command.bri = Some(bri.as_str().parse::<u8>().unwrap());
            }
            command
        }
        _ => panic!("can not understand command {:?}", command),
    };
    if args.len() == 5 {
        parsed.transitiontime = args[4].parse::<u16>().ok();
    }
    parsed
}

fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255f64;
    let g = g as f64 / 255f64;
    let b = b as f64 / 255f64;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    if max == min {
        (0f64, 0f64, max)
    } else {
        let d = max - min;
        let s = d / max;
        let h = if max == r {
            (g - b) / d + (if g < b { 6f64 } else { 0f64 })
        } else if max == g {
            (b - r) / d + 2f64
        } else {
            (r - g) / d + 4f64
        };
        (h / 6f64, s, max)
    }
}
