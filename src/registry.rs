pub struct Registry {
    pub nids: u32,
    pub nlabels: u32,
    pub ret: u32,
}
impl Registry {
    pub fn new() -> Self {
        Self {
            nids: 0,
            nlabels: 0,
            ret: 0
        }
    }
}