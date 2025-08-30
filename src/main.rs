use std::collections::HashMap;
use std::io::{self, Write};
#[warn(unused_imports)]
use colored::*;
use colored::Colorize;
mod storage {
    pub mod file_storage;
    pub mod object_storage;
    pub mod block_storage;
}
use storage::file_storage::FileStorage;
use storage::object_storage::{ObjectStorage};
use storage::block_storage::BlockStorage;

// ...existing code...
fn main() {
    let mut fs = FileStorage::new();
    let mut os = ObjectStorage::new();
    let mut bs = BlockStorage::new(5, 16);

    println!("{}", "Welcome to Storage CLI (file, object, block). Type 'help' for commands, 'exit' to quit.".bold().cyan());

    loop {
        print!("{} ", ">".bold().yellow());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", "Error reading input.".red());
            continue;
        }
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => break,
            "help" => {
                println!("{}", "Commands:".bold().green());
                println!("{}", " file create <path> <data>".green());
                println!("{}", " file read <path>".green());
                println!("{}", " file write <path> <data>".green());
                println!("{}", " file delete <path>".green());
                println!("{}", " file list".green());
                println!("{}", " object create <id> <data> key=val ...".magenta());
                println!("{}", " object read <id>".magenta());
                println!("{}", " object write <id> <data>".magenta());
                println!("{}", " object delete <id>".magenta());
                println!("{}", " object list".magenta());
                println!("{}", " block write <index> <data>".blue());
                println!("{}", " block read <index>".blue());
                println!("{}", " block delete <index>".blue());
                println!("{}", " block list".blue());
            }

            // File Storage
            "file" if args.len() >= 2 => match args[1] {
                "create" if args.len() >= 4 => {
                    fs.create(args[2], args[3]);
                    println!("{} {}", "File created:".bold().green(), args[2].green());
                },
                "read" if args.len() >= 3 => {
                    match fs.read(args[2]) {
                        Some(data) => println!("{} {}: {}", "Reading".bold().green(), args[2].green(), data.yellow()),
                        None => println!("{} {}", "File not found:".red(), args[2].red()),
                    }
                },
                "write" if args.len() >= 4 => {
                    fs.write(args[2], args[3]);
                    println!("{} {}", "File updated:".bold().green(), args[2].green());
                },
                "delete" if args.len() >= 3 => {
                    fs.delete(args[2]);
                    println!("{} {}", "File deleted:".bold().green(), args[2].green());
                },
                "list" => {
                    println!("{}", "Files:".bold().green());
                    for path in fs.list() {
                        println!(" - {}", path.yellow());
                    }
                },
                _ => println!("{}", "Invalid file command".red()),
            },

            // Object Storage
            "object" if args.len() >= 2 => match args[1] {
                "create" if args.len() >= 4 => {
                    let id = args[2];
                    let data = args[3];
                    let mut metadata = HashMap::new();
                    for pair in &args[4..] {
                        if let Some((k, v)) = pair.split_once('=') {
                            metadata.insert(k.to_string(), v.to_string());
                        }
                    }
                    os.create(id, data, metadata);
                    println!("{} {}", "Object created:".bold().magenta(), id.magenta());
                },
                "read" if args.len() >= 3 => {
                    match os.read(args[2]) {
                        Some(obj) => println!("{} {}: {:?}", "Reading".bold().magenta(), args[2].magenta(), obj),
                        None => println!("{} {}", "Object not found:".red(), args[2].red()),
                    }
                },
                "write" if args.len() >= 4 => {
                    os.write(args[2], args[3]);
                    println!("{} {}", "Object updated:".bold().magenta(), args[2].magenta());
                },
                "delete" if args.len() >= 3 => {
                    os.delete(args[2]);
                    println!("{} {}", "Object deleted:".bold().magenta(), args[2].magenta());
                },
                "list" => {
                    println!("{}", "Objects:".bold().magenta());
                    for id in os.list() {
                        println!(" - {}", id.magenta());
                    }
                },
                _ => println!("{}", "Invalid object command".red()),
            },

            // Block Storage
            "block" if args.len() >= 2 => match args[1] {
                "write" if args.len() >= 4 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        bs.write(index, args[3]);
                        println!("{} {}", "Data written to block".bold().blue(), index.to_string().blue());
                    }
                },
                "read" if args.len() >= 3 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        match bs.read(index) {
                            Some(data) => println!("{} {}: {}", "Reading block".bold().blue(), index.to_string().blue(), data.as_str().yellow()),
                            None => println!("{} {}", "Block is empty or invalid:".red(), index.to_string().red()),
                        }
                    }
                },
                "delete" if args.len() >= 3 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        bs.delete(index);
                        println!("{} {}", "Block cleared:".bold().blue(), index.to_string().blue());
                    }
                },
                "list" => {
                    println!("{}", "Blocks:".bold().blue());
                    for (i, block) in bs.list().iter().enumerate() {
                        match block {
                            Some(data) => println!(" - Block {}: {}", i.to_string().blue(), data.as_str().yellow()),
                            None => println!(" - Block {}: {}", i.to_string().blue(), "empty".red()),
                        }
                    }
                },
                _ => println!("{}", "Invalid block command".red()),
            },

            _ => println!("{}", "Unknown command, type 'help'".red()),
        }
    }
}


