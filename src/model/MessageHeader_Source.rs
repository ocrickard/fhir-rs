#![allow(unused_imports, non_camel_case_types)]

use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// The header for a message exchange that is either requesting or responding to an
/// action.  The reference(s) that are the subject of the action as well as other
/// information related to the action are typically transmitted in a bundle in which
/// the MessageHeader resource instance is the first resource in the bundle.

#[derive(Debug)]
pub struct MessageHeader_Source<'a> {
    pub value: &'a Value,
}

impl MessageHeader_Source<'_> {
    /// Human-readable name for the source system.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the routing target to send acknowledgements to.
    pub fn endpoint(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("endpoint") {
            return Some(string);
        }
        return None;
    }

    /// Can convey versions of multiple systems in situations where a message passes
    /// through multiple hands.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
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

    /// May include configuration or other information useful in debugging.
    pub fn software(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("software") {
            return Some(string);
        }
        return None;
    }

    /// An e-mail, phone, website or other contact point to use to resolve issues with
    /// message communications.
    pub fn contact(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("contact") {
            return Some(ContactPoint { value: val });
        }
        return None;
    }

    /// Extensions for software
    pub fn _software(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_software") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
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

    /// Extensions for endpoint
    pub fn _endpoint(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_endpoint") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.endpoint() {}
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.software() {}
        if let Some(_val) = self.contact() {
            _val.validate();
        }
        if let Some(_val) = self._software() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._endpoint() {
            _val.validate();
        }
        return true;
    }
}
