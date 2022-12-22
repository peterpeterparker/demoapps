use ic_cdk_macros::{query, init};
use std::cell::RefCell;
use candid::CandidType;
use serde::Deserialize;
use ic_cdk::export::candid::{candid_method, export_service};
use ic_cdk::print;

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
    let description = option_env!("APP_DESCRIPTION").map(|value| value.to_string());
    let url = option_env!("APP_URL").map(|value| value.to_string());

    STATE.with(|state| {
        *state.borrow_mut() = State {
            meta: Meta {
                name: option_env!("APP_NAME").unwrap().to_string(),
                description: description.clone(),
                theme: option_env!("APP_THEME").unwrap().to_string(),
                logo: option_env!("APP_LOGO").unwrap().to_string(),
                url: url.clone(),
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

        let dir = PathBuf::from(option_env!("CARGO_MANIFEST_DIR").unwrap());
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
