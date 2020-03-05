#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MeasureReport_Stratum::MeasureReport_Stratum;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.

#[derive(Debug)]
pub struct MeasureReport_Stratifier<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MeasureReport_Stratifier<'_> {
    pub fn new(value: &Value) -> MeasureReport_Stratifier {
        MeasureReport_Stratifier {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The meaning of this stratifier, as defined in the measure definition.
    pub fn code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
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

    /// This element contains the results for a single stratum within the stratifier.
    /// For example, when stratifying on administrative gender, there will be four
    /// strata, one for each possible gender value.
    pub fn stratum(&self) -> Option<Vec<MeasureReport_Stratum>> {
        if let Some(Value::Array(val)) = self.value.get("stratum") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Stratum {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.code() {
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
        if let Some(_val) = self.stratum() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MeasureReport_StratifierBuilder {
    pub(crate) value: Value,
}

impl MeasureReport_StratifierBuilder {
    pub fn build(&self) -> MeasureReport_Stratifier {
        MeasureReport_Stratifier {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MeasureReport_Stratifier) -> MeasureReport_StratifierBuilder {
        MeasureReport_StratifierBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MeasureReport_StratifierBuilder {
        let mut __value: Value = json!({});
        return MeasureReport_StratifierBuilder { value: __value };
    }

    pub fn code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MeasureReport_StratifierBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MeasureReport_StratifierBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MeasureReport_StratifierBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MeasureReport_StratifierBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn stratum<'a>(
        &'a mut self,
        val: Vec<MeasureReport_Stratum>,
    ) -> &'a mut MeasureReport_StratifierBuilder {
        self.value["stratum"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
