#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// For referring to data content defined in other formats.

#[derive(Debug)]
pub struct Attachment<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Attachment<'_> {
    pub fn new(value: &Value) -> Attachment {
        Attachment {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for creation
    pub fn _creation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_creation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for hash
    pub fn _hash(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_hash") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for size
    pub fn _size(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_size") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// The date that the attachment was first created.
    pub fn creation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("creation") {
            return Some(string);
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// The number of bytes of data that make up this attachment (before base64
    /// encoding, if that is done).
    pub fn size(&self) -> Option<u64> {
        if let Some(val) = self.value.get("size") {
            return Some(val.as_u64().unwrap());
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

    /// A location where the data can be accessed.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._content_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._creation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._hash() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._size() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.creation() {}
        if let Some(_val) = self.data() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.hash() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.size() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.url() {}
        return true;
    }
}

#[derive(Debug)]
pub struct AttachmentBuilder {
    pub(crate) value: Value,
}

impl AttachmentBuilder {
    pub fn build(&self) -> Attachment {
        Attachment {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Attachment) -> AttachmentBuilder {
        AttachmentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> AttachmentBuilder {
        let mut __value: Value = json!({});
        return AttachmentBuilder { value: __value };
    }

    pub fn _content_type<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_contentType"] = json!(val.value);
        return self;
    }

    pub fn _creation<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_creation"] = json!(val.value);
        return self;
    }

    pub fn _data<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_data"] = json!(val.value);
        return self;
    }

    pub fn _hash<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_hash"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _size<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_size"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut AttachmentBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn content_type<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["contentType"] = json!(val);
        return self;
    }

    pub fn creation<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["creation"] = json!(val);
        return self;
    }

    pub fn data<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["data"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AttachmentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn hash<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["hash"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn size<'a>(&'a mut self, val: u64) -> &'a mut AttachmentBuilder {
        self.value["size"] = json!(val);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut AttachmentBuilder {
        self.value["url"] = json!(val);
        return self;
    }
}
