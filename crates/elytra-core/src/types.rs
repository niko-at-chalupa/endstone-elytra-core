use std::fmt::Error;
use crate::id::{compute_plugin_id, kebabify};

pub trait Plugin {
    fn name(&self) -> &str;
    fn kebabbed_name(&self) -> &str;
    fn repository_url(&self) -> &str;
    // ~~EndGit plugins have their own IDs, so sometimes we need it
    // as None.~~
    // EndGit plugins can have arbitrary IDs
    fn id(&self) -> u64;
}

pub struct PendingPlugin {
    name: Box<str>,
    repository_url: Box<str>,
    id: u64,
    kebabbed_name: Box<str>,
}

impl Plugin for PendingPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn repository_url(&self) -> &str {
        &self.repository_url
    }

    fn id(&self) -> u64 {
        self.id
    }

    fn kebabbed_name(&self) -> &str {
        &self.kebabbed_name
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

    fn id(&self) -> u64 {
        self.id
    }

    fn kebabbed_name(&self) -> &str {
        &self.kebabbed_name
    }
}

impl AvailablePlugin {
    pub fn releases_url(&self) -> &str {
        &self.releases_url
    }
}

pub enum PluginType {
    Available(AvailablePlugin),
    Pending(PendingPlugin),
}