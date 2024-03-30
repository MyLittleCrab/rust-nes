fn main() {
    cc::Build::new()
        .compiler("clang")
        .target("mos-nes")
        .file("nes/src/nmi.c")
        .compile("nmi");
}
