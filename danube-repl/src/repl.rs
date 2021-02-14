use danube_vm::{version::version, VM};

#[derive(Debug)]
pub(crate) struct REPL {
    vm: VM,
}

impl REPL {
    pub(crate) fn new() -> Self {
        REPL { vm: VM::new() }
    }

    pub(crate) fn run(&mut self) {
        use std::io::Write;

        println!("Welcome to Danube {}", version());
        println!();
        println!("------------------------------------------");
        println!("We appreciate your concern!");
        println!("You can improve the Danube experience.");
        println!("Always we are waiting for you.");
        println!();
        println!("[link: https://github.com/Danue1/danube/]");
        println!("------------------------------------------");
        println!();
        println!("You can quit with \"exit\" or CTRL + C.");

        let mut buffer = String::new();

        loop {
            let stdin = std::io::stdin();

            print!("> ");
            std::io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");

            let command = buffer.trim();
            match command {
                "exit" => {
                    std::process::exit(0);
                }
                _ => {
                    println!("Not implemented!");
                }
            }

            buffer.clear();
        }
    }
}
