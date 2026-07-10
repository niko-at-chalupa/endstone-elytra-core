use crate::database;

#[cfg(feature = "endgit")]
use crate::endgit;

pub struct AppState {
    pub db: database::Database,
    #[cfg(feature = "endgit")]
    pub endgit: endgit::Endgit,
}

impl AppState {
    pub fn new(db: database::Database) -> Self {
        #[cfg(feature = "endgit")]
        let endgit = match endgit::Endgit::new() {
            Ok(v) => v,
            Err(e) => {
                std::panic::panic_any(e);
            }
        };
        Self {
            db: db,
            #[cfg(feature = "endgit")]
            endgit: endgit,
        }
    }
}