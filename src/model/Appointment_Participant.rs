#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).

#[derive(Debug)]
pub struct Appointment_Participant<'a> {
    pub value: &'a Value,
}

impl Appointment_Participant<'_> {
    /// Participation status of the actor.
    pub fn status(&self) -> Option<Appointment_ParticipantStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Appointment_ParticipantStatus::from_string(&val).unwrap());
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

    /// A Person, Location/HealthcareService or Device that is participating in the
    /// appointment.
    pub fn actor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("actor") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Role of participant in the appointment.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Whether this participant is required to be present at the meeting. This covers a
    /// use-case where two doctors need to meet to discuss the results for a specific
    /// patient, and the patient is not required to be present.
    pub fn required(&self) -> Option<Appointment_ParticipantRequired> {
        if let Some(Value::String(val)) = self.value.get("required") {
            return Some(Appointment_ParticipantRequired::from_string(&val).unwrap());
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

    /// Participation period of the actor.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.actor() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.required() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self._required() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
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
}
