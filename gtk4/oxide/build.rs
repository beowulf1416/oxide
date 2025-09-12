fn main() {
    glib_build_tools::compile_resources(
        &["assets/ui"],
        "assets/resources.gresource.xml",
        "org.devphilplus.oxide.gresource",
    );
}