use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Error};
use badgemagic::{
    protocol::{Mode, PayloadBuffer, Speed, Style},
    usb_hid::Device as UsbDevice,
};
use badgemagic::embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRawLE},
    mono_font::{iso_8859_1::FONT_5X8, MonoTextStyle},
    pixelcolor::BinaryColor,
    text::Text,
    Drawable, Pixel,
};
use serde::Deserialize;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn set_text(text: &str, speed: u8, mode: &str) -> String {
    let speed: Speed = Speed::try_from(speed).unwrap_or(Speed::Fps2_8);
    let mode: Mode = Mode::try_from(mode).unwrap_or(Mode::Left);

    let result = write_text_payload(text, speed, mode);
    match result {
        Ok(_) => { "Success!".to_string() }
        Err(err) => { format!("Something went wrong: {}", err.backtrace()) }
    }
}

#[tauri::command]
fn list_devices() -> Result<Vec<String>, String> {
    let devices = UsbDevice::list_all();
    match devices {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_text, list_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, untagged)]
enum Content {
    Text { text: String },
    Bitstring { bitstring: String },
    BitmapBase64 { width: u32, bitmap_base64: String },
    BitmapFile { width: u32, bitmap_file: PathBuf },
    // TODO: implement png
    // PngFile { png_file: PathBuf },
}

fn write_text_payload(input_text: &str, speed: Speed, mode: Mode) -> Result<()> {
    let mut payload = PayloadBuffer::new();
    let style = Style::default().speed(speed).mode(mode);

    let text = Text::new(
        input_text,
        Point::new(0, 8),
        MonoTextStyle::new(&FONT_5X8, BinaryColor::On),
    );

    payload.add_message_drawable(style, &text);

    write_payload(payload)
}

fn write_payload(
    // transport: &TransportProtocol,
    payload: PayloadBuffer,
) -> Result<()> {
    UsbDevice::single()?.write(payload)
    // match transport {
    //     TransportProtocol::Usb => UsbDevice::single()?.write(payload),
    //     TransportProtocol::Ble => tokio::runtime::Builder::new_current_thread()
    //         .enable_all()
    //         .build()?
    //         .block_on(async { BleDevice::single().await?.write(payload).await }),
    // }
}
