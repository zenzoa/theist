extern crate embed_resource;

fn main() {
	// Compile and link icon-resource.rc
	embed_resource::compile("icon-resource.rc");
}