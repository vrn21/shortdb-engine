
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
impl MemTable{
    
    pub fn new() -> MemTable{
        MemTable{
            entries:Vec::new(),
            size:0,
        }
    }
    
    pub fn get_index(&self, key : &[u8]) -> Result<usize,usize>{
        self.entries.binary_search_by_keys(&key, |e| e.key.as_slice())
    }

    pub fn set(&mut self, key: &[u8], value: &[u8], timestamp : u128) {
        let entry = MemTableEntry{
            key: key.to_owned(),
            value: Some(value.to_owned()),
            timestamp,
            tombstone:false,
        };

        match self.get_index(key){
            Ok(idx) => {
                if let Some(v) = self.entries[idx].value.as_ref() {
                    if value.len() < v.len() {
                        self.size -= v.len() - value.len();
                    } else{
                        self.size += value.len() - v.len();
                    }
                }
                self.entries[idx] = entry;
            }
            Err(idx){
                self.size += key.len() + value.len() + 16 + 1;
                self.entries.insert(idx,entry)
            }
        }
    }

} 
