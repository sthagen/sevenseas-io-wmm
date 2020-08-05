// build.rs

fn main() {
    cc::Build::new()
        .file("lib/Core/Src/wmm.c")
        .file("lib/Core/Src/WMM_COF.c")
        .include("lib/Core/Inc")
        .compile("wmm");
}
