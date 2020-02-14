#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_Diagnosis<'a> {
    pub value: &'a Value,
}

impl Encounter_Diagnosis<'_> {
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

    /// Ranking of the diagnosis (for each role type).
    pub fn rank(&self) -> Option<i64> {
        if let Some(val) = self.value.get("rank") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for rank
    pub fn _rank(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_rank") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Reason the encounter takes place, as specified using information from another
    /// resource. For admissions, this is the admission diagnosis. The indication will
    /// typically be a Condition (with other resources referenced in the
    /// evidence.detail), or a Procedure.
    pub fn condition(&self) -> Reference {
        Reference {
            value: &self.value["condition"],
        }
    }

    /// Role that this diagnosis has within the encounter (e.g. admission, billing,
    /// discharge …).
    pub fn fhir_use(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("use") {
            return Some(CodeableConcept { value: val });
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
        if let Some(_val) = self.rank() {}
        if let Some(_val) = self._rank() {
            _val.validate();
        }
        let _ = self.condition().validate();
        if let Some(_val) = self.fhir_use() {
            _val.validate();
        }
        return true;
    }
}
