#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::MeasureReport_Population::MeasureReport_Population;
use crate::model::MeasureReport_Stratifier::MeasureReport_Stratifier;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.

#[derive(Debug)]
pub struct MeasureReport_Group<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MeasureReport_Group<'_> {
    pub fn new(value: &Value) -> MeasureReport_Group {
        MeasureReport_Group {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The meaning of the population group as defined in the measure definition.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
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

    /// The measure score for this population group, calculated as appropriate for the
    /// measure type and scoring method, and based on the contents of the populations
    /// defined in the group.
    pub fn measure_score(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("measureScore") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
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

    /// The populations that make up the population group, one for each type of
    /// population appropriate for the measure.
    pub fn population(&self) -> Option<Vec<MeasureReport_Population>> {
        if let Some(Value::Array(val)) = self.value.get("population") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Population {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When a measure includes multiple stratifiers, there will be a stratifier group
    /// for each stratifier defined by the measure.
    pub fn stratifier(&self) -> Option<Vec<MeasureReport_Stratifier>> {
        if let Some(Value::Array(val)) = self.value.get("stratifier") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Stratifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.code() {
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
        if let Some(_val) = self.measure_score() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.population() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.stratifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MeasureReport_GroupBuilder {
    pub(crate) value: Value,
}

impl MeasureReport_GroupBuilder {
    pub fn build(&self) -> MeasureReport_Group {
        MeasureReport_Group {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MeasureReport_Group) -> MeasureReport_GroupBuilder {
        MeasureReport_GroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MeasureReport_GroupBuilder {
        let mut __value: Value = json!({});
        return MeasureReport_GroupBuilder { value: __value };
    }

    pub fn code<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MeasureReport_GroupBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MeasureReport_GroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MeasureReport_GroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn measure_score<'a>(&'a mut self, val: Quantity) -> &'a mut MeasureReport_GroupBuilder {
        self.value["measureScore"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MeasureReport_GroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn population<'a>(
        &'a mut self,
        val: Vec<MeasureReport_Population>,
    ) -> &'a mut MeasureReport_GroupBuilder {
        self.value["population"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn stratifier<'a>(
        &'a mut self,
        val: Vec<MeasureReport_Stratifier>,
    ) -> &'a mut MeasureReport_GroupBuilder {
        self.value["stratifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
