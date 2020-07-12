extern crate raster;
use std::cmp;

pub fn gray_scale(img: raster::Image) -> raster::Image {
    let mut image = img;
    
    for w in 0..image.width {
        for h in 0..image.height {
            let pixel = image.get_pixel(w, h).unwrap();
            let gray = ((pixel.r as f64) + (pixel.g as f64) + (pixel.b as f64))/3.0;
            let _ = image.set_pixel(w, h, raster::Color::rgba(gray as u8, gray as u8, gray as u8,pixel.a));
        }
    }
    image
}

pub fn brightness(img: raster::Image, factor: f32) -> raster::Image {
    let mut image = img;

    for w in 0..image.width {
        for h in 0..image.height {
            let pixel = image.get_pixel(w, h).unwrap();
            let r = cmp::max(0, cmp::min(255, (pixel.r as f32 * factor) as i32));
            let g = cmp::max(0, cmp::min(255, (pixel.g as f32 * factor) as i32));
            let b = cmp::max(0, cmp::min(255, (pixel.b as f32 * factor) as i32));
            let _ = image.set_pixel(w, h, raster::Color::rgba(r as u8, g as u8, b as u8,pixel.a));
        }
    }
    image
}

pub fn gamma_correction(img: raster::Image, gamma: f32) -> raster::Image {
    let mut image = img;

    if gamma < 0.01 || gamma > 9.99 {
        println!("Invalid gamma param");
        image
    }else {
        let gamma = 1.0 / gamma;
        for h in 0..image.height {
            for w in 0..image.width {
                let pixel = image.get_pixel(w, h).unwrap();
                let r = 255.0 * (pixel.r as f32 / 255.0).powf(gamma);
                let g = 255.0 * (pixel.g as f32 / 255.0).powf(gamma);
                let b = 255.0 * (pixel.b as f32 / 255.0).powf(gamma);
                let _ = image.set_pixel(w, h, raster::Color::rgba(r as u8, g as u8, b as u8,pixel.a));
            }
        }
        image
    }
}

pub fn mirror_effect(img: raster::Image) -> raster::Image {
    let input_image = img.clone();
    let mut image = img.clone();

    for w in 0..image.width/2 {
        for h in 0..image.height {
            let width = input_image.width - w - 1;
            let pixel_a = input_image.get_pixel(width, h).unwrap();
            let pixel_b = input_image.get_pixel(w, h).unwrap();
            let _ = image.set_pixel(w, h, raster::Color::rgba(pixel_a.r, pixel_a.g, pixel_a.b ,pixel_a.a));
            let _ = image.set_pixel(width, h, raster::Color::rgba(pixel_b.r, pixel_b.g, pixel_b.b ,pixel_b.a));
        }
    }
    image
}

pub fn constrast(img: raster::Image) -> raster::Image {
    let mut imin = 255;
    let mut imax = 0;
    let mut image = img;

    for w in 0..image.width {
        for h in 0..image.height {
            let pixel = image.get_pixel(w, h).unwrap();
            let i = ((pixel.r as f64) + (pixel.g as f64) + (pixel.b as f64))/3.0;
            imin = cmp::min(imin, i as i32);
            imax = cmp::max(imax, i as i32);
        }
    }

    for w in 0..image.width {
        for h in 0..image.height {
            let pixel =  image.get_pixel(w, h).unwrap();
            let i = ((pixel.r as f64) + (pixel.g as f64) + (pixel.b as f64))/3.0;
            let lum = 255.0 * (i - imin as f64) / (imax - imin) as f64;
            let r = pixel.r as f64 * lum / i;
            let g = pixel.g as f64 * lum / i;
            let b = pixel.b as f64 * lum / i;
            let _ = image.set_pixel(w, h, raster::Color::rgba(r as u8, g as u8, b as u8, pixel.a));
        }
    }
    image 
}

pub fn negative(img: raster::Image) -> raster::Image {
    let mut image = img.clone();

    for w in 0..image.width {
        for h in 0..image.height {
            let pixel = image.get_pixel(w, h).unwrap();
            let r = 255 - pixel.r;
            let g = 255 - pixel.g;
            let b = 255 - pixel.b;
            let _ = image.set_pixel(w, h, raster::Color::rgba(r, g, b,pixel.a));
        }
    }
    image
}
