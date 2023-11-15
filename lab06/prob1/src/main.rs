use anyhow::Result;
use std::fs;

trait Command {
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: Vec<&str>);
}

struct Ping {}

impl Command for Ping {
    fn get_name(&self) -> &'static str {
        "ping"
    }

    fn exec(&mut self, _args: Vec<&str>) {
        println!("pong!");
    }
}

struct Count {}

impl Command for Count {
    fn get_name(&self) -> &'static str {
        "count"
    }

    fn exec(&mut self, args: Vec<&str>) {
        println!("counted {} args", args.len());
    }
}

struct Times {
    count: u32,
}

impl Command for Times {
    fn get_name(&self) -> &'static str {
        "times"
    }

    fn exec(&mut self, _args: Vec<&str>) {
        self.count += 1;
        println!("'times' command has been called {} times.", self.count);
    }
}

struct Help {}

impl Command for Help {
    fn get_name(&self) -> &'static str {
        "help"
    }

    fn exec(&mut self, _args: Vec<&str>) {
        println!("Available commands:");
        println!("\t>ping");
        println!("\t>count");
        println!("\t>times");
        println!("\t>help");
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        return Terminal {
            commands: Vec::new(),
        };
    }

    fn register(&mut self, comm: Box<dyn Command>) {
        self.commands.push(comm);
    }

    fn run(&mut self) -> Result<()> {
        let commands: String = fs::read_to_string("src/input.txt")?;
        for line in commands.lines() {
            let mut comm_and_args: Vec<&str> = Vec::new();
            for word in line.split(" ") {
                if word.len() == 0 {
                    continue;
                }
                comm_and_args.push(word);
            }
            if comm_and_args.len() == 0 {
                continue;
            }
            if comm_and_args[0] == "stop" {
                println!("Stop command reached.");
                return Ok(());
            }
            
            let mut found: bool = false;
            let mut most_likely_match = "help";
            for valid_comm in &mut self.commands {
                if comm_and_args[0]==valid_comm.get_name() {
                    valid_comm.exec(comm_and_args[1..comm_and_args.len()].to_vec());
                    found = true;
                    break;
                }
                if comm_and_args[0].to_lowercase()==valid_comm.get_name().to_lowercase() {
                    most_likely_match = valid_comm.get_name();
                }
            }
            if found == false {
                println!("Command '{}' not found. Try '{}'.", comm_and_args[0], most_likely_match);
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut t = Terminal::new();

    t.register(Box::new(Ping {}));
    t.register(Box::new(Count {}));
    t.register(Box::new(Times { count: 0 }));
    t.register(Box::new(Help {}));

    t.run()?;
    Ok(())
}
