use std::{env, fs, path::{Path, PathBuf}};
use image;
use chrono::{DateTime, Datelike, Utc};
use spinners:: {Spinner, Spinners};

fn main() {
    let mut files: Vec<PathBuf> = vec![];

    println!("Rusty image sorter v. 0.5");
    if let Ok(current_dir) = env::current_dir() {
        //spinner initialization 
        let mut spi = Spinner::new(Spinners::Dots, "\tLooking for image files in a current working folder...".into());

        scanner(current_dir, &mut files, false);
        spi.stop();

        println!("\nDetected images count: {} file(s)", files.len());
        if files.len() > 0 {
            //show spinner one more time
            let mut spi = Spinner::new(Spinners::Dots, "\tSorting images...".into());
            mover(&files);
            spi.stop()
        }
    } else {
        println!("Error getting current directory");
    }
}

fn mover(files: &Vec<PathBuf>) {
    for file in files.iter() {
        println!("{}", file.to_str().unwrap());
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


fn scanner(path: PathBuf, files: &mut Vec<PathBuf>, recursive: bool) {
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
                                scanner(curr_path, files, recursive);
                            }
                        } else {    
                            let ex = curr_path.extension();
                            match ex {
                                Some(exp) => {
                                    let img_type = image::ImageFormat::from_extension(exp);
                                    match img_type {
                                        Some(i) => {
                                            print!("Some extenstion: {} ", exp.to_str().unwrap());
                                            println!(": {} ", i.to_mime_type());
                                            files.push(curr_path);
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