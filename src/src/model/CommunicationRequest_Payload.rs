#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to convey information; e.g. the CDS system proposes that an alert be
/// sent to a responsible provider, the CDS system proposes that the public health
/// agency be notified about a reportable condition.

#[derive(Debug)]
pub struct CommunicationRequest_Payload<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CommunicationRequest_Payload<'_> {
    pub fn new(value: &Value) -> CommunicationRequest_Payload {
        CommunicationRequest_Payload {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for contentString
    pub fn _content_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("contentAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("contentReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The communicated content (or for multi-part communications, one portion of the
    /// communication).
    pub fn content_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentString") {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._content_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_string() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CommunicationRequest_PayloadBuilder {
    pub(crate) value: Value,
}

impl CommunicationRequest_PayloadBuilder {
    pub fn build(&self) -> CommunicationRequest_Payload {
        CommunicationRequest_Payload {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CommunicationRequest_Payload) -> CommunicationRequest_PayloadBuilder {
        CommunicationRequest_PayloadBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CommunicationRequest_PayloadBuilder {
        let mut __value: Value = json!({});
        return CommunicationRequest_PayloadBuilder { value: __value };
    }

    pub fn _content_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["_contentString"] = json!(val.value);
        return self;
    }

    pub fn content_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["contentAttachment"] = json!(val.value);
        return self;
    }

    pub fn content_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["contentReference"] = json!(val.value);
        return self;
    }

    pub fn content_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["contentString"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CommunicationRequest_PayloadBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
