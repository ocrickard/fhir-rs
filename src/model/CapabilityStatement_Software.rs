#![allow(unused_imports, non_camel_case_types)]

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
pub struct CapabilityStatement_Software<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CapabilityStatement_Software<'_> {
    pub fn new(value: &Value) -> CapabilityStatement_Software {
        CapabilityStatement_Software {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for releaseDate
    pub fn _release_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_releaseDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Name the software is known by.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Date this version of the software was released.
    pub fn release_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("releaseDate") {
            return Some(string);
        }
        return None;
    }

    /// The version identifier for the software covered by this statement.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._release_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.release_date() {}
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct CapabilityStatement_SoftwareBuilder {
    pub(crate) value: Value,
}

impl CapabilityStatement_SoftwareBuilder {
    pub fn build(&self) -> CapabilityStatement_Software {
        CapabilityStatement_Software {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CapabilityStatement_Software) -> CapabilityStatement_SoftwareBuilder {
        CapabilityStatement_SoftwareBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CapabilityStatement_SoftwareBuilder {
        let mut __value: Value = json!({});
        return CapabilityStatement_SoftwareBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _release_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["_releaseDate"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn release_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["releaseDate"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut CapabilityStatement_SoftwareBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
