#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A TerminologyCapabilities resource documents a set of capabilities (behaviors)
/// of a FHIR Terminology Server that may be used as a statement of actual server
/// functionality or a statement of required or desired server implementation.

#[derive(Debug)]
pub struct TerminologyCapabilities_ValidateCode<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TerminologyCapabilities_ValidateCode<'_> {
    pub fn new(value: &Value) -> TerminologyCapabilities_ValidateCode {
        TerminologyCapabilities_ValidateCode {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for translations
    pub fn _translations(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_translations") {
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

    /// Whether translations are validated.
    pub fn translations(&self) -> Option<bool> {
        if let Some(val) = self.value.get("translations") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._translations() {
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
        if let Some(_val) = self.translations() {}
        return true;
    }
}

#[derive(Debug)]
pub struct TerminologyCapabilities_ValidateCodeBuilder {
    pub(crate) value: Value,
}

impl TerminologyCapabilities_ValidateCodeBuilder {
    pub fn build(&self) -> TerminologyCapabilities_ValidateCode {
        TerminologyCapabilities_ValidateCode {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: TerminologyCapabilities_ValidateCode,
    ) -> TerminologyCapabilities_ValidateCodeBuilder {
        TerminologyCapabilities_ValidateCodeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> TerminologyCapabilities_ValidateCodeBuilder {
        let mut __value: Value = json!({});
        return TerminologyCapabilities_ValidateCodeBuilder { value: __value };
    }

    pub fn _translations<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut TerminologyCapabilities_ValidateCodeBuilder {
        self.value["_translations"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_ValidateCodeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TerminologyCapabilities_ValidateCodeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut TerminologyCapabilities_ValidateCodeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn translations<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut TerminologyCapabilities_ValidateCodeBuilder {
        self.value["translations"] = json!(val);
        return self;
    }
}
