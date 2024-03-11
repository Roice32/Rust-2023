use anyhow::Result;
use rusqlite;
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

struct BookmarkEntry {
    name: String,
    url: String,
}

struct Bookmark {}

impl Command for Bookmark {
    fn get_name(&self) -> &'static str {
        "bm"
    }

    fn exec(&mut self, args: Vec<&str>) {
        let conn = rusqlite::Connection::open("bookmarks.db").unwrap();
        let create = r"
create table if not exists bookmarks (
    name text    not null,
    url  text    not null);";
        conn.execute(create, ()).unwrap();
        if args[0] == "add" {
            conn.execute(
                "insert into bookmarks (name, url) values (?1, ?2);",
                (args[1], args[2]),
            )
            .unwrap();
        } else if args[0] == "search" && args.len() > 1 {
            let mut stmt = conn.prepare("select * from bookmarks").unwrap();
            let bookmars_iter = stmt
                .query_map([], |row| {
                    Ok(BookmarkEntry {
                        name: row.get("name").unwrap(),
                        url: row.get("url").unwrap(),
                    })
                })
                .unwrap();
            let mut no_results = true;
            for bm in bookmars_iter {
                let b = bm.unwrap();
                if b.name.contains(args[1]) {
                    no_results = false;
                    println!("name={} url={}", b.name, b.url);
                }
            }
            if no_results == true {
                println!("Found no bookmarks having '{}' in their name.", args[1]);
            }
        } else {
            println!("Invalid parameters of command 'bm'.");
        }
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
        println!("\t>bm");
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
                if comm_and_args[0] == valid_comm.get_name() {
                    valid_comm.exec(comm_and_args[1..comm_and_args.len()].to_vec());
                    found = true;
                    break;
                }
                if comm_and_args[0].to_lowercase() == valid_comm.get_name().to_lowercase() {
                    most_likely_match = valid_comm.get_name();
                }
            }
            if found == false {
                println!(
                    "Command '{}' not found. Try '{}'.",
                    comm_and_args[0], most_likely_match
                );
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
    t.register(Box::new(Bookmark {}));
    t.register(Box::new(Help {}));

    t.run()?;
    Ok(())
}
