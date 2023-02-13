#[derive(Clone)]
pub struct FreeTag {
	pub name: String,
	pub version: String,
	pub block_type: String,
	pub contents: String
}

impl FreeTag {
	pub fn new() -> FreeTag {
		FreeTag {
			name: "Untitled Tag".to_string(),
			version: "".to_string(),
			block_type: "".to_string(),
			contents: "".to_string()
		}
	}
}
