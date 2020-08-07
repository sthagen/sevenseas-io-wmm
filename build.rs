// build.rs

fn main() {
    cc::Build::new()
        .file("lib/wmm.c")
        .file("lib/WMM_COF.c")
        .compile("wmm");
}
