extern crate image;

use image::GenericImage;
use image::Pixel;

fn main() {
    let mut img_pad = {
        let img = image::open("assets/font.png").unwrap();

        let mut img = img.to_rgba();

        // calculate_and_print_histogram(&img);

        // Pad glyphs.
        let mut img_pad = image::RgbaImage::new(img.width() * 2, img.height() * 2);

        for r in 0..16 {
            for c in 0..16 {
                let sub = img.sub_image(c * 64, r * 64, 64, 64);
                img_pad.copy_from(&sub, c * 128 + 32, r * 128 + 32);
            }
        }

        img_pad.save("assets/font-padded.png").unwrap();

        img_pad
    };

    // Calculate sdf
    let mut sdf = image::RgbaImage::new(img_pad.width(), img_pad.height());

    const R: i32 = 5;
    const R_SQ: i32 = R * R;

    for (x, y, pixel) in sdf.enumerate_pixels_mut() {
        let outside: bool = unsafe { img_pad.unsafe_get_pixel(x, y)[3] < 128 };
        let x = x as i32;
        let y = y as i32;

        let mut closest_d_sq: i32 = R_SQ;

        // x high, exclusive maximum value for x.
        let x_h = img_pad.width() as i32;
        // y high, exclusive maximum value for y.
        let y_h = img_pad.height() as i32;

        // Define bounding box of circle lying within the source image.
        let dx_l: i32 = if x >= R { -R } else { -x };
        let dx_h: i32 = if x + R <= x_h { R } else { x_h - x };
        let dy_l: i32 = if y >= R { -R } else { -y };
        let dy_h: i32 = if y + R <= y_h { R } else { y_h - y };

        for dy in dy_l..dy_h {
            for dx in dx_l..dx_h {
                let d_sq = dx * dx + dy * dy;

                if d_sq > R_SQ {
                    // Pixel does not lie within search radius.
                    continue;
                }

                let other_outside: bool =
                    unsafe { img_pad.unsafe_get_pixel((x + dx) as u32, (y + dy) as u32)[3] < 128 };

                if outside != other_outside {
                    if d_sq < closest_d_sq {
                        closest_d_sq = d_sq;
                    }
                }
            }
        }

        #[inline]
        fn map_lin(x: f32, x0: f32, x1: f32, y0: f32, y1: f32) -> f32 {
            (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0)
        }

        let d: f32 = (closest_d_sq as f32).sqrt();
        assert!(d >= 1.0);
        assert!(d <= R as f32);
        let a: f32 = if outside {
            let a = map_lin(d, R as f32, 0.0, 0.0, 127.0);
            assert!(a >= 0.0);
            assert!(a <= 127.0);
            a
        } else {
            let a = map_lin(d, 0.0, R as f32, 128.0, 255.0);
            assert!(a >= 128.0);
            assert!(a <= 255.0);
            a
        };

        *pixel = image::Rgba::from_channels(
            (x % 256) as u8,
            (y % 256) as u8,
            ((x + y) % 256) as u8,
            a.floor() as u8,
        );
    }

    sdf.save("assets/font-padded-sdf.png").unwrap();
}

pub fn calculate_and_print_histogram<
    I: image::GenericImage<Pixel = P>,
    P: image::Pixel<Subpixel = u8>,
>(
    img: &I,
) {
    let c = I::Pixel::channel_count() as usize;

    let mut counts: Vec<usize> = vec![0; c * 256];

    for (_, _, pixel) in img.pixels() {
        for (i, &component) in pixel.channels().iter().enumerate() {
            counts[i * 256 + component as usize] += 1;
        }
    }

    for i in 0..c {
        println!("{} {:#?}", i, &counts[i * 256..(i + 1) * 256]);
    }
}
