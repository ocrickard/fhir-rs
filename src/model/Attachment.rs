#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// For referring to data content defined in other formats.

#[derive(Debug)]
pub struct Attachment<'a> {
    pub value: &'a Value,
}

impl Attachment<'_> {
    /// The date that the attachment was first created.
    pub fn creation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("creation") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for creation
    pub fn _creation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_creation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for hash
    pub fn _hash(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hash") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A label or set of text to display in place of the data.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// The number of bytes of data that make up this attachment (before base64
    /// encoding, if that is done).
    pub fn size(&self) -> Option<u64> {
        if let Some(val) = self.value.get("size") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Extensions for size
    pub fn _size(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_size") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The actual data of the attachment - a sequence of bytes, base64 encoded.
    pub fn data(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("data") {
            return Some(string);
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A location where the data can be accessed.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The calculated hash of the data using SHA-1. Represented using base64.
    pub fn hash(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("hash") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies the type of the data in the attachment and allows a method to be
    /// chosen to interpret or render the data. Includes mime type parameters such as
    /// charset where appropriate.
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The human language of the content. The value can be any valid value according to
    /// BCP 47.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.creation() {}
        if let Some(_val) = self._creation() {
            _val.validate();
        }
        if let Some(_val) = self._hash() {
            _val.validate();
        }
        if let Some(_val) = self._content_type() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.size() {}
        if let Some(_val) = self._size() {
            _val.validate();
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.data() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.hash() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._data() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        return true;
    }
}
