#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_StatusHistory<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Encounter_StatusHistory<'_> {
    pub fn new(value: &Value) -> Encounter_StatusHistory {
        Encounter_StatusHistory {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
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

    /// The time that the episode was in the specified status.
    pub fn period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +.
    pub fn status(&self) -> Option<Encounter_StatusHistoryStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Encounter_StatusHistoryStatus::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._status() {
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
        if !self.period().validate() {
            return false;
        }
        if let Some(_val) = self.status() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Encounter_StatusHistoryBuilder {
    pub(crate) value: Value,
}

impl Encounter_StatusHistoryBuilder {
    pub fn build(&self) -> Encounter_StatusHistory {
        Encounter_StatusHistory {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Encounter_StatusHistory) -> Encounter_StatusHistoryBuilder {
        Encounter_StatusHistoryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(period: Period) -> Encounter_StatusHistoryBuilder {
        let mut __value: Value = json!({});
        __value["period"] = json!(period.value);
        return Encounter_StatusHistoryBuilder { value: __value };
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut Encounter_StatusHistoryBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Encounter_StatusHistoryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Encounter_StatusHistoryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Encounter_StatusHistoryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: Encounter_StatusHistoryStatus,
    ) -> &'a mut Encounter_StatusHistoryBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            Encounter_StatusHistoryStatus::Planned => "planned".to_string(),
            Encounter_StatusHistoryStatus::Arrived => "arrived".to_string(),
            Encounter_StatusHistoryStatus::Triaged => "triaged".to_string(),
            Encounter_StatusHistoryStatus::InProgress => "in-progress".to_string(),
            Encounter_StatusHistoryStatus::Onleave => "onleave".to_string(),
            Encounter_StatusHistoryStatus::Finished => "finished".to_string(),
            Encounter_StatusHistoryStatus::Cancelled => "cancelled".to_string(),
            Encounter_StatusHistoryStatus::EnteredInError => "entered-in-error".to_string(),
            Encounter_StatusHistoryStatus::Unknown => "unknown".to_string(),
        }
    }
}
