#[cfg(feature = "endgit")]
use crate::endgit;

pub struct AppState {
    #[cfg(feature = "endgit")]
    pub endgit: endgit::Endgit,
}

impl AppState {
    pub fn new() -> Self {
        #[cfg(feature = "endgit")]
        let endgit = match endgit::Endgit::new() {
            Ok(v) => v,
            Err(e) => {
                std::panic::panic_any(e);
            }
        };
        Self {
            #[cfg(feature = "endgit")]
            endgit: endgit,
        }
    }
}