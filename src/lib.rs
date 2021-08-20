#![recursion_limit = "1024"]

use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use wasm_logger;
use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen(
    inline_js = "export function invoke_tauri(cmd, args = {}) { return window.__TAURI_INVOKE__(cmd, args=args) }"
)]
extern "C" {
    async fn invoke_tauri(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SetKeyRequest {
    key: String,
}

async fn set_key(key: String) {
    let req = SetKeyRequest{key: key.into()};
    invoke_tauri("set_key", JsValue::from_serde(&req).unwrap()).await;
}

async fn get_totp() -> String {
    let answer = invoke_tauri("get_totp", JsValue::undefined()).await;
    match answer.as_string() {
        Some(s) => s,
        None => "unknown!".into(),
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
    Ok(())
}

type TOTPKey = String;
type TOTP = String;

enum Message {
    SetKey(TOTPKey),
    RequestTOTP,
    UpdateTOTP(TOTP),
}

#[derive(Clone, PartialEq)]
struct Properties {
    pub key: String,
    pub totp: String,
}

struct Model {
    props: Properties,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            props: Properties {
                key: "".into(),
                totp: "".into(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::SetKey(key) => {
                wasm_bindgen_futures::spawn_local(async move {
                    set_key(key).await;
                });
                false
            }
            Message::RequestTOTP => {
                let cb = self.link.callback(Message::UpdateTOTP);
                wasm_bindgen_futures::spawn_local(async move {
                    let totp = get_totp().await;
                    cb.emit(totp);
                });
                false
            }
            Message::UpdateTOTP(totp) => {
                self.props.totp = totp;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let set_key = self.link.callback(|event: FocusEvent| {
            log::info!("FocusEvent: {}", event.detail());
            Message::SetKey("123".into())
        });
        let request_totp = self.link.callback(|event: MouseEvent| {
            log::info!("MouseEvent: {}", event.detail());
            Message::RequestTOTP
        });
        html! {
            <>
            <form id="main-form" onsubmit=set_key >
                <input id="key-input" name="key" />
                <input type="submit" value="set key" />
            </form>
            <p>{format!("Key: {}", self.props.key)}</p>
            <p>{format!("TOTP: {}", self.props.totp)}</p>
            <button onclick=request_totp >{"get totp"}</button>
            </>
        }
    }
}
