pub trait Konnitiwa {
    fn konnitiwa(&self) -> String;
}

pub trait Japanese {
    fn name(&self) -> &str;
}

impl<T: Japanese> Konnitiwa for T {
    fn konnitiwa(&self) -> String {
        format!("こんにちわ, {}だよ", self.name())
    }
}
