#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the event of a patient consuming or otherwise being administered a
/// medication.  This may be as simple as swallowing a tablet or it may be a long
/// running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.

#[derive(Debug)]
pub struct MedicationAdministration_Performer<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationAdministration_Performer<'_> {
    pub fn new(value: &Value) -> MedicationAdministration_Performer {
        MedicationAdministration_Performer {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Indicates who or what performed the medication administration.
    pub fn actor(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["actor"]),
        }
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

    /// Distinguishes the type of involvement of the performer in the medication
    /// administration.
    pub fn function(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("function") {
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

    pub fn validate(&self) -> bool {
        if !self.actor().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.function() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationAdministration_PerformerBuilder {
    pub(crate) value: Value,
}

impl MedicationAdministration_PerformerBuilder {
    pub fn build(&self) -> MedicationAdministration_Performer {
        MedicationAdministration_Performer {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationAdministration_Performer,
    ) -> MedicationAdministration_PerformerBuilder {
        MedicationAdministration_PerformerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(actor: Reference) -> MedicationAdministration_PerformerBuilder {
        let mut __value: Value = json!({});
        __value["actor"] = json!(actor.value);
        return MedicationAdministration_PerformerBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationAdministration_PerformerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn function<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationAdministration_PerformerBuilder {
        self.value["function"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationAdministration_PerformerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationAdministration_PerformerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
