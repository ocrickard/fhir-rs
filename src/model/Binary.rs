#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A resource that represents the data of a single raw artifact as digital content
/// accessible in its native format.  A Binary resource can contain any content,
/// whether text, image, pdf, zip archive, etc.

#[derive(Debug)]
pub struct Binary<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Binary<'_> {
    pub fn new(value: &Value) -> Binary {
        Binary {
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

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
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

    /// MimeType of the binary content represented as a standard MimeType (BCP 13).
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// The actual content, base64 encoded.
    pub fn data(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("data") {
            return Some(string);
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This element identifies another resource that can be used as a proxy of the
    /// security sensitivity to use when deciding and enforcing access control rules for
    /// the Binary resource. Given that the Binary resource contains very few elements
    /// that can be used to determine the sensitivity of the data and relationships to
    /// individuals, the referenced resource stands in as a proxy equivalent for this
    /// purpose. This referenced resource may be related to the Binary (e.g. Media,
    /// DocumentReference), or may be some non-related Resource purely as a security
    /// proxy. E.g. to identify that the binary resource relates to a patient, and
    /// access should only be granted to applications that have access to the patient.
    pub fn security_context(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("securityContext") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._content_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.data() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.security_context() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BinaryBuilder {
    pub(crate) value: Value,
}

impl BinaryBuilder {
    pub fn build(&self) -> Binary {
        Binary {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Binary) -> BinaryBuilder {
        BinaryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BinaryBuilder {
        let mut __value: Value = json!({});
        return BinaryBuilder { value: __value };
    }

    pub fn _content_type<'a>(&'a mut self, val: Element) -> &'a mut BinaryBuilder {
        self.value["_contentType"] = json!(val.value);
        return self;
    }

    pub fn _data<'a>(&'a mut self, val: Element) -> &'a mut BinaryBuilder {
        self.value["_data"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut BinaryBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut BinaryBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn content_type<'a>(&'a mut self, val: &str) -> &'a mut BinaryBuilder {
        self.value["contentType"] = json!(val);
        return self;
    }

    pub fn data<'a>(&'a mut self, val: &str) -> &'a mut BinaryBuilder {
        self.value["data"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BinaryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut BinaryBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut BinaryBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut BinaryBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn security_context<'a>(&'a mut self, val: Reference) -> &'a mut BinaryBuilder {
        self.value["securityContext"] = json!(val.value);
        return self;
    }
}
