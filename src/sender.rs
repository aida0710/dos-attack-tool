use std::time::Instant;
use pcap::{Active, Capture};
use std::error::Error;
use std::sync::Arc;

use super::settings::{SendPacketSettings, SettingsLocator, SettingsPattern};
use super::packet_builder::build_packet;

pub fn packet_sender(cap: &mut Capture<Active>, pattern: SettingsPattern) -> Result<(), Box<dyn Error>> {
    let locator = SettingsLocator::new();
    let settings = locator.get_settings(&pattern)
        .ok_or_else(|| format!("設定パターンが見つかりません: {:?}", pattern))?;

    send_packets(cap, settings)
}

fn send_packets(cap: &mut Capture<Active>, settings: Arc<SendPacketSettings>) -> Result<(), Box<dyn Error>> {
    let ethernet_buffer = build_packet(&settings)?;

    println!("パケット送信を開始します...");
    let start_time = Instant::now();

    for i in 0..settings.packet_count {
        cap.sendpacket(ethernet_buffer.clone())?;
        if (i + 1) % 10000 == 0 {
            let elapsed_time = start_time.elapsed();
            println!("パケット {} / {} を送信しました (経過時間: {:.2} 秒)", i + 1, settings.packet_count, elapsed_time.as_secs_f64());
        }
        std::thread::sleep(settings.interval);
    }

    let elapsed_time = start_time.elapsed();
    println!("パケット送信が完了しました");
    println!("総送信パケット数: {}", settings.packet_count);
    println!("経過時間: {:.2} 秒", elapsed_time.as_secs_f64());

    Ok(())
}
