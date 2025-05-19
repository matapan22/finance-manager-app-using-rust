fn main() {
    glib_build_tools::compile_resources(
        &["src/resources"],                   // Path to the resources folder
        "src/resources/resources.gresource.xml", // Path to the XML file
        "resources.gresource",                // Name of the output file
    );
}
