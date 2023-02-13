use iced::widget::{ text, Text };
use iced::{ Font, alignment, Length };

const ICONS: Font = Font::External {
	name: "Icons",
	bytes: include_bytes!("../../fonts/icons.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
	text(unicode.to_string())
		.font(ICONS)
		.width(Length::Units(20))
		.horizontal_alignment(alignment::Horizontal::Center)
		.size(20)
}

pub fn new_icon() -> Text<'static> {
	icon('\u{F144}')
}

pub fn open_icon() -> Text<'static> {
	icon('\u{F15D}')
}

pub fn save_icon() -> Text<'static> {
	icon('\u{F188}')
}

pub fn compile_icon() -> Text<'static> {
	icon('\u{F17F}')
}

pub fn add_icon() -> Text<'static> {
	icon('\u{F101}')
}

pub fn delete_icon() -> Text<'static> {
	icon('\u{F140}')
}

pub fn up_icon() -> Text<'static> {
	icon('\u{F10A}')
}

pub fn down_icon() -> Text<'static> {
	icon('\u{F103}')
}

pub fn script_icon() -> Text<'static> {
	icon('\u{F146}')
}

pub fn sprite_icon() -> Text<'static> {
	icon('\u{F161}')
}

pub fn sound_icon() -> Text<'static> {
	icon('\u{F1AE}')
}

pub fn catalogue_icon() -> Text<'static> {
	icon('\u{F112}')
}

pub fn genetics_icon() -> Text<'static> {
	icon('\u{F168}')
}

pub fn bodydata_icon() -> Text<'static> {
	icon('\u{F19D}')
}

pub fn alert_icon() -> Text<'static> {
	icon('\u{F129}')
}

pub fn error_icon() -> Text<'static> {
	icon('\u{F12E}')
}

pub fn dismiss_icon() -> Text<'static> {
	icon('\u{F134}')
}
