pub mod config;
mod font;
mod locations;
mod util;

pub use crate::config::Config;
pub use crate::util::Error;
use chrono::{DateTime, TimeZone, Utc};
use image::{GenericImage, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::Point;
use std::fs;
use std::path::{Path, PathBuf};

fn crop_image(image: &mut RgbImage, rect: &Rect) -> Result<RgbImage, util::Error> {
    Ok(image
        .sub_image(
            rect.left() as u32,
            rect.top() as u32,
            rect.width(),
            rect.height(),
        )
        .to_image())
}

fn mean_color(image: &RgbImage) -> Result<Rgb<u8>, util::Error> {
    let num_pixels = image.width() * image.height();
    let color: Vec<u8> = image
        .pixels()
        .fold(vec![0u32, 0u32, 0u32], |mut acc, pixel| {
            for i in 0..acc.len() {
                acc[i] += pixel[i] as u32;
            }
            acc
        })
        .iter()
        .map(|c| (c / num_pixels) as u8)
        .collect();

    Ok(Rgb([color[0], color[1], color[2]]))
}

fn image_paths(dir: &Path) -> Result<Vec<PathBuf>, util::Error> {
    let mut paths: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some())
        .collect();
    paths.sort();
    Ok(paths)
}

fn parse_date(name: &str) -> Result<(DateTime<Utc>, String), util::Error> {
    let mut parts: Vec<&str> = name.splitn(5, '_').collect();
    match parts.len() {
        5 => {
            let parts = parts.split_off(2);
            let (year, rest) = parts[1].split_at(4);
            let (month, day) = rest.split_at(2);
            let date = day.to_string() + "." + month + "." + year;
            let (hour, rest) = parts[2].split_at(2);
            let (minutes, seconds) = rest.split_at(2);
            let time = hour.to_string() + ":" + minutes + ":" + seconds;

            let utc = Utc
                .ymd(year.parse()?, month.parse()?, day.parse()?)
                .and_hms(hour.parse()?, minutes.parse()?, seconds.parse()?);
            Ok((utc, String::from(date + ", " + &time)))
        }
        _ => Err(util::Error::Custom(String::from(
            "File: \"".to_string() + name + "\" has wrong name format",
        ))),
    }
}

fn output_file_path(
    target_dir: &Path,
    source_file: &Path,
    utc: &DateTime<Utc>,
) -> Result<PathBuf, util::Error> {
    let mut stem = source_file
        .file_stem()
        .ok_or_else(|| util::Error::Custom(String::from("Could not extract the file name")))?
        .to_os_string();
    stem.push("_green");
    stem.push(utc.to_string());
    let path =
        target_dir
            .join(stem)
            .with_extension(source_file.extension().ok_or_else(|| {
                util::Error::Custom(String::from("Could not obtain the file extension"))
            })?);
    println!("Save {:?}", path);
    Ok(path)
}

fn draw_citing(image: &mut RgbImage, config: &Config, position: &Point<u32>, text: &str) {
    if let Some(width) = font::text_width(config.font.scale, &config.font.font, text) {
        let height = config.font.scale.y as u32;
        draw_filled_rect_mut(
            image,
            Rect::at(position.x as i32, position.y as i32).of_size(width, height),
            config.font.background_color,
        );
        draw_text_mut(
            image,
            config.font.color,
            position.x,
            position.y,
            config.font.scale,
            &config.font.font,
            text,
        );
    }
}

fn generate_image(
    config: &Config,
    date: &String,
    file_path: &Path,
) -> Result<RgbImage, util::Error> {
    let mut in_image = image::open(&file_path)?.to_rgb8();
    let color = crop_image(&mut in_image, &config.roi).and_then(|i| mean_color(&i))?;
    let dimensions = in_image.dimensions();
    let mut image = image::ImageBuffer::new(2 * dimensions.0, dimensions.1);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if x < dimensions.0 {
            *pixel = color;
        } else {
            *pixel = *in_image.get_pixel(x - dimensions.0, y);
        }
    }

    let mut position = Point {
        x: config.font.pos.0,
        y: config.font.pos.1,
    };
    let location_date = config.location.clone() + ", " + &date;
    draw_citing(&mut image, &config, &position, &location_date.as_str());

    let font_height = config.font.scale.y as u32;
    position.y = config.font.pos.1 + font_height;
    let title = "Average colour of forest activity";
    draw_citing(&mut image, &config, &position, title);

    position.y = config.font.pos.1 + 2 * font_height;
    let color_string = format!("{:?}", color);
    draw_citing(&mut image, &config, &position, &color_string.as_str());
    Ok(image)
}

fn date_from_file_name(file_path: &Path) -> Result<(DateTime<Utc>, String), util::Error> {
    file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| util::Error::Custom(String::from("Cannot obtain file name")))
        .and_then(|n| parse_date(n))
}

pub fn run(config: Config) -> Result<(), util::Error> {
    let input_paths = image_paths(&config.input_path)?;
    let mut current_date = config.start_date.clone();
    let mut file_iter = input_paths.iter();
    let mut prev_file = file_iter.next().unwrap();
    let (mut prev_utc, mut prev_date) = date_from_file_name(&prev_file)?;

    while let Some(file) = file_iter.next() {
        let (utc, date) = date_from_file_name(&file)?;
        if utc > config.end_date {
            break;
        }
        // Skip outdated images
        if prev_utc < current_date {
            prev_utc = utc;
            prev_date = date;
            prev_file = file;
            continue;
        }

        while current_date < utc {
            let image = generate_image(&config, &prev_date, &prev_file)?;
            output_file_path(&config.output_path, &prev_file, &current_date)
                .and_then(|path| image.save(path).map_err(|e| util::Error::Image(e)))?;
            current_date = current_date + config.duration;
        }
        prev_utc = utc;
        prev_date = date;
        prev_file = file;
    }

    while current_date <= config.end_date {
        let image = generate_image(&config, &prev_date, &prev_file)?;
        output_file_path(&config.output_path, &prev_file, &current_date)
            .and_then(|path| image.save(path).map_err(|e| util::Error::Image(e)))?;
        current_date = current_date + config.duration;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mean_color_works() {
        use image::{Rgb, RgbImage};
        let expected = Rgb([42 as u8, 21 as u8, 84 as u8]);
        let mut image = RgbImage::new(10, 10);
        for p in image.pixels_mut() {
            *p = expected;
        }
        let actual = mean_color(&mut image).unwrap();
        assert_eq!(expected, actual);
    }
}
