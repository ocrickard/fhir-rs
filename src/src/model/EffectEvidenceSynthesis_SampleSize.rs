#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_SampleSize<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EffectEvidenceSynthesis_SampleSize<'_> {
    pub fn new(value: &Value) -> EffectEvidenceSynthesis_SampleSize {
        EffectEvidenceSynthesis_SampleSize {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfParticipants
    pub fn _number_of_participants(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfParticipants") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfStudies
    pub fn _number_of_studies(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfStudies") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Human-readable summary of sample size.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// Number of participants included in this evidence synthesis.
    pub fn number_of_participants(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfParticipants") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Number of studies included in this evidence synthesis.
    pub fn number_of_studies(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfStudies") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_participants() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_studies() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.number_of_participants() {}
        if let Some(_val) = self.number_of_studies() {}
        return true;
    }
}

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_SampleSizeBuilder {
    pub(crate) value: Value,
}

impl EffectEvidenceSynthesis_SampleSizeBuilder {
    pub fn build(&self) -> EffectEvidenceSynthesis_SampleSize {
        EffectEvidenceSynthesis_SampleSize {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: EffectEvidenceSynthesis_SampleSize,
    ) -> EffectEvidenceSynthesis_SampleSizeBuilder {
        EffectEvidenceSynthesis_SampleSizeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> EffectEvidenceSynthesis_SampleSizeBuilder {
        let mut __value: Value = json!({});
        return EffectEvidenceSynthesis_SampleSizeBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _number_of_participants<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["_numberOfParticipants"] = json!(val.value);
        return self;
    }

    pub fn _number_of_studies<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["_numberOfStudies"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_of_participants<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["numberOfParticipants"] = json!(val);
        return self;
    }

    pub fn number_of_studies<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut EffectEvidenceSynthesis_SampleSizeBuilder {
        self.value["numberOfStudies"] = json!(val);
        return self;
    }
}
