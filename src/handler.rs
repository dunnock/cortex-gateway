use evmap::{ShallowCopy};

pub struct Handler {
    pub id: u32,
    pub path: String,
    pub topic: String,
}

impl PartialEq for Handler {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Handler {}
impl ShallowCopy for Handler {
    // It should be quite efficient - copy performed only on first write
    // Later evmap operates with 2 copies one for read another for write and oplog
     unsafe fn shallow_copy(&mut self) -> Self {
         Handler {
            id: self.id,
            path: self.path.shallow_copy(),
            topic: self.topic.shallow_copy()
         }
     }
}