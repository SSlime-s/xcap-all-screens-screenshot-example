use image::{DynamicImage, RgbaImage};
use xcap::image;

struct MonitorInfo {
    xy: (i32, i32),
    wh: (u32, u32),
    image: RgbaImage,
}

fn main() {
    let monitors = xcap::Monitor::all().unwrap();
    let monitor_infos = monitors
        .into_iter()
        .map(|monitor| {
            let image = monitor.capture_image().unwrap();
            MonitorInfo {
                xy: (monitor.x(), monitor.y()),
                wh: (monitor.width(), monitor.height()),
                image,
            }
        })
        .collect::<Vec<_>>();
    let merged_image = merge_images(&monitor_infos);
    merged_image.save("screenshot.png").unwrap();
}

struct VirtualImageSize {
    xy: (i32, i32),
    wh: (u32, u32),
}

fn get_whole_size(monitor_infos: &[MonitorInfo]) -> VirtualImageSize {
    let x = monitor_infos.iter().map(|info| info.xy.0).min().unwrap();
    let y = monitor_infos.iter().map(|info| info.xy.1).min().unwrap();
    let w = monitor_infos
        .iter()
        .map(|info| info.xy.0 + info.wh.0 as i32)
        .max()
        .unwrap()
        - x;
    let h = monitor_infos
        .iter()
        .map(|info| info.xy.1 + info.wh.1 as i32)
        .max()
        .unwrap()
        - y;
    VirtualImageSize {
        xy: (x, y),
        wh: (w as u32, h as u32),
    }
}
fn merge_images(monitor_infos: &[MonitorInfo]) -> DynamicImage {
    let VirtualImageSize { xy, wh } = get_whole_size(monitor_infos);
    let mut image = RgbaImage::new(wh.0, wh.1);
    for info in monitor_infos {
        image::imageops::overlay(
            &mut image,
            &info.image,
            (info.xy.0 - xy.0) as i64,
            (info.xy.1 - xy.1) as i64,
        );
    }
    DynamicImage::ImageRgba8(image)
}
