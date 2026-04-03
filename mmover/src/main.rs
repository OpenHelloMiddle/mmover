/*
 * Copyright (C) 2026 OpenHelloMiddle Contributors
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use clap::Parser;
use enigo::{Axis, Button, Coordinate, Enigo, Mouse, Settings};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    x: Option<String>,

    #[arg(short, long)]
    y: Option<String>,

    #[arg(short, long)]
    get: bool,

    #[arg(long)]
    click_left: bool,

    #[arg(long)]
    click_right: bool,

    #[arg(long)]
    click_middle: bool,

    #[arg(long)]
    click_forward: bool,

    #[arg(long)]
    click_back: bool,

    #[arg(short, long)]
    roll: Option<i32>,
}

fn parse_coord(input: &str) -> Result<(bool, i32), String> {
    let is_relative = input.starts_with('+') || input.starts_with('-');
    input.parse::<i32>()
        .map(|v| (is_relative, v))
        .map_err(|_| format!("Invaild data: {}", input))
}

fn main() {
    let mut executed = false;
    let args = Args::parse();

    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to init enigo: {}", e);
            std::process::exit(1);
        }
    };

    if args.get {
        match enigo.location() {
            Ok((x, y)) => {
                println!("Mouse position: ({}, {})", x, y);
            }
            Err(e) => {
                eprintln!("Failed to get mouse position: {}", e);
            }
        }
        executed = true;
    }

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
    }

    if args.click_left {
        if let Err(e) = enigo.button(Button::Left, enigo::Direction::Click) {
            eprintln!("Left click failed: {}", e);
        } else {
            println!("Left clicked");
            executed = true;
        }
    }

    if args.click_right {
        if let Err(e) = enigo.button(Button::Right, enigo::Direction::Click) {
            eprintln!("Right click failed: {}", e);
        } else {
            println!("Right clicked");
            executed = true;
        }
    }

    if args.click_middle {
        if let Err(e) = enigo.button(Button::Middle, enigo::Direction::Click) {
            eprintln!("Middle click failed: {}", e);
        } else {
            println!("Middle clicked");
            executed = true;
        }
    }

    if args.click_forward {
        if let Err(e) = enigo.button(Button::Forward, enigo::Direction::Click) {
            eprintln!("Forward click failed: {}", e);
        } else {
            println!("Forward clicked");
            executed = true;
        }
    }

    if args.click_back {
        if let Err(e) = enigo.button(Button::Back, enigo::Direction::Click) {
            eprintln!("Back click failed: {}", e);
        } else {
            println!("Back clicked");
            executed = true;
        }
    }

    if let Some(roll_amount) = args.roll
        && roll_amount != 0 {
            if let Err(e) = enigo.scroll(roll_amount, Axis::Vertical) {
                eprintln!("Scroll failed: {}", e);
            } else {
                println!("Scrolled {} ({})", roll_amount.abs(), if roll_amount > 0 { "up" } else { "down" });
                executed = true;
            }
        }

    if executed {
        println!("Success");
    } else if !use_abs {
        println!("Success, but nothing happened.");
    }
}
