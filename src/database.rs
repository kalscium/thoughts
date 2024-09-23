use std::path::Path;
use stack_db::prelude::*;

pub struct Database<'l> {
    pub stackdb: StackDB<'l, SkdbDirAlloc>,
    tail: u64,
    idx: u64,
}

impl<'l> Database<'l> {
    #[inline]
    pub fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut stackdb = StackDB::new(SkdbDirAlloc::new(path)?)?;
        stackdb.write(0, &8u64.to_be_bytes())?;
        stackdb.flush()?;

        Ok(Self {
            stackdb,
            tail: 8,
            idx: 8,
        })
    }

    #[inline]
    pub fn push(&mut self, value: &[u8]) -> Result<(), Error> {
        // write string and string length
        self.stackdb.write(self.tail, &value.len().to_be_bytes())?;
        self.stackdb.write(self.tail+8, value)?;

        // update tail
        self.tail += value.len() as u64 + 8;
        self.stackdb.write(0, &self.tail.to_be_bytes())?;
        self.stackdb.flush()?;

        Ok(())
    }

    #[inline]
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut stackdb = StackDB::new(SkdbDirAlloc::load(path.as_ref())?)?;
        let tail = u64::from_be_bytes((*stackdb.read(0..8)?).try_into().unwrap());
        Ok(Self {
            stackdb,
            tail,
            idx: 8,
        })
    }
}

impl<'l> Iterator for Database<'l> {
    type Item = Box<[u8]>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.tail { return None };
        if let Ok(len) = self.stackdb.read(self.idx..self.idx+8) {
            let len = u64::from_be_bytes((*len).try_into().unwrap());
            if let Ok(bytes) = self.stackdb.read(self.idx+8..self.idx+8+len) {
                // update idx
                self.idx += 8 + len;
                
                Some(bytes)
            } else { None }
        } else { None }
    }
}
