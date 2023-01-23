use regex::Regex;

fn get_captures(filename: &str) -> Option<regex::Captures> {
	let filename_pattern = Regex::new(r"(.*[\\/])?([^\.]+)(\..*)?").unwrap();
	filename_pattern.captures(filename)
}

pub fn path(filename: &str) -> String {
	if let Some(captures) = get_captures(filename) {
		if let Some(path) = captures.get(1) {
			return path.as_str().to_string();
		}
	}
	"".to_string()
}

pub fn title(filename: &str) -> String {
	if let Some(captures) = get_captures(filename) {
		if let Some(title) = captures.get(2) {
			return title.as_str().to_string();
		}
	}
	"".to_string()
}

pub fn extension(filename: &str) -> String {
	if let Some(captures) = get_captures(filename) {
		if let Some(extension) = captures.get(3) {
			return extension.as_str()[1..].to_string();
		}
	}
	"".to_string()
}

pub fn filename(filename: &str) -> String {
	if let Some(captures) = get_captures(filename) {
		if let Some(title) = captures.get(2) {
			if let Some(path) = captures.get(3) {
				return format!("{}{}", title.as_str(), path.as_str());
			}
		}
	}
	"".to_string()
}
