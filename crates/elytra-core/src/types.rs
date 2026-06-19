use std::fmt::Error;
use crate::id::kebabify;

pub trait Plugin {
    fn name(&self) -> &str;
    fn kebabbed_name(&self) -> &str;
    fn repository_url(&self) -> &str;
}

pub struct PendingPlugin {
    name: Box<str>,
    repository_url: Box<str>,
    kebabbed_name: Box<str>,
}

impl Plugin for PendingPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn repository_url(&self) -> &str {
        &self.repository_url
    }

    fn kebabbed_name(&self) -> &str {
        &self.kebabbed_name
    }
}

impl PendingPlugin {
    pub fn new(name: &str, repository_url: &str) -> Self {
        let kebabbed_name = kebabify(name);
        Self {
            name: name.to_owned().into_boxed_str(),
            repository_url: repository_url.to_owned().into_boxed_str(),
            kebabbed_name: kebabbed_name,
        }
    }

    pub fn approve(&self) -> Result<AvailablePlugin, Error> {
        todo!()
    }
}

pub struct AvailablePlugin {
    name: Box<str>,
    releases_url: Option<Box<str>>,
    repository_url: Box<str>,
    id: u64,
    kebabbed_name: Box<str>,
}

impl Plugin for AvailablePlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn repository_url(&self) -> &str {
        &self.repository_url
    }

    fn kebabbed_name(&self) -> &str {
        &self.kebabbed_name
    }
}

impl AvailablePlugin {
    pub fn new(name: &str, repository_url: &str, id: u64) -> Self {
        let kebabbed_name = kebabify(name);
        Self {
            name: name.to_owned().into_boxed_str(),
            repository_url: repository_url.to_owned().into_boxed_str(),
            id: id,
            kebabbed_name: kebabbed_name,
            releases_url: None,
        }
    }
    
    pub fn releases_url(&self) -> Option<&str> {
        self.releases_url.as_deref()
    }

    pub fn id(&self) -> u64 {
        self.id
    }
 
}

pub enum PluginType {
    Available(AvailablePlugin),
    Pending(PendingPlugin),
}