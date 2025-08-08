use anyhow::Result;
use badgemagic::embedded_graphics::{
    geometry::Point,
    mono_font::{iso_8859_1::FONT_5X8, iso_8859_1::FONT_6X9, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    text::Text,
};
use badgemagic::{
    protocol::{Mode, PayloadBuffer, Speed, Style},
    usb_hid::Device as UsbDevice,
};
use serde::Deserialize;
use std::num::TryFromIntError;
use std::path::PathBuf;
use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn set_text(text: &str, speed: u8, animation: &str, effects: Vec<&str>, font_size: u8) -> String {
    let speed: Speed = Speed::try_from(speed).unwrap_or(Speed::Fps2_8);
    let mode: Mode = Mode::try_from(animation).unwrap_or(Mode::Left);

    let flash: bool = effects.contains(&"flashing");
    let border: bool = effects.contains(&"border");
    let invert: bool = effects.contains(&"inverted");

    let size: FontSize = FontSize::try_from(font_size).unwrap_or(FontSize::Size6x9);

    let mut payload = PayloadBuffer::new();

    payload = write_text_to_payload(payload, text, speed, mode, flash, border, invert, size);

    match write_payload(payload) {
        Ok(_) => "Success!".to_string(),
        Err(err) => {
            format!("Something went wrong: {}", err.backtrace())
        }
    }
}

#[tauri::command]
fn set_messages(messages: Vec<Message>) -> String {
    let mut payload = PayloadBuffer::new();

    for message in messages {
        let speed: Speed = Speed::try_from(message.speed).unwrap_or(Speed::Fps2_8);
        let mode: Mode = Mode::try_from(message.animation.as_str()).unwrap_or(Mode::Left);

        let flash: bool = message.effects.contains(&"flashing".to_string());
        let border: bool = message.effects.contains(&"border".to_string());
        let invert: bool = message.effects.contains(&"inverted".to_string());

        let size: FontSize = FontSize::try_from(message.font_size).unwrap_or(FontSize::Size6x9);

        payload = write_text_to_payload(
            payload,
            message.text.as_str(),
            speed,
            mode,
            flash,
            border,
            invert,
            size,
        );
    }

    match write_payload(payload) {
        Ok(_) => "Success!".to_string(),
        Err(err) => {
            format!("Something went wrong: {}", err.backtrace())
        }
    }
}

#[tauri::command]
fn list_devices() -> Result<Vec<String>, String> {
    let devices = UsbDevice::list_all();
    match devices {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE messages (id INTEGER PRIMARY KEY AUTOINCREMENT, content_id INTEGER, type TEXT);\
                  CREATE TABLE text_messages (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, speed INTEGER, animation TEXT, effects TEXT, font_size INTEGER)",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:messages.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_text,
            set_messages,
            list_devices
        ])
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

fn write_text_to_payload(
    mut payload: PayloadBuffer,
    input_text: &str,
    speed: Speed,
    mode: Mode,
    flashing: bool,
    border: bool,
    inverted: bool,
    font_size: FontSize,
) -> PayloadBuffer {
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

    let text = Text::new(input_text, position, font);

    payload.add_message_drawable(style, &text);

    payload
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deserialize)]
struct Message {
    id: u32,
    text: String,
    speed: u8,
    animation: String,
    effects: Vec<String>,
    font_size: u8,
    m_type: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum FontSize {
    Size5x8,
    #[default]
    Size6x9,
}

impl TryFrom<u8> for FontSize {
    type Error = TryFromIntError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        Ok(match value {
            8 => Self::Size5x8,
            9 => Self::Size6x9,
            _ => return Err(u8::try_from(-1).unwrap_err()),
        })
    }
}

impl<'a> From<FontSize> for MonoFont<'a> {
    fn from(value: FontSize) -> Self {
        match value {
            FontSize::Size5x8 => FONT_5X8,
            FontSize::Size6x9 => FONT_6X9,
        }
    }
}

impl From<FontSize> for Point {
    fn from(value: FontSize) -> Self {
        match value {
            FontSize::Size5x8 => Point::new(0, 8),
            FontSize::Size6x9 => Point::new(0, 7),
        }
    }
}
