#![allow(unused_imports, non_camel_case_types)]

use crate::model::CapabilityStatement_Endpoint::CapabilityStatement_Endpoint;
use crate::model::CapabilityStatement_SupportedMessage::CapabilityStatement_SupportedMessage;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_Messaging<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement_Messaging<'_> {
    pub fn new(value: &Value) -> CapabilityStatement_Messaging {
        CapabilityStatement_Messaging {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reliableCache
    pub fn _reliable_cache(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reliableCache") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Documentation about the system's messaging capabilities for this endpoint not
    /// otherwise documented by the capability statement.  For example, the process for
    /// becoming an authorized messaging exchange partner.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
        }
        return None;
    }

    /// An endpoint (network accessible address) to which messages and/or replies are to
    /// be sent.
    pub fn endpoint(&self) -> Option<Vec<CapabilityStatement_Endpoint>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Endpoint {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Length if the receiver's reliable messaging cache in minutes (if a receiver) or
    /// how long the cache length on the receiver should be (if a sender).
    pub fn reliable_cache(&self) -> Option<u64> {
        if let Some(val) = self.value.get("reliableCache") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// References to message definitions for messages this system can send or receive.
    pub fn supported_message(&self) -> Option<Vec<CapabilityStatement_SupportedMessage>> {
        if let Some(Value::Array(val)) = self.value.get("supportedMessage") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_SupportedMessage {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._documentation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reliable_cache() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
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
        if let Some(_val) = self.reliable_cache() {}
        if let Some(_val) = self.supported_message() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement_MessagingBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement_MessagingBuilder {
    pub fn build(&self) -> CapabilityStatement_Messaging {
        CapabilityStatement_Messaging {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement_Messaging) -> CapabilityStatement_MessagingBuilder {
        CapabilityStatement_MessagingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CapabilityStatement_MessagingBuilder {
        let mut __value: Value = json!({});
        return CapabilityStatement_MessagingBuilder { value: __value };
    }

    pub fn _documentation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["_documentation"] = json!(val.value);
        return self;
    }

    pub fn _reliable_cache<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["_reliableCache"] = json!(val.value);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["documentation"] = json!(val);
        return self;
    }

    pub fn endpoint<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement_Endpoint>,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reliable_cache<'a>(
        &'a mut self,
        val: u64,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["reliableCache"] = json!(val);
        return self;
    }

    pub fn supported_message<'a>(
        &'a mut self,
        val: Vec<CapabilityStatement_SupportedMessage>,
    ) -> &'a mut CapabilityStatement_MessagingBuilder {
        self.value["supportedMessage"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
