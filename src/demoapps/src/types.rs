pub mod state {
    use crate::types::assets::AssetHashes;
    use candid::CandidType;
    use serde::Deserialize;

    #[derive(Default, Clone)]
    pub struct State {
        pub meta: Meta,
        pub body: String,
        pub asset_hashes: AssetHashes,
    }

    #[derive(Default, CandidType, Deserialize, Clone)]
    pub struct Meta {
        pub name: String,
        pub description: Option<String>,
        pub theme: String,
        pub logo: String,
        pub url: Option<String>,
    }
}

pub mod http {
    use candid::{CandidType, Deserialize, Func};
    use serde_bytes::ByteBuf;

    #[derive(CandidType, Deserialize, Clone)]
    pub struct HeaderField(pub String, pub String);

    #[derive(CandidType, Deserialize, Clone)]
    pub struct HttpRequest {
        pub url: String,
        pub method: String,
        pub headers: Vec<HeaderField>,
        pub body: Vec<u8>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct HttpResponse {
        pub body: Vec<u8>,
        pub headers: Vec<HeaderField>,
        pub status_code: u16,
        pub streaming_strategy: Option<StreamingStrategy>,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub enum StreamingStrategy {
        Callback {
            token: StreamingCallbackToken,
            callback: Func,
        },
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct StreamingCallbackToken {
        pub full_path: String,
        pub token: Option<String>,
        pub headers: Vec<HeaderField>,
        pub sha256: Option<ByteBuf>,
        pub index: usize,
        pub encoding_type: String,
    }

    #[derive(CandidType, Deserialize, Clone)]
    pub struct StreamingCallbackHttpResponse {
        pub body: Vec<u8>,
        pub token: Option<StreamingCallbackToken>,
    }
}

pub mod assets {
    use ic_certified_map::{Hash, RbTree};
    use std::clone::Clone;

    #[derive(Default, Clone)]
    pub struct AssetHashes {
        pub tree: RbTree<String, Hash>,
    }
}