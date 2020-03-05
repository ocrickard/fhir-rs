#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An action that is or was performed on or for a patient. This can be a physical
/// intervention like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.

#[derive(Debug)]
pub struct Procedure_FocalDevice<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Procedure_FocalDevice<'_> {
    pub fn new(value: &Value) -> Procedure_FocalDevice {
        Procedure_FocalDevice {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The kind of change that happened to the device during the procedure.
    pub fn action(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("action") {
            return Some(CodeableConcept {
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

    /// The device that was manipulated (changed) during the procedure.
    pub fn manipulated(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["manipulated"]),
        }
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
        if let Some(_val) = self.action() {
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
        if !self.manipulated().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Procedure_FocalDeviceBuilder {
    pub(crate) value: Value,
}

impl Procedure_FocalDeviceBuilder {
    pub fn build(&self) -> Procedure_FocalDevice {
        Procedure_FocalDevice {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Procedure_FocalDevice) -> Procedure_FocalDeviceBuilder {
        Procedure_FocalDeviceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(manipulated: Reference) -> Procedure_FocalDeviceBuilder {
        let mut __value: Value = json!({});
        __value["manipulated"] = json!(manipulated.value);
        return Procedure_FocalDeviceBuilder { value: __value };
    }

    pub fn action<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Procedure_FocalDeviceBuilder {
        self.value["action"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Procedure_FocalDeviceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Procedure_FocalDeviceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Procedure_FocalDeviceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
