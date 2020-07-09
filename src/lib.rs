pub mod token;
pub mod parse;

use std::io;
use std::process;
use std::error::Error;

pub enum Status {
    Continue,
    Exit,
}

pub fn run(input: &String) -> Result<Status, Box<dyn Error>> {
    if input.is_empty() {
        return Ok(Status::Exit);
    }

    let tokens = token::List::new(input);

    exec_and_wait(tokens)?;

    Ok(Status::Continue)
}

pub fn get_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;

    Ok(buffer.to_string())
}


fn exec_and_wait(tokens: token::List) -> Result<(), io::Error> {

    /*
    let mut child = process::Command::new(cmd)
        .args(args)
        .spawn()?;

    child.wait()?;
    */

    Ok(())
}
