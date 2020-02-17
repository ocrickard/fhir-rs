#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The characteristics, operational status and capabilities of a medical-related
/// component of a medical device.

#[derive(Debug)]
pub struct DeviceDefinition_Material<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DeviceDefinition_Material<'_> {
    pub fn new(value: &Value) -> DeviceDefinition_Material {
        DeviceDefinition_Material {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for allergenicIndicator
    pub fn _allergenic_indicator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_allergenicIndicator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for alternate
    pub fn _alternate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_alternate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Whether the substance is a known or suspected allergen.
    pub fn allergenic_indicator(&self) -> Option<bool> {
        if let Some(val) = self.value.get("allergenicIndicator") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates an alternative material of the device.
    pub fn alternate(&self) -> Option<bool> {
        if let Some(val) = self.value.get("alternate") {
            return Some(val.as_bool().unwrap());
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

    /// The substance.
    pub fn substance(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["substance"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._allergenic_indicator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._alternate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.allergenic_indicator() {}
        if let Some(_val) = self.alternate() {}
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
        if !self.substance().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DeviceDefinition_MaterialBuilder {
    pub(crate) value: Value,
}

impl DeviceDefinition_MaterialBuilder {
    pub fn build(&self) -> DeviceDefinition_Material {
        DeviceDefinition_Material {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DeviceDefinition_Material) -> DeviceDefinition_MaterialBuilder {
        DeviceDefinition_MaterialBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(substance: CodeableConcept) -> DeviceDefinition_MaterialBuilder {
        let mut __value: Value = json!({});
        __value["substance"] = json!(substance.value);
        return DeviceDefinition_MaterialBuilder { value: __value };
    }

    pub fn _allergenic_indicator<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["_allergenicIndicator"] = json!(val.value);
        return self;
    }

    pub fn _alternate<'a>(&'a mut self, val: Element) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["_alternate"] = json!(val.value);
        return self;
    }

    pub fn allergenic_indicator<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["allergenicIndicator"] = json!(val);
        return self;
    }

    pub fn alternate<'a>(&'a mut self, val: bool) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["alternate"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DeviceDefinition_MaterialBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
