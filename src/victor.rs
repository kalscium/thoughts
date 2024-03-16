use lazy_db::*;

pub struct Victor {
    container: LazyContainer,
    tail: u64,
    idx: u64,
}

impl Victor {
    pub fn new(container: LazyContainer) -> Result<Self, LDBError> {
        LazyData::new_u64(container.data_writer("tail")?, 0)?;
        Ok(Self {
            container,
            tail: 0,
            idx: 0
        })
    }

    pub fn as_container(&self) -> &LazyContainer { &self.container }

    pub fn push(&mut self, value: &str) -> Result<(), LDBError> {
        LazyData::new_string(self.container.data_writer(self.tail.to_string())?, value)?;
        self.tail += 1;
        LazyData::new_u64(self.container.data_writer("tail")?, self.tail)
    }

    pub fn load(container: LazyContainer) -> Result<Self, LDBError> {
        let tail = container.read_data("tail")?.collect_u64()?;
        Ok(Self {
            container,
            tail,
            idx: 0,
        })
    }
}

impl Iterator for Victor {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(data) = self.container.read_data(self.idx.to_string()) {
            if let Ok(string) = data.collect_string() {
                self.idx += 1;
                Some(string)
            } else { None }
        } else { None }
    }
}
