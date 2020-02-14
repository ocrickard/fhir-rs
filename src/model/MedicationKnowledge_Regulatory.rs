#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_MaxDispense::MedicationKnowledge_MaxDispense;
use crate::model::MedicationKnowledge_Schedule::MedicationKnowledge_Schedule;
use crate::model::MedicationKnowledge_Substitution::MedicationKnowledge_Substitution;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Regulatory<'a> {
    pub value: &'a Value,
}

impl MedicationKnowledge_Regulatory<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies if changes are allowed when dispensing a medication from a regulatory
    /// perspective.
    pub fn substitution(&self) -> Option<Vec<MedicationKnowledge_Substitution>> {
        if let Some(Value::Array(val)) = self.value.get("substitution") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Substitution { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies the schedule of a medication in jurisdiction.
    pub fn schedule(&self) -> Option<Vec<MedicationKnowledge_Schedule>> {
        if let Some(Value::Array(val)) = self.value.get("schedule") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Schedule { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The authority that is specifying the regulations.
    pub fn regulatory_authority(&self) -> Reference {
        Reference {
            value: &self.value["regulatoryAuthority"],
        }
    }

    /// The maximum number of units of the medication that can be dispensed in a period.
    pub fn max_dispense(&self) -> Option<MedicationKnowledge_MaxDispense> {
        if let Some(val) = self.value.get("maxDispense") {
            return Some(MedicationKnowledge_MaxDispense { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.substitution() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.schedule() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.regulatory_authority().validate();
        if let Some(_val) = self.max_dispense() {
            _val.validate();
        }
        return true;
    }
}
