#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::value::Value;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_StatusHistory<'a> {
    pub value: &'a Value,
}

impl Encounter_StatusHistory<'_> {
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

    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +.
    pub fn status(&self) -> Option<Encounter_StatusHistoryStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Encounter_StatusHistoryStatus::from_string(&val).unwrap());
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The time that the episode was in the specified status.
    pub fn period(&self) -> Period {
        Period {
            value: &self.value["period"],
        }
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        let _ = self.period().validate();
        if let Some(_val) = self._status() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Encounter_StatusHistoryStatus {
    Planned,
    Arrived,
    Triaged,
    InProgress,
    Onleave,
    Finished,
    Cancelled,
    EnteredInError,
    Unknown,
}

impl Encounter_StatusHistoryStatus {
    pub fn from_string(string: &str) -> Option<Encounter_StatusHistoryStatus> {
        match string {
            "planned" => Some(Encounter_StatusHistoryStatus::Planned),
            "arrived" => Some(Encounter_StatusHistoryStatus::Arrived),
            "triaged" => Some(Encounter_StatusHistoryStatus::Triaged),
            "in-progress" => Some(Encounter_StatusHistoryStatus::InProgress),
            "onleave" => Some(Encounter_StatusHistoryStatus::Onleave),
            "finished" => Some(Encounter_StatusHistoryStatus::Finished),
            "cancelled" => Some(Encounter_StatusHistoryStatus::Cancelled),
            "entered-in-error" => Some(Encounter_StatusHistoryStatus::EnteredInError),
            "unknown" => Some(Encounter_StatusHistoryStatus::Unknown),
            _ => None,
        }
    }
}
