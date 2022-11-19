use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hi(name: &str) {
  println!("Hi there {}!", name);
}
