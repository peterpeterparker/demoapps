mod types;

use ic_cdk_macros::{query, init, post_upgrade};
use std::cell::RefCell;
use candid::CandidType;
use serde::Deserialize;
use ic_cdk::export::candid::{candid_method, export_service};
use crate::types::http::{HttpRequest, HttpResponse};

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
    init_state()
}

#[post_upgrade]
fn post_upgrade() {
    init_state()
}

fn init_state() {
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

#[query]
#[candid_method(query)]
fn http_request(HttpRequest { method: _, url: _, headers: _, body: _ }: HttpRequest) -> HttpResponse {
    let name = STATE.with(|state| state.borrow().meta.name.clone());
    let body = format!("<html lang=\"en\"><body><h1>{}</h1></body></html>", name);

    HttpResponse {
        body: body.as_bytes().to_vec(),
        headers: Vec::new(),
        status_code: 200,
        streaming_strategy: None,
    }
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
