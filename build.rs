fn main() {
    cynic_codegen::register_schema("kotkowo")
        .from_sdl_file("schemas/kotkowo.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
