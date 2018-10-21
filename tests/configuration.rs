extern crate tcs3472;
extern crate embedded_hal_mock as hal;

mod common;
use common::{ setup, check_sent_data, Register, BitFlags };
use tcs3472::RgbCGain;

const ENABLE_CMD : u8 = BitFlags::CMD | Register::ENABLE;

#[test]
fn can_enable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON]);
}

#[test]
fn can_disable() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.disable().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, 0]);
}

#[test]
fn can_enable_rgbc() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.enable_rgbc().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON | BitFlags::RGBC_EN]);
}

#[test]
fn can_disable_rgbc() {
    let mut dev = setup(&[0]);
    dev.enable().unwrap();
    dev.enable_rgbc().unwrap();
    dev.disable_rgbc().unwrap();
    check_sent_data(dev, &[ENABLE_CMD, BitFlags::POWER_ON & !BitFlags::RGBC_EN]);
}

macro_rules! set_rgbc_gain_test {
    ($name:ident, $variant:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut dev = setup(&[0]);
            dev.set_rgbc_gain(RgbCGain::$variant).unwrap();
            check_sent_data(dev, &[BitFlags::CMD | Register::CONTROL, $expected]);
        }
    };
}

set_rgbc_gain_test!(can_set_rgbc_gain_1x,   _1x, 0);
set_rgbc_gain_test!(can_set_rgbc_gain_4x,   _4x, 1);
set_rgbc_gain_test!(can_set_rgbc_gain_16x, _16x, 2);
set_rgbc_gain_test!(can_set_rgbc_gain_60x, _60x, 3);