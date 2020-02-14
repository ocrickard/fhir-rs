#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_Location<'a> {
    pub value: &'a Value,
}

impl Encounter_Location<'_> {
    /// This will be used to specify the required levels (bed/ward/room/etc.) desired to
    /// be recorded to simplify either messaging or query.
    pub fn physical_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("physicalType") {
            return Some(CodeableConcept { value: val });
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

    /// The status of the participants' presence at the specified location during the
    /// period specified. If the participant is no longer at the location, then the
    /// period will have an end date/time.
    pub fn status(&self) -> Option<Encounter_LocationStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Encounter_LocationStatus::from_string(&val).unwrap());
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The location where the encounter takes place.
    pub fn location(&self) -> Reference {
        Reference {
            value: &self.value["location"],
        }
    }

    /// Time period during which the patient was present at the location.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.physical_type() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        let _ = self.location().validate();
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Encounter_LocationStatus {
    Planned,
    Active,
    Reserved,
    Completed,
}

impl Encounter_LocationStatus {
    pub fn from_string(string: &str) -> Option<Encounter_LocationStatus> {
        match string {
            "planned" => Some(Encounter_LocationStatus::Planned),
            "active" => Some(Encounter_LocationStatus::Active),
            "reserved" => Some(Encounter_LocationStatus::Reserved),
            "completed" => Some(Encounter_LocationStatus::Completed),
            _ => None,
        }
    }
}
