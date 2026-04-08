/*
 * Copyright (C) 2026 OpenHelloMiddle Contributors
 * SPDX-License-Identifier: GPL-3.0-or-later
 */
use clap::Parser;
use enigo::{Axis, Button, Coordinate, Enigo, Mouse, Settings};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, help = "Move mouse on X axis (supports +10/-10 for relative)")]
    x: Option<String>,

    #[arg(short, long, help = "Move mouse on Y axis (supports +10/-10 for relative)")]
    y: Option<String>,

    #[arg(short, long, help = "Get current mouse position")]
    get: bool,

    #[arg(long, help = "Click left button")]
    click_left: bool,

    #[arg(long, help = "Click right button")]
    click_right: bool,

    #[arg(long, help = "Click middle button")]
    click_middle: bool,

    #[arg(long, help = "Click forward side button")]
    click_forward: bool,

    #[arg(long, help = "Click back side button")]
    click_back: bool,

    #[arg(short, long, help = "Vertical scroll amount")]
    vertical_roll: Option<i32>,

    #[arg(short, long, help = "Horizontal scroll amount")]
    horizontal_roll: Option<i32>,
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

    let click_tasks = [
        (args.click_left,   Button::Left,   "Left"),
        (args.click_right,  Button::Right,  "Right"),
        (args.click_middle, Button::Middle, "Middle"),
        (args.click_forward,Button::Forward,"Forward"),
        (args.click_back,   Button::Back,   "Back"),
    ];

    for (should_click, button, name) in click_tasks {
        if should_click {
            if let Err(e) = enigo.button(button, enigo::Direction::Click) {
                eprintln!("{} click failed: {}", name, e);
            } else {
                println!("{} clicked", name);
                executed = true;
            }
        }
    }

    let mut do_scroll = |opt: Option<i32>, axis: Axis, name: &str, pos: &str, neg: &str| {
        if let Some(amount) = opt && amount != 0 {
            if let Err(e) = enigo.scroll(amount, axis) {
                eprintln!("{} scroll failed: {}", name, e);
            } else {
                println!("{} Scrolled {} ({})", name, amount.abs(), if amount > 0 { pos } else { neg });
                executed = true;
            }
        }
    };

    do_scroll(args.vertical_roll,  Axis::Vertical,  "Vertical",  "down", "up");
    do_scroll(args.horizontal_roll,Axis::Horizontal,"Horizontal","right","left");

    if executed {
        println!("Success");
    } else if !use_abs {
        println!("Success, but nothing happened.");
    }
}
