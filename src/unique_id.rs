pub struct UniqueIdService {
    ids: Vec<String>
}
impl UniqueIdService {
    pub fn new() -> Self {
        Self {
            ids: vec![]
        }
    }

    pub fn generate_unique_name<'a>(&'a mut self, prefix: &str) -> &'a String {
        let filtered = self.ids.iter().filter(|id| id.contains(prefix)).collect::<Vec<_>>();
        self.ids.push(format!("{}{}", prefix, filtered.len()));

        self.ids.last().unwrap()
    }
}