mod utils;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn find_hash_origin(major_max: JsValue, minor_max: JsValue, patch_max: JsValue, build_max: JsValue, hash: JsValue) -> JsValue {
    let major_max = major_max.as_f64().expect("major_max must be a valid u32") as u32;
    let minor_max = minor_max.as_f64().expect("minor_max must be a valid u32") as u32;
    let patch_max = patch_max.as_f64().expect("patch_max must be a valid u32") as u32;
    let build_max = build_max.as_f64().expect("build_max must be a valid u32") as u32;

    let hash = hash.as_f64().expect("hash must be a valid i32") as i32;

    for major in 1..=major_max {
        for minor in 0..=minor_max {
            for patch in 0..=patch_max {
                for build in 0..=build_max {
                    let res = utils::hash_version(major, minor, patch, build);

                    if res == hash {
                        println!("Found with v8 {}.{}.{}.{} with hash 0x{:08X}", major, minor, patch, build, res);
                        return JsValue::from(format!("{}.{}.{}.{}", major, minor, patch, build));
                    }
                }
            }
        }
    }

    JsValue::NULL
}
