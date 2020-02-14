#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Meta::Meta;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A resource that represents the data of a single raw artifact as digital content
/// accessible in its native format.  A Binary resource can contain any content,
/// whether text, image, pdf, zip archive, etc.

#[derive(Debug)]
pub struct Binary<'a> {
    pub value: &'a Value,
}

impl Binary<'_> {
    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element { value: val });
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

    /// MimeType of the binary content represented as a standard MimeType (BCP 13).
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
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

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element { value: val });
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
            return Some(Reference { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._content_type() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.data() {}
        if let Some(_val) = self._data() {
            _val.validate();
        }
        if let Some(_val) = self.security_context() {
            _val.validate();
        }
        return true;
    }
}
