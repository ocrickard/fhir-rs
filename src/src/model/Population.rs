#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A populatioof people with some set of grouping criteria.

#[derive(Debug)]
pub struct Population<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Population<'_> {
    pub fn new(value: &Value) -> Population {
        Population {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The age of the specific population.
    pub fn age_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("ageCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The age of the specific population.
    pub fn age_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("ageRange") {
            return Some(Range {
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

    /// The gender of the specific population.
    pub fn gender(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("gender") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The existing physiological conditions of the specific population to which this
    /// applies.
    pub fn physiological_condition(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("physiologicalCondition") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Race of the specific population.
    pub fn race(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("race") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.age_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.age_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.gender() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.physiological_condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.race() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct PopulationBuilder {
    pub(crate) value: Value,
}

impl PopulationBuilder {
    pub fn build(&self) -> Population {
        Population {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Population) -> PopulationBuilder {
        PopulationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> PopulationBuilder {
        let mut __value: Value = json!({});
        return PopulationBuilder { value: __value };
    }

    pub fn age_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PopulationBuilder {
        self.value["ageCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn age_range<'a>(&'a mut self, val: Range) -> &'a mut PopulationBuilder {
        self.value["ageRange"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PopulationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(&'a mut self, val: CodeableConcept) -> &'a mut PopulationBuilder {
        self.value["gender"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut PopulationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut PopulationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn physiological_condition<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut PopulationBuilder {
        self.value["physiologicalCondition"] = json!(val.value);
        return self;
    }

    pub fn race<'a>(&'a mut self, val: CodeableConcept) -> &'a mut PopulationBuilder {
        self.value["race"] = json!(val.value);
        return self;
    }
}
