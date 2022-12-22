use crate::types::assets::AssetHashes;
use sha2::{Digest, Sha256};

impl From<&String> for AssetHashes {
    fn from(html: &String) -> Self {
        let mut asset_hashes = Self::default();

        asset_hashes.insert(&html);

        asset_hashes
    }
}

impl AssetHashes {
    pub(crate) fn insert(&mut self, html: &String) {
        let mut hasher = Sha256::new();
        hasher.update(html.as_bytes());
        let sha256 = hasher.finalize().into();

        self.tree.insert("/".to_string(), sha256);
        self.tree.insert("/index.html".to_string(), sha256);
    }
}