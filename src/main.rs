mod vid_types;

use std::{env, fs, path::{Path, PathBuf}};
use image;
use chrono::{DateTime, Datelike, Utc};
use spinners:: {Spinner, Spinners};

fn main() {
    let mut image_files: Vec<PathBuf> = vec![];
    let mut video_files: Vec<PathBuf> = vec![];

    println!("Rusty image sorter v. 0.5");
    if let Ok(current_dir) = env::current_dir() {
        //spinner initialization 
        let mut spi = Spinner::new(Spinners::Dots, "\tLooking for media files in a current working folder...".into());

        scanner(current_dir, &mut image_files, &mut video_files, false);
        spi.stop();

        println!("\nDetected images count: {} file(s)", image_files.len());
        println!("\nDetected videos count: {} file(s)", video_files.len());
        if image_files.len() > 0 {
            //show spinner one more time
            let mut spi = Spinner::new(Spinners::Dots, "\tSorting images...".into());
            mover(&image_files);
            spi.stop()
        }
        if video_files.len() > 0 {
            //show spinner one more time
            let mut spi = Spinner::new(Spinners::Dots, "\tSorting videos...".into());
            mover(&video_files);
            spi.stop()
        }

    } else {
        println!("Error getting current directory");
    }
}

fn mover(files: &Vec<PathBuf>) {
    for file in files.iter() {
        if let Ok(time) = file.metadata().unwrap().modified() {
            let dd: DateTime<Utc> = time.into();
            let pku = Path::new(file);
            let mut des = PathBuf::new();
            des.push(pku.parent().unwrap());
            des.push(dd.year().to_string());
            des.push(dd.month().to_string());
            des.push(file.file_name().unwrap());
            match fs::create_dir_all(des.parent().unwrap()) {
                Ok(_) => {
                    let res = fs::rename(file, des);
                    match res {
                        Ok(_) => {}
                        Err(e) => {println!("Error moving a file! {}", e)}
                    }
                }
                Err(e) => {println!("Error creating folder structure. {}", e)}
            }
        }
    }

}


fn scanner(path: PathBuf, img_files: &mut Vec<PathBuf>, video_files: &mut Vec<PathBuf>, recursive: bool) {
    let dir_result = fs::read_dir(path);
    match dir_result {
        Ok(res) => {
            for path in res {
                match path {
                    Ok(p) => {
                        //println!("Entry: {}", p.path().display());
                        let curr_path = p.path();
                        if curr_path.is_dir() {
                            if recursive {
                                scanner(curr_path, img_files, video_files, recursive);
                            }
                        } else {    
                            let ex = curr_path.extension();
                            match ex {
                                Some(exp) => {
                                    // check if this is image file
                                    let img_type = image::ImageFormat::from_extension(exp);
                                    match img_type {
                                        Some(_) => {
                                            img_files.push(curr_path.to_owned());
                                        }
                                        None => {}
                                    }
                                    // check if this is video file
                                    let vid_type = vid_types::VideoFormat::from_extension(exp.to_str().unwrap());
                                    match vid_type {
                                        Some(_) => {
                                            video_files.push(curr_path.to_owned());
                                        }
                                        None => {}
                                    }
                                }
                                None => {
                                    //println!("NO extenstion {}", curr_path.file_name().unwrap().to_str().unwrap());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error resolving path {}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error opening a directory. {}", e);
        }
    }
}