use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::process;

struct FileInfo {
    file_name: String,
    file_string: String,
    file_dir: String,
    file_extension: String,
}

impl FileInfo {
    fn new(file_place: String) -> FileInfo {
        let mut file_string = String::new();
        File::open(&file_place)
            .unwrap_or_else(|_| {
                eprintln!("no file found: {}", &file_place);
                process::exit(1);
            })
            .read_to_string(&mut file_string)
            .unwrap_or_else(|_| {
                eprintln!("failed to read: {}", &file_place);
                process::exit(1);
            });

        let file_path = Path::new(&file_place);

        let file_name = file_path.file_stem()
            .unwrap().to_str()
            .unwrap_or_else(|| {
                eprintln!("no file name found");
                process::exit(1);
            }).to_string();

        let file_extension = file_path.extension()
            .unwrap().to_str()
            .unwrap_or_else(|| {
                eprintln!("no file extension found");
                process::exit(1);
            }).to_string();

        let file_dir = file_path.parent().unwrap().to_str().unwrap_or_else(|| {panic!("no file directory found")}).to_string();
        FileInfo {
            file_name: file_name,
            file_string: file_string,
            file_dir: file_dir,
            file_extension: file_extension,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_place = &args[1];
    let input_file = FileInfo::new(file_place.to_string());

    println!("input file name: {}", input_file.file_name);

    if input_file.file_extension != "srt" {
        eprintln!("Error: Input file extension is not 'srt'");
        process::exit(1);
    }

    println!("Start parsing file: {}", input_file.file_name);

    let output_file: File;

    if args.len() > 2 {
        let output_path = Path::new(&input_file.file_dir.to_string()).join(&args[2]);
        output_file = File::create(&output_path)
            .unwrap_or_else(|_| { panic!("failed to create output file") });
        println!("Output file created: {}", output_path.display());
    } else {
        let output_path = Path::new(&input_file.file_dir.to_string()).join(&input_file.file_name);
        output_file = File::create(&output_path.with_extension("lrc"))
            .unwrap_or_else(|_| { panic!("failed to create output file") });
        println!("Output file created: {}", output_path.with_extension("lrc").display());
    }

    if let Err(err) = srt2lrc(input_file, output_file) {
        eprintln!("Error: {}", err);
        process::exit(1);
    } else {
        println!("File converted successfully!");
    }
}

fn srt2lrc(file: FileInfo, mut output_file: File) -> Result<(), std::io::Error> {
    let mut lines = file.file_string.lines();
    let mut line = lines.next();
    let mut line_num = 1;

    while line.is_some() {
        let current_line = line.unwrap();
        let mut bind_time: Vec<u32> = vec![];
        if line_num % 4 == 2 {
            let times: Vec<&str> = current_line.split("-->").collect();
            let start_time: Vec<&str> = times[0].split(":").collect();
            let start_sec: Vec<&str> = start_time[2].split(",").collect();
            bind_time = vec![start_time[0].parse::<u32>().unwrap() * 60 + start_time[1].parse::<u32>().unwrap(), start_sec[0].parse::<u32>().unwrap()];
        }
        line = lines.next();
        line_num += 1;

        if let Some(words) = line {
            if bind_time.len() > 0 {
                writeln!(output_file, "[{}:{}.00] {}", bind_time[0], bind_time[1], words)?;
            }
        }
    }

    Ok(())
}