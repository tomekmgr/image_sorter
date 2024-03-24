use std::{fs, path::{Path, PathBuf}, str::FromStr};
use image;
use chrono::{DateTime, Datelike, Utc};

fn main() {
    let mut files: Vec<PathBuf> = vec![];

    scanner(PathBuf::from_str(r".").unwrap(), &mut files, false);
    mover(&files);

    println!("Hello, world! {}", files.len());
}


fn mover(files: &Vec<PathBuf>) {
    for file in files.iter() {
        println!("{}", file.to_str().unwrap());
        if let Ok(time) = file.metadata().unwrap().modified() {
            let dd: DateTime<Utc> = time.into();
            println!("{time:?}  {}", dd);
            let pku = Path::new(file);
            let mut des = PathBuf::new();
            des.push(pku.parent().unwrap());
            des.push(dd.year().to_string());
            des.push(dd.month().to_string());
            des.push(file.file_name().unwrap());
            println!("{}", des.to_str().unwrap());
            match fs::create_dir_all(des.parent().unwrap()) {
                Ok(_) => {
                    let res = fs::rename(file, des);
                    match res {
                        Ok(_) => {println!("File moved")}
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
//                            println!("{}", p.path().extension().unwrap().to_str().unwrap());                       
                            //files.push(curr_path);
                        }
                    },
                    Err(e) => {
                        println!("Erorex {}", e);
                    }
                }
            }
        },

        Err(e) => {
            println!("yeah, error! {}", e);
        }
    }
}