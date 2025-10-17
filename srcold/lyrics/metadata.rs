#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagType {
	Title,
	Artist,
	Album,
	Author,
	Lyricist,
	Length,
	By,
	Offset,
	Tool,
	Version,
	Misc(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metadata {
	tag_type: TagType,
	value: String,
}

impl Metadata {
	pub fn parse_separate(tag_type: &str, value: &str) -> Self {
		Self {
			tag_type: Self::parse_tag_type(tag_type),
			value: value.trim().to_owned(),
		}
	}

	fn parse_tag_type(s: &str) -> TagType {
		match s.trim() {
			"ti" => TagType::Title,
			"ar" => TagType::Artist,
			"al" => TagType::Album,
			"au" => TagType::Author,
			"lr" => TagType::Lyricist,
			"length" => TagType::Length,
			"by" => TagType::By,
			"offset" => TagType::Offset,
			"re" | "tool" => TagType::Tool,
			"ve" => TagType::Version,
			s => TagType::Misc(s.to_owned()),
		}
	}
}
