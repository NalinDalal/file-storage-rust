pub struct BlockStorage {
    blocks: Vec<Option<String>>, // each block stores optional data
    block_size: usize,
}

impl BlockStorage {
    pub fn new(num_blocks: usize, block_size: usize) -> Self {
        BlockStorage {
            blocks: vec![None; num_blocks],
            block_size,
        }
    }

    pub fn write(&mut self, index: usize, data: &str) {
        if index >= self.blocks.len() {
            return;
        }
        if data.len() > self.block_size {
            return;
        }
        self.blocks[index] = Some(data.to_string());
    }

    pub fn read(&self, index: usize) -> Option<&String> {
        match self.blocks.get(index) {
            Some(Some(data)) => Some(data),
            _ => None,
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index < self.blocks.len() {
            self.blocks[index] = None;
        }
    }

    pub fn list(&self) -> &Vec<Option<String>> {
        &self.blocks
    }
}

