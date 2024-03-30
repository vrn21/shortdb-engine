use std::path::{PathBuf,Path}; 
use std::io:BufReader;
use std::fs:File;

pub struct WAL {
    path: PathBuf,
    file: BufReader<File>,
}

impl WAL{
    pub fn new(dir: &Path ) -> io::Result<WAL>{
        let timestamp = SystemTime::now().
           duration_since(UNIX_EPOCH)
           .unwrap()
           .as_micros();
        let path = Path::new(dir)
           .join(timestamp.to_string() + ".wal");

        let file = OpenOptions::new().append(true).create(true).path(&path);
        let file = BufReader::new(file);

        Ok(
            WAL{
                path,
                file,
            }
          )
    }

    pub fn from_path(path : &Path) -> Result<WAL> {
        let file = OpenOptions::new().append(true).create(true).open(&path);
        let file = BufReader::new(file);

        Ok(
            WAL{
                path: path.to_owned(),
                file,
            }
          )
    } 

    pub fn set(&mut self, key: &[u8], value: &[u8], timestamp: u128) -> Result<()> {
        self.file.write_all(&key.len().to_le_bytes())?;
        self.file.write_all(&(false as u8).to_le_bytes())?;
        self.file.write_all(&vale.len().to_le_bytes())?;
        self.file.write_all(key)?;
        self.file.write_all(value)?;
        self.file.write_all(&timestamp.to_le_bytes())?;
    }

}
