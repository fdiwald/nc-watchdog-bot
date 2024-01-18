use frankenstein::{MessageEntity, MessageEntityType};

pub(crate) struct Hypertext {
    pub kind: Option<MessageEntityType>,
    pub text: String,
    pub url: Option<String>,
}

impl Hypertext {
    pub(crate) fn text<T>(text: T) -> Hypertext
    where
        T: Into<String>,
    {
        Hypertext {
            kind: None,
            text: text.into(),
            url: None,
        }
    }

    pub(crate) fn bold<T>(text: T) -> Hypertext
    where
        T: Into<String>,
    {
        Hypertext {
            kind: Some(MessageEntityType::Bold),
            text: text.into(),
            url: None,
        }
    }
}

pub(crate) fn compile_hypertext_elements(
    hypertexts: Vec<Hypertext>,
) -> (String, Vec<MessageEntity>) {
    let mut offset = 0u16;
    let mut message = String::new();
    let mut entities = vec![];
    for hypertext in hypertexts {
        message.push_str(&hypertext.text);
        let length = hypertext.text.encode_utf16().count() as u16;

        if let Some(kind) = hypertext.kind {
            entities.push(MessageEntity {
                type_field: kind,
                offset,
                length,
                url: hypertext.url,
                user: None,
                language: None,
                custom_emoji_id: None,
            });
        }
        offset += length
    }

    (message, entities)
}

impl<T> From<T> for Hypertext where T: Into<String> {
    fn from(value: T) -> Self {
        Hypertext::text(value)
    }
}