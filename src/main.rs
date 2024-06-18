use clap::Parser;
use std::{num::ParseIntError, time::Duration};

use rusb::GlobalContext;

fn get_device(vendor_id: u16, product_id: u16) -> rusb::Device<GlobalContext> {
    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();
        if device_desc.vendor_id() == vendor_id && device_desc.product_id() == product_id {
            // println!("Found device");
            return device;
        }
    }
    panic!("Device not found");
}
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Effects {
    STATIC = 0,
    BREATHE = 1,
    FADE = 2,
    GETTING_OFF = 3,
    LITTLE_STARS = 4,
    LASER = 5,
    WAVE = 6,
    NEON = 7,
    RAINDROP = 8,
    RIPPLE = 9,
    WAVE2 = 10,
    SWIRL = 11,
}
struct Firefly {
    _vendor_id: u16,
    _product_id: u16,
    device_handle: rusb::DeviceHandle<GlobalContext>,
    colors: Vec<String>,
}
impl Firefly {
    fn check_colors(colors: &Vec<String>) {
        assert_eq!(colors.len(), 7, "Exactly 7 colors are required");
    }
    pub fn new(colors: Vec<String>) -> Self {
        let vendor_id = 0x04d9;
        let product_id = 0xa1cd;
        let device = get_device(vendor_id, product_id);
        let device_handle = device.open().unwrap();
        device_handle.set_auto_detach_kernel_driver(true).unwrap();
        Self::check_colors(&colors);
        Firefly {
            colors,
            _vendor_id: vendor_id,
            _product_id: product_id,
            device_handle,
        }
    }
    pub fn set_colors(&mut self, colors: Vec<String>) {
        assert_eq!(colors.len(), 7, "Exactly 7 colors are required");
        self.colors = colors;
    }
    pub fn claim_interface(&self, interface: u8) {
        self.device_handle.claim_interface(interface).unwrap();
    }
    pub fn execute_header_request(&self) {
        let data = [0x30, 00, 00, 00, 00, 0x55, 0xaa, 00];

        self.device_handle
            .write_control(0x21, 0x09, 0x0300, 2, &data, Duration::new(1, 0))
            .unwrap();
    }
    pub fn execute_color_request(&self) {
        // let data= "05242605242601293907193d10280a1c213f220e3b007600c042e308000000000000000078128b02b8bd6400a443c667000000005a00000018bf6400e0d9d400";
        // let buf = decode_hex(data).unwrap();
        let mut buf = encode_colors(self.colors.clone());
        buf.resize(64, 0);
        self.device_handle
            .write_interrupt(0x04, &buf, Duration::new(1, 0))
            .unwrap();
    }
    pub fn execute_effects_request(&self, effect: Effects, color_idx: usize) {
        assert!(color_idx <= 7, "Color index out of bounds, got {}", color_idx);
        let data = [
            0x08,
            effect as u8,
            0x3f,
            0x01,
            00,
            color_idx as u8,
            0xc4,
            0x3b,
        ];
        self.device_handle
            .write_control(0x21, 0x09, 0x0300, 2, &data, Duration::new(1, 0))
            .unwrap();
    }
    pub fn release_interface(&self, interface: u8) {
        self.device_handle.release_interface(interface).unwrap();
    }
}

fn encode_color(color: impl Into<String>) -> Vec<u8> {
    let mut color = color.into();
    if color[0..1] == *"#" {
        color = (color[1..]).to_string();
    }
    if color.len() != 6 {
        panic!("Invalid color");
    }
    let color = decode_hex(&color).unwrap();
    assert_eq!(color.len(), 3);
    //divide by 4 because that the spec of the keyboard
    let color = color.iter().map(|x| x / 4).collect();
    return color;
}
fn encode_colors(colors: Vec<String>) -> Vec<u8> {
    colors
        .iter()
        .map(|color| encode_color(color))
        .flatten()
        .collect()
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long,
        help="
            array of exactly 7 colors to be used for lighting. 
            If unspecified, default colors are used.
            Colors must be valid hex colors. eg: #ff0000. Delimited by comma",
        num_args=0..,value_delimiter=',')]
    colors: Vec<String>,
    #[arg(short, long)]
    effect: Effects,
    #[arg(
        long = "ci",
        help = "Index of the color to use for the effect. 0-6 for index of the colors. 7 for loop",
        default_value = "7"
    )]
    color_idx: usize,
}
fn main() {
    let args = Args::parse();
    let mut colors = args.colors;
    let default_colors = vec![
        "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#00ffff", "#ff00ff", "#ffffff",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    println!("{:?}", colors);
    if colors.len() == 0 {
        colors = default_colors;
    } else if colors.len() != 7 {
        panic!("Exactly 7 colors are required");
    }

    if args.color_idx > 7 {
        panic!("Color index out of bounds");
    }
    let firefly = Firefly::new(colors);
    // claim interface 2
    firefly.claim_interface(2);

    firefly.execute_header_request();
    firefly.execute_color_request();
    firefly.execute_effects_request(args.effect, args.color_idx);
    // release interface
    firefly.release_interface(2);
}
#[test]
fn test_encode_color() {
    let color = encode_color("#ffffff");
    assert_eq!(color, vec![63, 63, 63]);
}
