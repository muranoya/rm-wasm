extern crate getopts;

use getopts::Options;

use std::env;
use std::fs::File;

fn parse_custom_section() {
    
}

fn check_wasm_file(f: &mut dyn std::io::Read) -> Result<bool, std::io::Error> {
    let mut buf = [0; 8];
    f.read(&mut buf).and(Ok(4)).and_then(|_| {
        Ok(buf[0] == 0 && buf[1] == 'a' as u8 && buf[2] == 's' as u8 && buf[3] == 'm' as u8 &&
           buf[4] == 1 && buf[5] == 0 && buf[6] == 0 && buf[7] == 0)
    })
}

fn print_usage(program: &str, opts: Options) {
    let desc = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&desc));
}

fn parse_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "Help");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(_) => { panic!("getopts gets error") }
    };

    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        std::process::exit(1);
    };

    let file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&args[0], opts);
        std::process::exit(1);
    };
    return file;
}

fn main() -> std::io::Result<()> {
    let opt = parse_arg();
    let mut f = File::open(opt)?;
    print!("{}", check_wasm_file(&mut f).unwrap());
    Ok(())
}
