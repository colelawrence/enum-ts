use std::io;

mod codegen;

fn main() -> Result<(), ()> {
    let mut input = String::new();
    let stdin = io::stdin();
    loop {
        match stdin.read_line(&mut input) {
            Ok(0) => {
                let parsed = codegen::parse(&input);
                println!("{}", codegen::generate(parsed));
                return Ok(());
            }
            Ok(_number_of_bytes_read) => {
                // collected into input
            }
            Err(error) => eprintln!("error: {}", error),
        }
    }
}
