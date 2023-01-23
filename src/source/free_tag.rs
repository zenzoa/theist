use super::decode::parse_tokens;
use crate::agent::free_tag::FreeTag;

pub fn encode(tag: &FreeTag) -> String {
	let mut content = String::new();

	content.push_str(&format!("other \"{}\" {}\n", &tag.name, &tag.block_type));

	if !tag.version.is_empty() {
		content.push_str(&format!("\tversion \"{}\"\n", &tag.version));
	}

	content.push_str("\"\"\"");
	content.push_str(&tag.contents);
	content.push_str("\"\"\"");

	content.push('\n');

	content
}

pub fn decode(lines: Vec<&str>, name: String, block_type: String) -> (FreeTag, usize) {
	let mut version = String::new();
	let mut contents = String::new();
	let mut contents_started = false;

	let mut i = 1;
	while i < lines.len() {
		if contents_started {
			if lines[i].trim() == "\"\"\"" {
				break;
			} else {
				contents.push_str(lines[i]);
				contents.push('\n');
			}

		} else if lines[i].trim() == "\"\"\"" {
				contents_started = true;

		} else {
			let tokens = parse_tokens(lines[i]);
			if let Some(token) = tokens.get(0) {
				match token.as_str() {
					"version" => {
						if let Some(value) = tokens.get(1) {
							version = value.to_string();
						}
					},
					_ => { break; }
				}
			} else {
				break;
			}
		}

		i += 1;
	}

	(FreeTag {
		name,
		version,
		block_type,
		contents
	}, i)
}
