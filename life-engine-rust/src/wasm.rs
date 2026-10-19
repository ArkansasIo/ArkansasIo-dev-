#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmLifeEngine {
    inner: crate::unified::LifeEngine,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl WasmLifeEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(seed: u64, population: usize) -> Self {
        Self { inner: crate::unified::LifeEngine::new(seed, population) }
    }

    pub fn step(&mut self) { self.inner.step(); }
    pub fn tick(&self) -> u64 { self.inner.tick }
    pub fn population(&self) -> usize { self.inner.population() }
    pub fn births(&self) -> u64 { self.inner.births }
    pub fn deaths(&self) -> u64 { self.inner.deaths }
}
