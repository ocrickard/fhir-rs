#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// An occurrence of information being transmitted; e.g. an alert that was sent to a
/// responsible provider, a public health agency that was notified about a
/// reportable condition.

#[derive(Debug)]
pub struct Communication_Payload<'a> {
    pub value: &'a Value,
}

impl Communication_Payload<'_> {
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for contentString
    pub fn _content_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("contentAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// A communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentString") {
            return Some(string);
        }
        return None;
    }

    /// A communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("contentReference") {
            return Some(Reference { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._content_string() {
            _val.validate();
        }
        if let Some(_val) = self.content_attachment() {
            _val.validate();
        }
        if let Some(_val) = self.content_string() {}
        if let Some(_val) = self.content_reference() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}
