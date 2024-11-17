use std::path::PathBuf;

use uuid::Uuid;

pub const TMP_DIR: &str = "tmp";

pub struct Filename(PathBuf);

impl Filename {
    pub fn new() -> Self {
        const CAPACITY: usize = {
            let uuid = 8 + 4 + 4 + 4 + 12;
            let extension = 3;

            TMP_DIR.len() + "/".len() + uuid + ".".len() + extension
        };

        let uuid = Uuid::new_v4().simple();

        Self(
            PathBuf::with_capacity(CAPACITY)
                .join(TMP_DIR)
                .join(uuid.to_string()),
        )
    }
}

impl Filename {
    pub fn typ(&self) -> PathBuf {
        self.0.with_extension("typ")
    }

    pub fn svg(&self) -> PathBuf {
        self.0.with_extension("svg")
    }

    pub fn png(&self) -> PathBuf {
        self.0.with_extension("png")
    }
}
