use ini::Ini;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("please select config file.");
    }
    else {
        let arg = &args[1];
        let i = Ini::load_from_file(arg).unwrap();
        for (sec, prop) in i.iter() {
            println!("Section: {:?}", sec);
            for (k, v) in prop.iter() {
                println!("{}:{}", k, v);
            }
        }
    }
}
