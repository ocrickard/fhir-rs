#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An association between a patient and an organization / healthcare provider(s)
/// during which time encounters may occur. The managing organization assumes a
/// level of responsibility for the patient during this time.

#[derive(Debug)]
pub struct EpisodeOfCare_StatusHistory<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EpisodeOfCare_StatusHistory<'_> {
    pub fn new(value: &Value) -> EpisodeOfCare_StatusHistory {
        EpisodeOfCare_StatusHistory {
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

    /// The period during this EpisodeOfCare that the specific status applied.
    pub fn period(&self) -> Period {
        Period {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    /// planned | waitlist | active | onhold | finished | cancelled.
    pub fn status(&self) -> Option<EpisodeOfCare_StatusHistoryStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(EpisodeOfCare_StatusHistoryStatus::from_string(&val).unwrap());
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
pub struct EpisodeOfCare_StatusHistoryBuilder {
    pub(crate) value: Value,
}

impl EpisodeOfCare_StatusHistoryBuilder {
    pub fn build(&self) -> EpisodeOfCare_StatusHistory {
        EpisodeOfCare_StatusHistory {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: EpisodeOfCare_StatusHistory) -> EpisodeOfCare_StatusHistoryBuilder {
        EpisodeOfCare_StatusHistoryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(period: Period) -> EpisodeOfCare_StatusHistoryBuilder {
        let mut __value: Value = json!({});
        __value["period"] = json!(period.value);
        return EpisodeOfCare_StatusHistoryBuilder { value: __value };
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut EpisodeOfCare_StatusHistoryBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EpisodeOfCare_StatusHistoryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EpisodeOfCare_StatusHistoryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EpisodeOfCare_StatusHistoryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: EpisodeOfCare_StatusHistoryStatus,
    ) -> &'a mut EpisodeOfCare_StatusHistoryBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum EpisodeOfCare_StatusHistoryStatus {
    Planned,
    Waitlist,
    Active,
    Onhold,
    Finished,
    Cancelled,
    EnteredInError,
}

impl EpisodeOfCare_StatusHistoryStatus {
    pub fn from_string(string: &str) -> Option<EpisodeOfCare_StatusHistoryStatus> {
        match string {
            "planned" => Some(EpisodeOfCare_StatusHistoryStatus::Planned),
            "waitlist" => Some(EpisodeOfCare_StatusHistoryStatus::Waitlist),
            "active" => Some(EpisodeOfCare_StatusHistoryStatus::Active),
            "onhold" => Some(EpisodeOfCare_StatusHistoryStatus::Onhold),
            "finished" => Some(EpisodeOfCare_StatusHistoryStatus::Finished),
            "cancelled" => Some(EpisodeOfCare_StatusHistoryStatus::Cancelled),
            "entered-in-error" => Some(EpisodeOfCare_StatusHistoryStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EpisodeOfCare_StatusHistoryStatus::Planned => "planned".to_string(),
            EpisodeOfCare_StatusHistoryStatus::Waitlist => "waitlist".to_string(),
            EpisodeOfCare_StatusHistoryStatus::Active => "active".to_string(),
            EpisodeOfCare_StatusHistoryStatus::Onhold => "onhold".to_string(),
            EpisodeOfCare_StatusHistoryStatus::Finished => "finished".to_string(),
            EpisodeOfCare_StatusHistoryStatus::Cancelled => "cancelled".to_string(),
            EpisodeOfCare_StatusHistoryStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
