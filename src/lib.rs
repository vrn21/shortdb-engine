
pub struct MemTable{
    entries: Vec<MemTableEntry>,
    size: usize,
}

pub struct MemTableEntry{
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub timestamp: u128,
    pub tombstone: bool,
}

