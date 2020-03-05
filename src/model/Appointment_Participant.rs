#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).

#[derive(Debug)]
pub struct Appointment_Participant<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Appointment_Participant<'_> {
    pub fn new(value: &Value) -> Appointment_Participant {
        Appointment_Participant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// A Person, Location/HealthcareService or Device that is participating in the
    /// appointment.
    pub fn actor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("actor") {
            return Some(Reference {
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

    /// Participation period of the actor.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Whether this participant is required to be present at the meeting. This covers a
    /// use-case where two doctors need to meet to discuss the results for a specific
    /// patient, and the patient is not required to be present.
    pub fn required(&self) -> Option<Appointment_ParticipantRequired> {
        if let Some(Value::String(val)) = self.value.get("required") {
            return Some(Appointment_ParticipantRequired::from_string(&val).unwrap());
        }
        return None;
    }

    /// Participation status of the actor.
    pub fn status(&self) -> Option<Appointment_ParticipantStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Appointment_ParticipantStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Role of participant in the appointment.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._required() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actor() {
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
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.required() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Appointment_ParticipantBuilder {
    pub(crate) value: Value,
}

impl Appointment_ParticipantBuilder {
    pub fn build(&self) -> Appointment_Participant {
        Appointment_Participant {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Appointment_Participant) -> Appointment_ParticipantBuilder {
        Appointment_ParticipantBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Appointment_ParticipantBuilder {
        let mut __value: Value = json!({});
        return Appointment_ParticipantBuilder { value: __value };
    }

    pub fn _required<'a>(&'a mut self, val: Element) -> &'a mut Appointment_ParticipantBuilder {
        self.value["_required"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut Appointment_ParticipantBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn actor<'a>(&'a mut self, val: Reference) -> &'a mut Appointment_ParticipantBuilder {
        self.value["actor"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Appointment_ParticipantBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Appointment_ParticipantBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Appointment_ParticipantBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut Appointment_ParticipantBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn required<'a>(
        &'a mut self,
        val: Appointment_ParticipantRequired,
    ) -> &'a mut Appointment_ParticipantBuilder {
        self.value["required"] = json!(val.to_string());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: Appointment_ParticipantStatus,
    ) -> &'a mut Appointment_ParticipantBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Appointment_ParticipantBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum Appointment_ParticipantRequired {
    Required,
    Optional,
    InformationOnly,
}

impl Appointment_ParticipantRequired {
    pub fn from_string(string: &str) -> Option<Appointment_ParticipantRequired> {
        match string {
            "required" => Some(Appointment_ParticipantRequired::Required),
            "optional" => Some(Appointment_ParticipantRequired::Optional),
            "information-only" => Some(Appointment_ParticipantRequired::InformationOnly),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Appointment_ParticipantRequired::Required => "required".to_string(),
            Appointment_ParticipantRequired::Optional => "optional".to_string(),
            Appointment_ParticipantRequired::InformationOnly => "information-only".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Appointment_ParticipantStatus {
    Accepted,
    Declined,
    Tentative,
    NeedsAction,
}

impl Appointment_ParticipantStatus {
    pub fn from_string(string: &str) -> Option<Appointment_ParticipantStatus> {
        match string {
            "accepted" => Some(Appointment_ParticipantStatus::Accepted),
            "declined" => Some(Appointment_ParticipantStatus::Declined),
            "tentative" => Some(Appointment_ParticipantStatus::Tentative),
            "needs-action" => Some(Appointment_ParticipantStatus::NeedsAction),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Appointment_ParticipantStatus::Accepted => "accepted".to_string(),
            Appointment_ParticipantStatus::Declined => "declined".to_string(),
            Appointment_ParticipantStatus::Tentative => "tentative".to_string(),
            Appointment_ParticipantStatus::NeedsAction => "needs-action".to_string(),
        }
    }
}
