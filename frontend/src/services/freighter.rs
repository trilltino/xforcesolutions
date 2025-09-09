use js_sys::{Function, Promise, Reflect};
use std::fmt;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;

#[derive(Debug, Clone)]
pub enum FreighterError {
    FreighterExtNotFound,
    NotAFunction(String),
    JsExecutionError(String),
    NoWindow,
    UserRejected,
}


impl fmt::Display for FreighterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FreighterError::FreighterExtNotFound => {
                write!(f, "Freighter wallet extension not found. Install from https://freighter.app/")
            }
            FreighterError::NotAFunction(method) => {
                write!(f, "Property {method} exists but is not a callable function")
            }
            FreighterError::JsExecutionError(msg) => {
                write!(f, "JavaScript error: {msg}")
            }
            FreighterError::NoWindow => write!(f, "Window object not available"),
            FreighterError::UserRejected => write!(f, "User rejected the connection request"),
        }
    }
}

impl std::error::Error for FreighterError {}

impl From<JsValue> for FreighterError {
    fn from(js_val: JsValue) -> Self {
        if let Some(error_msg) = js_val.as_string() {
            if error_msg.to_lowercase().contains("user") && error_msg.to_lowercase().contains("reject") {
                FreighterError::UserRejected
            } else if error_msg.contains("freighter") || error_msg.contains("not found") {
                FreighterError::FreighterExtNotFound
            } else {
                FreighterError::JsExecutionError(error_msg)
            }
        } else {
            FreighterError::JsExecutionError("Unknown JavaScript error".to_string())
        }
    }
}

fn get_freighter_api() -> Result<JsValue, FreighterError> {
    let window = window().ok_or(FreighterError::NoWindow)?;
    let api = Reflect::get(window.as_ref(), &JsValue::from_str("freighterApi"))?;
    if api.is_undefined() || api.is_null() {
        return Err(FreighterError::FreighterExtNotFound);
    }
    Ok(api)
}

pub fn is_freighter_available() -> bool {
    get_freighter_api().is_ok()
}

pub async fn connect_wallet() -> Result<String, FreighterError> {
   web_sys::console::log_1(&JsValue::from_str(" Starting Freighter connection..."));
   let api = get_freighter_api()?;

   web_sys::console::log_1(&JsValue::from_str("Requesting access..."));

   let request_access_method = Reflect::get(&api, &JsValue::from_str("requestAccess"))?;

   if request_access_method.is_function() {
       let function = request_access_method.dyn_into::<Function>()?;
       let promise = function.call0(&api)?;
       let promise = promise.dyn_into::<Promise>()?;
       match JsFuture::from(promise).await {
           Ok(_) => web_sys::console::log_1(&JsValue::from_str("Access granted!")),
           Err(e) => {
               web_sys::console::log_1(&JsValue::from_str("Access denied"));
               return Err(FreighterError::from(e));
           }
       }
   }
   


   web_sys::console::log_1(&JsValue::from_str("Getting public key..."));
   let method_names = ["getPublicKey", "getUserInfo", "getAddress"];
   let mut get_public_key_method = JsValue::undefined();

   for method_name in &method_names {
       let method = Reflect::get(&api, &JsValue::from_str(method_name))?;
      
       if method.is_function() {
           web_sys::console::log_1(&JsValue::from_str(&format!("Found working method: {}", method_name)));
           get_public_key_method = method;
           break;
       }
   }

   if get_public_key_method.is_undefined() {
       return Err(FreighterError::NotAFunction("No valid method found".to_string()));
   }
   
   let function = get_public_key_method.dyn_into::<Function>()?;
   let promise = function.call0(&api)?;
   let promise = promise.dyn_into::<Promise>()?;
   
    match JsFuture::from(promise).await {
    Ok(result) => {
        if let Some(public_key) = result.as_string() {
            web_sys::console::log_1(&JsValue::from_str(&format!(" Got key: {}", public_key)));
            Ok(public_key)
        } 

        else if let Ok(obj) = result.clone().dyn_into::<js_sys::Object>() {
            if let Ok(address) = Reflect::get(&obj, &JsValue::from_str("address")) {
                if let Some(address_str) = address.as_string() {
                    web_sys::console::log_1(&JsValue::from_str(&format!("Got address: {}", address_str)));
                    Ok(address_str)
                } else {
                    web_sys::console::log_1(&JsValue::from_str(&format!("Address not string: {:?}", address)));
                    Err(FreighterError::JsExecutionError("Address property is not a string".to_string()))
                }
            } else {
                web_sys::console::log_1(&JsValue::from_str(&format!("No address property found: {:?}", result)));
                Err(FreighterError::JsExecutionError("No address property in result".to_string()))
            }
        } else {
            web_sys::console::log_1(&JsValue::from_str(&format!("Unknown result type: {:?}", result)));
            Err(FreighterError::JsExecutionError("getPublicKey returned unknown format".to_string()))
        }
    },
    Err(e) => {
        web_sys::console::log_1(&JsValue::from_str("getPublicKey failed"));
        web_sys::console::log_1(&e);
        Err(FreighterError::from(e))
    }
}}




#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "freighterApi"])]
    fn getPublicKey() -> js_sys::Promise;
}


pub async fn get_public_key() -> Result<String, FreighterError> {
    let promise = getPublicKey();
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    
    match result {
        Ok(value) => {
            let public_key = value.as_string()
                .ok_or(FreighterError::FreighterExtNotFound)?;
            Ok(public_key)
        }
        Err(_) => Err(FreighterError::UserRejected)
    }
}