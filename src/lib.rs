#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

struct Proposal {
    pub round: usize,
    pub message: String,
}

struct Acceptor {
    pub round: usize,
    pub accepted: Vec<Proposal>,
    pub backend: Service,
}

trait Service {
    fn append(&mut self, path: String, message: String) -> String;
    fn fetch(&self, path: String) -> String;
}
