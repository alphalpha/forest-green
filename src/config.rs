use crate::{font, locations, util};
use chrono::{DateTime, Duration, NaiveTime, TimeZone, Utc};
use confy;
use image::Rgb;
use imageproc::rect::Rect;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct RawConfig {
    pub images_path: String,
    pub roi: [i32; 4],
    pub font_path: String,
    pub font_size: f32,
    pub font_color: [u8; 3],
    pub location: String,
    pub start_date: [u32; 3],
    pub end_date: [u32; 3],
    pub duration: i64,
    pub night_times: [u32; 2],
    pub night_color: [u8; 3],
}

impl ::std::default::Default for RawConfig {
    fn default() -> Self {
        Self {
            images_path: "".into(),
            roi: [0, 0, 0, 0],
            font_path: "".into(),
            font_size: 0.0,
            font_color: [0, 0, 0],
            location: "".into(),
            start_date: [0, 0, 0],
            end_date: [0, 0, 0],
            duration: 0,
            night_times: [0, 0],
            night_color: [0, 0, 0],
        }
    }
}

pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub roi: Rect,
    pub font: font::Font,
    pub location: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub duration: Duration,
    pub night_start_time: NaiveTime,
    pub night_end_time: NaiveTime,
    pub night_color: Rgb<u8>,
}

impl Config {
    pub fn from(path: &Path) -> Result<Config, util::Error> {
        let raw_config: RawConfig = confy::load_path(path)?;
        let input_dir = Path::new(&raw_config.images_path).to_path_buf();
        if let Ok(metadata) = input_dir.metadata() {
            if !metadata.is_dir() {
                return Err(util::Error::Custom(String::from(
                    "Input path is not a directory",
                )));
            };
        }
        let output_dir = input_dir.join(Path::new("Output"));
        fs::create_dir(&output_dir)?;

        let location_map = locations::location_map()?;
        let location = location_map
            .get(&raw_config.location)
            .ok_or(util::Error::Custom(String::from(
                "Given location info is unknown",
            )))?
            .clone();

        let font = font::Font::new(
            Path::new(&raw_config.font_path),
            raw_config.font_size,
            Rgb(raw_config.font_color),
        );

        let night_start_time = NaiveTime::from_hms(raw_config.night_times[0], 0, 0);
        let night_end_time = NaiveTime::from_hms(raw_config.night_times[1], 0, 0);

        Ok(Config {
            input_path: input_dir,
            output_path: output_dir,
            roi: Rect::at(raw_config.roi[0], raw_config.roi[1])
                .of_size(raw_config.roi[2] as u32, raw_config.roi[3] as u32),
            font: font,
            location: location,
            start_date: Utc
                .ymd(
                    raw_config.start_date[0] as i32,
                    raw_config.start_date[1],
                    raw_config.start_date[2],
                )
                .and_hms(0, 0, 0),
            end_date: Utc
                .ymd(
                    raw_config.end_date[0] as i32,
                    raw_config.end_date[1],
                    raw_config.end_date[2],
                )
                .and_hms(23, 59, 59),
            duration: Duration::minutes(raw_config.duration),
            night_start_time,
            night_end_time,
            night_color: Rgb(raw_config.night_color),
        })
    }
}
