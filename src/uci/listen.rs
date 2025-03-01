use crate::search::eval::eval;
use crate::search::hashtable::Hashtable;
use crate::search::ttentry::TTEntry;
use crate::uci::{go, position, setoption};
use crate::{chess::position::Position, uci::moves};

pub fn listen() {
    let mut hash = 16;
    let mut is_frc = false;

    println!(
        "id name Rawr {}",
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
    );
    println!("id author kz04px");
    println!("option name UCI_Chess960 type check default {}", is_frc);
    println!("option name Hash type spin default {hash} min 1 max 4096");
    println!("uciok");

    let mut pos = Position::from_fen("startpos");
    let mut history = vec![pos.hash];
    let mut tt = Hashtable::<TTEntry>::new(0);
    let mut input = String::new();
    let mut got_isready = false;

    loop {
        input.clear();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break,
        }

        let mut stream = input.split_ascii_whitespace();
        match stream.next().unwrap_or("") {
            "isready" => {
                got_isready = true;
                break;
            }
            "setoption" => setoption::setoption(&mut stream, |name, value| match name {
                "Hash" | "hash" => {
                    if let Ok(size) = value.parse::<usize>() {
                        hash = size.clamp(1, 4096);
                    }
                }
                "UCI_Chess960" => {
                    is_frc = value == "true";
                    pos.is_frc = is_frc;
                }
                _ => {}
            }),
            "quit" => return,
            _ => {
                break;
            }
        }
    }

    tt.resize(hash);
    if got_isready {
        println!("readyok");
    }

    loop {
        if got_isready {
            input.clear();
            match std::io::stdin().read_line(&mut input) {
                Ok(0) => break,
                Ok(_) => {}
                Err(_) => break,
            }
        }
        got_isready = true;

        let mut stream: std::str::SplitAsciiWhitespace<'_> = input.split_ascii_whitespace();
        match stream.next().unwrap_or("") {
            "ucinewgame" => {
                pos = Position::startpos();
                pos.is_frc = is_frc;
                history.clear();
                history.push(pos.hash);
                tt.clear();
            }
            "isready" => println!("readyok"),
            "print" | "display" | "board" => print!("{pos}"),
            "go" => go::go(&mut stream, &mut pos, &mut history, &mut tt),
            "position" => {
                position::position(&mut stream, &mut pos, &mut history);
                pos.is_frc = is_frc;
            }
            "moves" => moves::moves(&mut stream, &mut pos, &mut history),
            "setoption" => setoption::setoption(&mut stream, |name, value| match name {
                "Hash" | "hash" => {
                    if let Ok(size) = value.parse::<usize>() {
                        hash = size.clamp(1, 4096);
                        tt.resize(hash);
                    }
                }
                "UCI_Chess960" => {
                    is_frc = value == "true";
                    pos.is_frc = is_frc;
                }
                _ => {}
            }),
            "history" => history.iter().for_each(|hash| println!("{:#x}", hash)),
            "eval" => println!("{}", eval(&pos)),
            "quit" => break,
            _ => {}
        }
    }
}
