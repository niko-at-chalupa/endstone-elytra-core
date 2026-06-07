use std::fmt::Error;

pub trait Plugin {
    pub fn name(&self) -> &str;
}

pub struct PendingPlugin {
    name: Box<str>,
}

impl Plugin for PendingPlugin {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PendingPlugin {
    pub fn approve(&self) -> Result<AvailablePlugin, Error> {
        todo!()
    }
}

pub struct AvailablePlugin {
    name: Box<str>,
    releases_url: Box<str>,
}

impl Plugin for AvailablePlugin {
    fn name(&self) -> &str {
        &self.name
    }
}