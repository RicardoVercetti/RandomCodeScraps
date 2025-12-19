use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    
    alert(&format!("Hello, {}!", name));
}

// commands
// wasm-pack build --target web                 →   compile straight for web
// wasm-pack build --target bundler             →   as a package available to npm