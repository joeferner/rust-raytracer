pub trait Random: Send + Sync {
    fn rand(&self) -> f64;
    fn rand_int_interval(&self, min: i64, max: i64) -> i64;
    fn rand_interval(&self, min: f64, max: f64) -> f64;
}

#[cfg(not(target_arch = "wasm32"))]
pub fn random_new() -> impl Random {
    use crate::random::rand::RandRandom;

    RandRandom::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub mod rand {
    use crate::Random;

    pub struct RandRandom {}

    impl RandRandom {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Random for RandRandom {
        fn rand(&self) -> f64 {
            rand::random()
        }

        fn rand_interval(&self, min: f64, max: f64) -> f64 {
            rand::random_range(min..max)
        }

        fn rand_int_interval(&self, min: i64, max: i64) -> i64 {
            rand::random_range(min..max)
        }
    }

    impl Default for RandRandom {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn random_new() -> impl Random {
    use crate::random::wasm::WasmRandom;

    WasmRandom::new()
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use crate::Random;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Math)]
        fn random() -> f64;
    }

    pub struct WasmRandom {}

    impl WasmRandom {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Random for WasmRandom {
        fn rand(&self) -> f64 {
            random()
        }

        fn rand_interval(&self, min: f64, max: f64) -> f64 {
            let delta = max - min;
            (random() * delta) + min
        }

        fn rand_int_interval(&self, min: i64, max: i64) -> i64 {
            let delta = max - min + 1; // inclusive range
            (self.rand() * delta as f64).floor() as i64 + min
        }
    }

    impl Default for WasmRandom {
        fn default() -> Self {
            Self::new()
        }
    }
}
