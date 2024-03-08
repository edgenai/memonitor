use bindgen::callbacks::ParseCallbacks;
use bindgen::{Builder, EnumVariation};
use cmake::Config;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

fn main() {
    let cur_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let vk_dir = cur_dir.join("vulkan");
    let submodules_dir = vk_dir.join("thirdparty");
    read_dir(submodules_dir.join("Vulkan-Headers"))
        .expect("Could not find Vulkan Headers. Did you forget to initialize submodules?");
    read_dir(submodules_dir.join("volk"))
        .expect("Could not find Volk. Did you forget to initialize submodules?");
    let mut build = Config::new(vk_dir.as_path());

    #[cfg(debug_assertions)]
    {
        build.define("MEMONITOR_VALIDATE", "ON");
    }

    let lib_out = build.build();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_out.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=volk");
    println!("cargo:rustc-link-lib=static=memonitor-vk");

    let vk_bindings = Builder::default()
        .header(vk_dir.join("include").join("memonitor.h").to_string_lossy())
        .allowlist_function("vk_.*")
        .allowlist_type("vk_.*")
        .parse_callbacks(Box::new(PrefixRemover::new("vk_")))
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .use_core()
        .generate()
        .expect("Failed to generate Vulkan bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    vk_bindings
        .write_to_file(out_path.join("vk_bindings.rs"))
        .expect("Couldn't write bindings");
}

#[derive(Debug)]
struct PrefixRemover {
    prefix: String,
}

impl PrefixRemover {
    fn new(prefix: impl ToString) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl ParseCallbacks for PrefixRemover {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        original_item_name
            .strip_prefix(&self.prefix)
            .map(move |s| s.to_string())
    }
}
