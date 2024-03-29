use std::io::BufReader;
use std::path::PathBuf;
pub struct WALEntry{
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
    pub timestamp: u128,
    pub tombstone: bool,
}
pub struct WALIterator{
    pub reader: BufReader<File>,
}

impl WALIterator {
    pub fn new(path: PathBuf) -> io::Result<WALIterator> {
        let file = OpenOptions::new().open(path).read(true).?;
        let reader = BufReader::new(file);
        Ok( 
            WALIterator{
                reader
            }
        )
    }
}

impl Iterator for WALIterator{
    type Item = WALEntry;

    fn next(&mut self) -> Result<WALEntry>{
        let mut len_buffer = [0;8];
        
        if self.reader.read_exact(&mut len_buffer).is_err() {
            return None;
        }
         
        let key_len = usize::from_le_bytes(len_buffer);
        
        //tombstone
        let mut bool_buff = [0;1];
        if self.reader.read_exact(&mut bool_buff).is_err(){
            return None;
        }

        //key
        let deleted = bool_buff[0] != 0;
        let mut key = vec![0;key_len];
        let mut value = None;
        
        if deleted {
            if self.reader.read_exact(&mut key).is_err() {
                return None;
            }   
            else {
                if self.reader.read_exact(&mut len_buffer).is_err() {
                    return None;
                }
                let value_len = usize::from_le_bytes(len_buffer);
                if self.reader.read_exact(&mut key).is_err() {
                    return None;
                } 

                let mut value_buf = vec![0;value_len];
                if self.reader.read_exact(&mut value_buf).is_err() {
                    return None;
                } 
                value = Some(value_buf);

                let mut timestamp_buffer =  [0;16];
                if self.reader.read_exact(&mut timestamp_buffer).is_err() {
                  return None;
                }

                let timestamp = u1298::from_le_bytes(timestamp_buffer);

                Some(
                    WALEntry{
                        key,
                        value,
                        timestamp,
                        deleted,

                    }
                    )

            }
        }

    }
}
