use embedded_hal::digital::v2::OutputPin;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use max7219::connectors::PinConnector;
use max7219::MAX7219;
use std::time::Duration;

pub mod error;
pub mod mapping;
pub mod store;

#[macro_use]
extern crate lazy_static;

struct LHandle(LineHandle);

impl OutputPin for LHandle {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_value(0).map_err(|_| ())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_value(1).map_err(|_| ())
    }
}

pub fn dispatch_data(data: &[[u8; 8]]) -> anyhow::Result<(), error::Err> {
    let mut chip = Chip::new("/dev/gpiochip0")?;

    let data_pin = chip
        .get_line(24)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;
    let cs_pin = chip
        .get_line(23)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;
    let clk_pin = chip
        .get_line(18)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;

    let data_pin = LHandle(data_pin);
    let cs_pin = LHandle(cs_pin);
    let clk_pin = LHandle(clk_pin);

    let mut max7219 = MAX7219::from_pins(4, data_pin, cs_pin, clk_pin).unwrap();

    max7219.power_on().unwrap();
    clear(&mut max7219);
    loading(&mut max7219);

    loop {
        data.iter().enumerate().for_each(|(i, m)| {
            let _ = max7219.write_raw(i, m);
        });
        std::thread::sleep(Duration::from_millis(1000));
    }
}

pub fn start() -> anyhow::Result<(), error::Err> {
    println!();
    println!("请输入对应的引脚编号: <data> <cs> <clk>");
    println!();

    let args: Vec<String> = std::env::args().collect();

    let data = args[1].parse::<u32>()?;
    let cs = args[2].parse::<u32>()?;
    let clk = args[3].parse::<u32>()?;

    println!("data={}, cs={}, clk={}", data, cs, clk);

    let mut chip = Chip::new("/dev/gpiochip0")?;

    let data_pin = chip
        .get_line(data)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;
    let cs_pin = chip
        .get_line(cs)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;
    let clk_pin = chip
        .get_line(clk)?
        .request(LineRequestFlags::OUTPUT, 0, "")?;

    let data_pin = LHandle(data_pin);
    let cs_pin = LHandle(cs_pin);
    let clk_pin = LHandle(clk_pin);

    let mut max7219 = MAX7219::from_pins(4, data_pin, cs_pin, clk_pin).unwrap();

    max7219.power_on().unwrap();
    loading_setting(&mut max7219);
    loading(&mut max7219);
    clear(&mut max7219);

    loop {
        load_time(&mut max7219);
    }
}

/// 设置
fn loading_setting(max7219: &mut MAX7219<PinConnector<LHandle, LHandle, LHandle>>) {
    (0..4).for_each(|v| {
        // 设置亮度
        max7219.set_intensity(v, 0x03).unwrap();
    });
}

/// 加载动画
fn loading(max7219: &mut MAX7219<PinConnector<LHandle, LHandle, LHandle>>) {
    let _ = max7219.write_raw(0, &mapping::UPPER_C);
    let _ = max7219.write_raw(1, &mapping::UPPER_L);
    let _ = max7219.write_raw(2, &mapping::UPPER_K);
    let _ = max7219.write_raw(3, &mapping::EXCLAMATION_MARK);
    std::thread::sleep(Duration::from_millis(3000));
}

/// 清空屏幕
fn clear(max7219: &mut MAX7219<PinConnector<LHandle, LHandle, LHandle>>) {
    (0..4).for_each(|v| {
        let _ = max7219.clear_display(v);
    });
}

/// 加载时间
fn load_time(max7219: &mut MAX7219<PinConnector<LHandle, LHandle, LHandle>>) {
    let now_time = chrono::Local::now().format("%H:%M:%S").to_string();

    let mut data = mapping::encode_string(&now_time);
    let data = mapping::merge_time(&mut data);
    data.iter().enumerate().for_each(|(i, m)| {
        let _ = max7219.write_raw(i, m);
    });
}
