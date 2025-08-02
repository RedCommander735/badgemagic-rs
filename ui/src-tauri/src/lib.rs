use std::fs;
use std::num::TryFromIntError;
use std::path::PathBuf;
use anyhow::{Result, Error};
use badgemagic::{
    protocol::{Mode, PayloadBuffer, Speed, Style},
    usb_hid::Device as UsbDevice,
};
use badgemagic::embedded_graphics::{
    geometry::Point,
    image::{Image, ImageRawLE},
    mono_font::{iso_8859_1::FONT_5X8, iso_8859_1::FONT_6X9, MonoTextStyle, MonoFont},
    pixelcolor::BinaryColor,
    text::{Text},
    Drawable, Pixel,
};
use serde::Deserialize;
#[tauri::command]
fn set_text(text: &str, speed: u8, animation: &str, effects: Vec<&str>, font_size: u8) -> String {
    let speed: Speed = Speed::try_from(speed).unwrap_or(Speed::Fps2_8);
    let mode: Mode = Mode::try_from(animation).unwrap_or(Mode::Left);

    let flash: bool = effects.contains(&"flashing");
    let border: bool = effects.contains(&"border");
    let invert: bool = effects.contains(&"inverted");

    let size: FontSize = FontSize::try_from(font_size).unwrap_or(FontSize::SIZE_6x9);

    let result = write_text_payload(text, speed, mode, flash, border, invert, size);
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

fn write_text_payload(input_text: &str, speed: Speed, mode: Mode, flashing: bool, border: bool, inverted: bool, font_size: FontSize) -> Result<()> {
    let mut payload = PayloadBuffer::new();
    let mut style = Style::default().speed(speed).mode(mode);

    if flashing {
        style = style.blink()
    }

    if border {
        style = style.border()
    }

    let font_s = MonoFont::from(font_size);
    let position = Point::from(font_size);

    let bg_color = BinaryColor::from(inverted);

    let mut font = MonoTextStyle::new(&font_s, bg_color.invert());
    font.background_color = Some(bg_color);

    let text = Text::new(
        input_text,
        position,
        font,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum FontSize {
    SIZE_5x8,
    #[default]
    SIZE_6x9,
}

impl TryFrom<u8> for FontSize {
    type Error = TryFromIntError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            8 => Self::SIZE_5x8,
            9 => Self::SIZE_6x9,
            _ => return Err(u8::try_from(-1).unwrap_err()),
        })
    }
}

impl<'a> From<FontSize> for MonoFont<'a> {
    fn from(value: FontSize) -> Self {
        match value {
            FontSize::SIZE_5x8 => FONT_5X8,
            FontSize::SIZE_6x9 => FONT_6X9,
        }
    }
}

impl From<FontSize> for Point {
    fn from(value: FontSize) -> Self {
        match value {
            FontSize::SIZE_5x8 => Point::new(0, 8),
            FontSize::SIZE_6x9 => Point::new(0, 7),
        }
    }
}