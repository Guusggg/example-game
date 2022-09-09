extern crate cc;

fn main() {
    let files = ["src/my_game.c", "src/wasm-rt-impl.c"];

    for file in files {
        println!("cargo:rerun-if-changed={}", file);
    }

    println!("cargo:rerun-if-env-changed=TARGET");

    cc::Build::new()
        .static_flag(true)
        .warnings(false)
        .include("./emsdk/upstream/emscripten/system/**")
        .files(files)
        .compile("client");
}
