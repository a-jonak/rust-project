extern crate raster;
use std::collections::HashMap;
use std::process::Command;

use methods;

pub fn interface(arguments: Vec<String>) {
    let mut methods_arr = HashMap::new();
    methods_arr.insert("gray_scale".to_string(), "Grayscale:

    Change color of the selected photo into gray.

---------------------------------------------------
If you want to use this method enter: 
 'cargo run gray_scale 0 /file_path/ /path_you_want_your_image_to_be_saved/'
 ");
    methods_arr.insert("brightness".to_string(), "Brightness: 
    
    Make the selected photo brighter or darker.

---------------------------------------------------
If you want to use this method enter: 
 'cargo run brightness /how_brighter_or_darker/ /file_path/ /path_you_want_your_image_to_be_saved/'
");
    methods_arr.insert("gamma_correction".to_string(), "Gamma correction: 
    
    Brighten dark places or darken those too light in the image.
    If you choose y < 1 dark places will be more bright
    or if you want more bright places be more dark choose y > 1 

---------------------------------------------------
If you want to use this method enter: 
 'cargo run gamma_correction /y/ /file_path/ /path_you_want_your_image_to_be_saved/'
");
    methods_arr.insert("mirror_effect".to_string(),"Mirror effect: 
    
    Creates a mirror image of the selected photo.

---------------------------------------------------
If you want to use this method enter: 
 'cargo run mirror_effect /file_path/ /path_you_want_your_image_to_be_saved/'
");
    methods_arr.insert("contrast".to_string(), "Contrast: 
    
    Makes the details of the photo clearer.

---------------------------------------------------
If you want to use this method enter: 
 'cargo run contrast /file_path/ /path_you_want_your_image_to_be_saved/'
");
    methods_arr.insert("negative".to_string(), "Negative: 
    
    Reverse colors of selected image.

---------------------------------------------------
If you want to use this method enter: 
 'cargo run negative /file_path/ /path_you_want_your_image_to_be_saved/'
");
    
    if arguments.len() > 1 {
        let command = arguments[1].clone();
        if command == "methods" {
            println!("
You can modify the selected photo using methods:
    *gray_scale
    *brightness
    *gamma_correction
    *mirror_effect
    *contrast
    *negative
Please type 'cargo run details /method_name/' to get some description.")
        }

        if command == "details" {
            if arguments.len() < 3 {
                println!("Please select method")
            }else {
                let method = arguments[2].clone();
                println!("{}", methods_arr[&method]);
            }
        }

        if methods_arr.contains_key(&command) {
            if arguments.len() == 4 {
                let image = raster::open(&arguments[2].clone()).unwrap();
                Command::new("display")
                        .arg(arguments[2].clone())
                        .spawn();
                let mut processed = raster::Image::blank(0, 0);
                if command == "gray_scale" {
                    processed = methods::gray_scale(image);
                }else if command == "mirror_effect" {
                    processed = methods::mirror_effect(image);
                }else if command == "contrast" {
                    processed = methods::constrast(image);
                }else if command == "negative" {
                    processed = methods::negative(image);
                }
                raster::save(&processed, &arguments[3].clone());
                Command::new("display")
                        .arg(arguments[3].clone())
                        .spawn();
                println!("Done!");
            }else if arguments.len() == 5 {
                let image = raster::open(&arguments[3].clone()).unwrap();
                Command::new("display")
                        .arg(arguments[3].clone())
                        .spawn();
                let mut processed = raster::Image::blank(0, 0);
                let factor = arguments[2].clone().parse::<f32>().unwrap();
                if command == "brightness" {
                    processed = methods::brightness(image, factor);
                }else if command == "gamma_correction" {
                    processed = methods::gamma_correction(image, factor);
                }
                raster::save(&processed, &arguments[4].clone());
                Command::new("display")
                        .arg(arguments[4].clone())
                        .spawn();
                println!("Done!");
            }
        }

    }else {
        println!("
    Hello in program to process images.
    Type 'cargo run methods' to see more.
")
    }
}
