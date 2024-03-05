use cmake::Config;
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
    let lib_out = build.build();
    println!(
        "cargo:rustc-link-search=native={}",
        lib_out.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=volk");
    println!("cargo:rustc-link-lib=static=memonitor-vk");
}
