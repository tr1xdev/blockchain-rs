use crate::core::block::Block;

pub struct Chain {
    pub blocks: Vec<crate::core::block::Block>,
}

impl Chain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Self::genesis_block()],
        }
    }

    pub fn genesis_block() -> Block {
        Block {
            index: 0,
            data: "genesis".to_string(),
            prev_hash: "0".to_string(),
            hash: "genesis".to_string(),
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<(), &'static str> {
        if data.is_empty() {
            return Err("data cannot be empty");
        }

        let last = self.blocks.last().unwrap();
        let new_index = last.index + 1;
        let hash = format!("{}:{}:{}", new_index, data, last.hash);

        let new_block = Block {
            index: new_index,
            data,
            prev_hash: last.hash.clone(),
            hash,
        };

        let idx = new_block.index;
        self.blocks.push(new_block);
        println!("✅ Block #{} added", idx);
        Ok(())
    }
}
