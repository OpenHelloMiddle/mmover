use clap::Parser;
use enigo::{Coordinate, Enigo, Mouse, Settings};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    x: Option<String>,

    #[arg(short, long)]
    y: Option<String>,
}

fn parse_coord(input: &str) -> Result<(bool, i32), String> {
    let is_relative = input.starts_with('+') || input.starts_with('-');
    input.parse::<i32>()
        .map(|v| (is_relative, v))
        .map_err(|_| format!("Invaild data: {}", input))
}

fn main() {
    let args = Args::parse();

    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to init enigo: {}", e);
            std::process::exit(1);
        }
    };

    let mut use_abs = false;
    let mut target_x = 0i32;
    let mut target_y = 0i32;

    if let Some(x_str) = &args.x {
        match parse_coord(x_str) {
            Ok((false, val)) => { target_x = val; use_abs = true; }
            Ok((true, delta)) => {
                if let Err(e) = enigo.move_mouse(delta, 0, Coordinate::Rel) {
                    eprintln!("Relative movement failed(x): {}", e);
                }
            }
            Err(e) => { eprintln!("Position parsing error(x): {}", e); std::process::exit(1); }
        }
    }
    if let Some(y_str) = &args.y {
        match parse_coord(y_str) {
            Ok((false, val)) => { target_y = val; use_abs = true; }
            Ok((true, delta)) => {
                if let Err(e) = enigo.move_mouse(0, delta, Coordinate::Rel) {
                    eprintln!("Relative movement failed(y): {}", e);
                }
            }
            Err(e) => { eprintln!("Position parsing error(y): {}", e); std::process::exit(1); }
        }
    }

    if use_abs {
        let (curr_x, curr_y) = enigo.location()
            .expect("Failed to get current mouse position");
        if !args.x.as_ref().is_some_and(|s| !s.starts_with('+') && !s.starts_with('-')) {
            target_x = curr_x;
        }
        if !args.y.as_ref().is_some_and(|s| !s.starts_with('+') && !s.starts_with('-')) {
            target_y = curr_y;
        }

        enigo.move_mouse(target_x, target_y, Coordinate::Abs)
            .expect("Absolute movement failure");
        println!("Success, M({}, {})", target_x, target_y);
    } else {
        println!("Success");
    }
}
