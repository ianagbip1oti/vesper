#![feature(stdarch_const_x86)]
mod lane;
mod board;
mod movegen;
mod eval;
mod search;
mod uci;
#[cfg(test)]
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "test" {
        return;
    }
    uci::main_loop();
}
