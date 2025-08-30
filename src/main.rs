use std::collections::HashMap;
use std::io::{self, Write};

// ================== FILE STORAGE ==================
struct FileStorage {
    files: HashMap<String, String>, // path -> data
}

impl FileStorage {
    fn new() -> Self {
        FileStorage { files: HashMap::new() }
    }

    fn create(&mut self, path: &str, data: &str) {
        self.files.insert(path.to_string(), data.to_string());
        println!("File created: {}", path);
    }

    fn read(&self, path: &str) {
        match self.files.get(path) {
            Some(data) => println!("Reading {}: {}", path, data),
            None => println!("File not found: {}", path),
        }
    }

    fn write(&mut self, path: &str, data: &str) {
        if let Some(file) = self.files.get_mut(path) {
            *file = data.to_string();
            println!("File updated: {}", path);
        } else {
            println!("File not found: {}", path);
        }
    }

    fn delete(&mut self, path: &str) {
        if self.files.remove(path).is_some() {
            println!("File deleted: {}", path);
        } else {
            println!("File not found: {}", path);
        }
    }

    fn list(&self) {
        println!("Files:");
        for path in self.files.keys() {
            println!(" - {}", path);
        }
    }
}

// ================== OBJECT STORAGE ==================
#[derive(Debug)]
struct Object {
    data: String,
#[warn(dead_code)]
    metadata: HashMap<String, String>,
}

struct ObjectStorage {
    objects: HashMap<String, Object>, // id -> object
}

impl ObjectStorage {
    fn new() -> Self {
        ObjectStorage { objects: HashMap::new() }
    }

    fn create(&mut self, id: &str, data: &str, metadata: HashMap<String, String>) {
        let obj = Object { data: data.to_string(), metadata };
        self.objects.insert(id.to_string(), obj);
        println!("Object created: {}", id);
    }

    fn read(&self, id: &str) {
        match self.objects.get(id) {
            Some(obj) => println!("Reading {}: {:?} ", id, obj),
            None => println!("Object not found: {}", id),
        }
    }

    fn write(&mut self, id: &str, new_data: &str) {
        if let Some(obj) = self.objects.get_mut(id) {
            obj.data = new_data.to_string();
            println!("Object updated: {}", id);
        } else {
            println!("Object not found: {}", id);
        }
    }

    fn delete(&mut self, id: &str) {
        if self.objects.remove(id).is_some() {
            println!("Object deleted: {}", id);
        } else {
            println!("Object not found: {}", id);
        }
    }

    fn list(&self) {
        println!("Objects:");
        for id in self.objects.keys() {
            println!(" - {}", id);
        }
    }
}

// ================== BLOCK STORAGE ==================
struct BlockStorage {
    blocks: Vec<Option<String>>, // each block stores optional data
    block_size: usize,
}

impl BlockStorage {
    fn new(num_blocks: usize, block_size: usize) -> Self {
        BlockStorage {
            blocks: vec![None; num_blocks],
            block_size,
        }
    }

    fn write(&mut self, index: usize, data: &str) {
        if index >= self.blocks.len() {
            println!("Invalid block index!");
            return;
        }
        if data.len() > self.block_size {
            println!("Data too large for block (max {} bytes)", self.block_size);
            return;
        }
        self.blocks[index] = Some(data.to_string());
        println!("Data written to block {}", index);
    }

    fn read(&self, index: usize) {
        match self.blocks.get(index) {
            Some(Some(data)) => println!("Reading block {}: {}", index, data),
            _ => println!("Block {} is empty or invalid", index),
        }
    }

    fn delete(&mut self, index: usize) {
        if index < self.blocks.len() {
            self.blocks[index] = None;
            println!("Block {} cleared", index);
        } else {
            println!("Invalid block index!");
        }
    }

    fn list(&self) {
        println!("Blocks:");
        for (i, block) in self.blocks.iter().enumerate() {
            match block {
                Some(data) => println!(" - Block {}: {}", i, data),
                None => println!(" - Block {}: empty", i),
            }
        }
    }
}

// ================== CLI LOOP ==================
fn main() {
    let mut fs = FileStorage::new();
    let mut os = ObjectStorage::new();
    let mut bs = BlockStorage::new(5, 16); // 5 blocks, 16 bytes each

    println!("Welcome to Storage CLI (file, object, block). Type 'help' for commands, 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input.");
            continue;
        }
        let args: Vec<&str> = input.trim().split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        match args[0] {
            "exit" => break,
            "help" => {
                println!("Commands:");
                println!(" file create <path> <data>");
                println!(" file read <path>");
                println!(" file write <path> <data>");
                println!(" file delete <path>");
                println!(" file list");
                println!(" object create <id> <data> key=val ...");
                println!(" object read <id>");
                println!(" object write <id> <data>");
                println!(" object delete <id>");
                println!(" object list");
                println!(" block write <index> <data>");
                println!(" block read <index>");
                println!(" block delete <index>");
                println!(" block list");
            }

            // File Storage
            "file" if args.len() >= 2 => match args[1] {
                "create" if args.len() >= 4 => fs.create(args[2], args[3]),
                "read" if args.len() >= 3 => fs.read(args[2]),
                "write" if args.len() >= 4 => fs.write(args[2], args[3]),
                "delete" if args.len() >= 3 => fs.delete(args[2]),
                "list" => fs.list(),
                _ => println!("Invalid file command"),
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
                }
                "read" if args.len() >= 3 => os.read(args[2]),
                "write" if args.len() >= 4 => os.write(args[2], args[3]),
                "delete" if args.len() >= 3 => os.delete(args[2]),
                "list" => os.list(),
                _ => println!("Invalid object command"),
            },

            // Block Storage
            "block" if args.len() >= 2 => match args[1] {
                "write" if args.len() >= 4 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        bs.write(index, args[3]);
                    }
                }
                "read" if args.len() >= 3 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        bs.read(index);
                    }
                }
                "delete" if args.len() >= 3 => {
                    if let Ok(index) = args[2].parse::<usize>() {
                        bs.delete(index);
                    }
                }
                "list" => bs.list(),
                _ => println!("Invalid block command"),
            },

            _ => println!("Unknown command, type 'help'"),
        }
    }
}

