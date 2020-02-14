#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Encounter_ClassHistory::Encounter_ClassHistory;
use crate::model::Encounter_Diagnosis::Encounter_Diagnosis;
use crate::model::Encounter_Hospitalization::Encounter_Hospitalization;
use crate::model::Encounter_Location::Encounter_Location;
use crate::model::Encounter_Participant::Encounter_Participant;
use crate::model::Encounter_StatusHistory::Encounter_StatusHistory;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter<'a> {
    pub value: &'a Value,
}

impl Encounter<'_> {
    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The list of diagnosis relevant to this encounter.
    pub fn diagnosis(&self) -> Option<Vec<Encounter_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Diagnosis { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The organization that is primarily responsible for this Encounter's services.
    /// This MAY be the same as the organization on the Patient record, however it could
    /// be different, such as if the actor performing the services was from an external
    /// organization (which may be billed seperately) for an external consultation.
    /// Refer to the example bundle showing an abbreviated set of Encounters for a
    /// colonoscopy.
    pub fn service_provider(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("serviceProvider") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// planned | arrived | triaged | in-progress | onleave | finished | cancelled +.
    pub fn status(&self) -> Option<EncounterStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(EncounterStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The patient or group present at the encounter.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The start and end time of the encounter.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Quantity of time the encounter lasted. This excludes the time during leaves of
    /// absence.
    pub fn length(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("length") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Details about the admission to a healthcare service.
    pub fn hospitalization(&self) -> Option<Encounter_Hospitalization> {
        if let Some(val) = self.value.get("hospitalization") {
            return Some(Encounter_Hospitalization { value: val });
        }
        return None;
    }

    /// Where a specific encounter should be classified as a part of a specific
    /// episode(s) of care this field should be used. This association can facilitate
    /// grouping of related encounters together for a specific purpose, such as
    /// government reporting, issue tracking, association via a common problem.  The
    /// association is recorded on the encounter as these are typically created after
    /// the episode of care and grouped on entry rather than editing the episode of care
    /// to append another encounter to it (the episode of care could span years).
    pub fn episode_of_care(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("episodeOfCare") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// Reason the encounter takes place, expressed as a code. For admissions, this can
    /// be used for a coded admission diagnosis.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The class history permits the tracking of the encounters transitions without
    /// needing to go  through the resource history.  This would be used for a case
    /// where an admission starts of as an emergency encounter, then transitions into an
    /// inpatient scenario. Doing this and not restarting a new encounter ensures that
    /// any lab/diagnostic results can more easily follow the patient and not require
    /// re-processing and not get lost or cancelled during a kind of discharge from
    /// emergency to inpatient.
    pub fn class_history(&self) -> Option<Vec<Encounter_ClassHistory>> {
        if let Some(Value::Array(val)) = self.value.get("classHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_ClassHistory { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status history permits the encounter resource to contain the status history
    /// without needing to read through the historical versions of the resource, or even
    /// have the server store them.
    pub fn status_history(&self) -> Option<Vec<Encounter_StatusHistory>> {
        if let Some(Value::Array(val)) = self.value.get("statusHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_StatusHistory { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The set of accounts that may be used for billing for this Encounter.
    pub fn account(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("account") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Reason the encounter takes place, expressed as a code. For admissions, this can
    /// be used for a coded admission diagnosis.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The appointment that scheduled this encounter.
    pub fn appointment(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("appointment") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifier(s) by which this encounter is known.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, skilled
    /// nursing, rehabilitation).
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

    /// Indicates the urgency of the encounter.
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Broad categorization of the service that is to be provided (e.g. cardiology).
    pub fn service_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("serviceType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Concepts representing classification of patient encounter such as ambulatory
    /// (outpatient), inpatient, emergency, home health or others due to local
    /// variations.
    pub fn class(&self) -> Coding {
        Coding {
            value: &self.value["class"],
        }
    }

    /// List of locations where  the patient has been during this encounter.
    pub fn location(&self) -> Option<Vec<Encounter_Location>> {
        if let Some(Value::Array(val)) = self.value.get("location") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Location { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Another Encounter of which this encounter is a part of (administratively or in
    /// time).
    pub fn part_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("partOf") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The list of people responsible for providing the service.
    pub fn participant(&self) -> Option<Vec<Encounter_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| Encounter_Participant { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The request this encounter satisfies (e.g. incoming referral or procedure
    /// request).
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.diagnosis() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.service_provider() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.length() {
            _val.validate();
        }
        if let Some(_val) = self.hospitalization() {
            _val.validate();
        }
        if let Some(_val) = self.episode_of_care() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.class_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.account() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.appointment() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.priority() {
            _val.validate();
        }
        if let Some(_val) = self.service_type() {
            _val.validate();
        }
        let _ = self.class().validate();
        if let Some(_val) = self.location() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.part_of() {
            _val.validate();
        }
        if let Some(_val) = self.participant() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum EncounterStatus {
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

impl EncounterStatus {
    pub fn from_string(string: &str) -> Option<EncounterStatus> {
        match string {
            "planned" => Some(EncounterStatus::Planned),
            "arrived" => Some(EncounterStatus::Arrived),
            "triaged" => Some(EncounterStatus::Triaged),
            "in-progress" => Some(EncounterStatus::InProgress),
            "onleave" => Some(EncounterStatus::Onleave),
            "finished" => Some(EncounterStatus::Finished),
            "cancelled" => Some(EncounterStatus::Cancelled),
            "entered-in-error" => Some(EncounterStatus::EnteredInError),
            "unknown" => Some(EncounterStatus::Unknown),
            _ => None,
        }
    }
}
