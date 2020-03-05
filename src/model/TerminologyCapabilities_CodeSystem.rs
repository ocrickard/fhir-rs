#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TerminologyCapabilities_Version::TerminologyCapabilities_Version;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_CodeSystem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TerminologyCapabilities_CodeSystem<'_> {
    pub fn new(value: &Value) -> TerminologyCapabilities_CodeSystem {
        TerminologyCapabilities_CodeSystem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for subsumption
    pub fn _subsumption(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subsumption") {
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

    /// True if subsumption is supported for this version of the code system.
    pub fn subsumption(&self) -> Option<bool> {
        if let Some(val) = self.value.get("subsumption") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// URI for the Code System.
    pub fn uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uri") {
            return Some(string);
        }
        return None;
    }

    /// For the code system, a list of versions that are supported by the server.
    pub fn version(&self) -> Option<Vec<TerminologyCapabilities_Version>> {
        if let Some(Value::Array(val)) = self.value.get("version") {
            return Some(
                val.into_iter()
                    .map(|e| TerminologyCapabilities_Version {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._subsumption() {
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
        if let Some(_val) = self.subsumption() {}
        if let Some(_val) = self.uri() {}
        if let Some(_val) = self.version() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TerminologyCapabilities_CodeSystemBuilder {
    pub(crate) value: Value,
}

impl TerminologyCapabilities_CodeSystemBuilder {
    pub fn build(&self) -> TerminologyCapabilities_CodeSystem {
        TerminologyCapabilities_CodeSystem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: TerminologyCapabilities_CodeSystem,
    ) -> TerminologyCapabilities_CodeSystemBuilder {
        TerminologyCapabilities_CodeSystemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TerminologyCapabilities_CodeSystemBuilder {
        let mut __value: Value = json!({});
        return TerminologyCapabilities_CodeSystemBuilder { value: __value };
    }

    pub fn _subsumption<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["_subsumption"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subsumption<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["subsumption"] = json!(val);
        return self;
    }

    pub fn uri<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["uri"] = json!(val);
        return self;
    }

    pub fn version<'a>(
        &'a mut self,
        val: Vec<TerminologyCapabilities_Version>,
    ) -> &'a mut TerminologyCapabilities_CodeSystemBuilder {
        self.value["version"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
