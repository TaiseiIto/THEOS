use std::env;

fn main() {
    let args: Args = match analyse_args(env::args()) {
        Ok(args) => args,
        Err(message) => panic!("{}", message),
    };
    println!("{:?}", args);
}

#[derive(Debug)]
struct Args {
    boot_sector: String,
    source_directory: String,
    destination: String,
}

fn analyse_args(mut args: env::Args) -> Result<Args, String> {
    let usage: String = String::from("Usage: $ ./imager /path/to/boot/sector /path/to/source/directory /path/to/destination");
    let _my_path: String = match args.next() {
        Some(my_path) => my_path,
        None => return Err(format!("{}\n{}\n", "Program path is not found.", usage)),
    };
    let boot_sector: String = match args.next() {
        Some(boot_sector) => boot_sector,
        None => return Err(format!("{}\n{}\n", "Boot sector is not found.", usage)),
    };
    let source_directory: String = match args.next() {
        Some(source_directory) => source_directory,
        None => return Err(format!("{}\n{}\n", "Boot sector is not found.", usage)),
    };
    let destination: String = match args.next() {
        Some(destination) => destination,
        None => return Err(format!("{}\n{}\n", "Boot sector is not found.", usage)),
    };
    Ok(Args {
        boot_sector,
        source_directory,
        destination,
    })
}

