# 🗄️ Rust Storage Simulator (File, Object, Block)

This project is a **learning tool** to understand the differences between **File Storage**, **Object Storage**, and **Block Storage**.

It provides an **interactive CLI** where you can create, read, write, delete, and list data using all three storage models.

---

## 📚 Storage Models

### 1. File Storage

- Hierarchical structure (folders and files).
- Access via **paths** (`/docs/readme.md`).
- Examples: **NTFS, ext4, NFS, SMB**.

**Good for:** Small files, hierarchical organization, granular permissions.  
**Limitations:** Limited scalability, weak metadata support.

---

### 2. Object Storage

- Flat namespace.
- Each object has:
  - **Data** (binary or text)
  - **Unique ID**
  - **Metadata** (key-value pairs)
- Examples: **AWS S3, Azure Blob, GCP Cloud Storage**.

**Good for:** Large datasets, unstructured data (media, logs, backups).  
**Limitations:** Higher latency for small files, no real folder structure.

---

### 3. Block Storage

- Raw storage blocks of fixed size (e.g. 512 bytes).
- Filesystems are built **on top of block storage**.
- Examples: **HDD/SSD volumes, Amazon EBS**.

**Good for:** Databases, low-latency operations, fine-grained I/O.  
**Limitations:** No built-in hierarchy or metadata, needs a filesystem.

---

## ⚡ Features

- **File Storage:**
  - Create, Read, Write, Delete files.
  - Path-based hierarchical keys.
- **Object Storage:**
  - Create, Read, Write, Delete objects.
  - Each object has **metadata**.
- **Block Storage:**
  - Write, Read, Delete fixed-size blocks.
  - Raw block-level operations.
- **Interactive CLI:**
  - Explore the three storage systems in one place.

---

## ▶️ Running the Project

```bash
# Clone repo
git clone https://github.com/yourusername/rust-storage-sim.git
cd rust-storage-sim

# Run
cargo run
```

You’ll see:

```
Welcome to Storage CLI (file, object, block).
Type 'help' for commands, 'exit' to quit.
```

---

## 💻 Commands

### File Storage

```
file create <path> <data>     # Create file
file read <path>              # Read file contents
file write <path> <data>      # Update file contents
file delete <path>            # Delete file
file list                     # List all files
```

### Object Storage

```
object create <id> <data> key=val ...   # Create object with metadata
object read <id>                        # Read object (data + metadata)
object write <id> <data>                # Update object data
object delete <id>                      # Delete object
object list                             # List all objects
```

### Block Storage

```
block write <index> <data>    # Write data to block index
block read <index>            # Read block contents
block delete <index>          # Clear block
block list                    # List all blocks
```

---

## 🔥 Example Session

```
> file create /docs/readme.md "Hello File!"
File created: /docs/readme.md

> file read /docs/readme.md
Reading /docs/readme.md: Hello File!

> object create obj123 "Hello Object!" author=Nalin type=text
Object created: obj123

> object read obj123
Reading obj123: Object { data: "Hello Object!", metadata: {"author": "Nalin", "type": "text"} }

> block write 0 "BlockData1"
Data written to block 0

> block list
 - Block 0: BlockData1
 - Block 1: empty
 - Block 2: empty
 - Block 3: empty
 - Block 4: empty
```

---

## 🎯 Learning Outcomes

- Understand **File vs Object vs Block storage models**.
- See how **paths, IDs, and raw blocks** change how we store and retrieve data.
- Practice **Rust ownership, HashMaps, and CLI parsing**.
