use ic_cdk_macros::{query, init};
use std::cell::RefCell;
use std::env;
use candid::CandidType;
use serde::Deserialize;
use ic_cdk::export::candid::{candid_method, export_service};

#[derive(Default, Clone)]
pub struct State {
    pub meta: Meta,
}

#[derive(Default, CandidType, Deserialize, Clone)]
pub struct Meta {
    pub name: String,
    pub description: Option<String>,
    pub theme: String,
    pub logo: String,
    pub url: Option<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

#[init]
fn init() {
    STATE.with(|state| {
        *state.borrow_mut() = State {
            meta: Meta {
                name: env::var("name").unwrap(),
                description: Some(env::var("description").unwrap()),
                theme: env::var("theme").unwrap(),
                logo: env::var("logo").unwrap(),
                url: Some(env::var("url").unwrap()),
            }
        };
    });
}

#[candid_method(query)]
#[query]
fn meta() -> Meta {
    STATE.with(|state| state.borrow().meta.clone())
}

///
/// Generate did files
///

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("src")
            .join("demoapps");
        write(dir.join("demoapps.did"), export_candid()).expect("Write failed.");
    }
}
