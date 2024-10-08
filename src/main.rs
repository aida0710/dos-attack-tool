use crate::select_device::select_device;
use dotenv::dotenv;
use pcap::{Active, Capture, Device};
mod select_device;
mod packet_builder;
mod sender;
mod settings;

use pnet::packet::MutablePacket;
use std::error::Error;
use crate::sender::packet_sender;
use crate::settings::SettingsPattern;

fn main() -> Result<(), Box<dyn Error>> {
    // .envファイルを読み込む
    dotenv().ok();
    let (mut cap, device): (Capture<Active>, Device) = select_device()?;
    println!("デバイスの選択に成功しました: {}", device.name);

    // デフォルトパターンを使用
    packet_sender(&mut cap, SettingsPattern::Attack)?;

    Ok(())
}