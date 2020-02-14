#![allow(unused_imports, non_camel_case_types)]

use crate::model::Appointment_Participant::Appointment_Participant;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).

#[derive(Debug)]
pub struct Appointment<'a> {
    pub value: &'a Value,
}

impl Appointment<'_> {
    /// The specialty of a practitioner that would be required to perform the service
    /// requested in this appointment.
    pub fn specialty(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialty") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// A broad categorization of the service that is to be performed during this
    /// appointment.
    pub fn service_category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("serviceCategory") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Reason the appointment has been scheduled to take place, as specified using
    /// information from another resource. When the patient arrives and the encounter
    /// begins it may be used as the admission diagnosis. The indication will typically
    /// be a Condition (with other resources referenced in the evidence.detail), or a
    /// Procedure.
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

    /// The priority of the appointment. Can be used to make informed decisions if
    /// needing to re-prioritize appointments. (The iCal Standard specifies 0 as
    /// undefined, 1 as highest, 9 as lowest priority).
    pub fn priority(&self) -> Option<u64> {
        if let Some(val) = self.value.get("priority") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// The slots from the participants' schedules that will be filled by the
    /// appointment.
    pub fn slot(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("slot") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The service request this appointment is allocated to assess (e.g. incoming
    /// referral or procedure request).
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

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Additional comments about the appointment.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
        }
        return None;
    }

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

    /// Date/Time that the appointment is to conclude.
    pub fn end(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("end") {
            return Some(string);
        }
        return None;
    }

    /// Additional information to support the appointment provided when making the
    /// appointment.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The style of appointment or patient that has been booked in the slot (not
    /// service type).
    pub fn appointment_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("appointmentType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The coded reason for the appointment being cancelled. This is often used in
    /// reporting/billing/futher processing to determine if further actions are
    /// required, or specific fees apply.
    pub fn cancelation_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("cancelationReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The specific service that is to be performed during this appointment.
    pub fn service_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("serviceType") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for minutesDuration
    pub fn _minutes_duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minutesDuration") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date that this appointment was initially created. This could be different to
    /// the meta.lastModified value on the initial entry, as this could have been before
    /// the resource was created on the FHIR server, and should remain unchanged over
    /// the lifespan of the appointment.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// List of participants involved in the appointment.
    pub fn participant(&self) -> Vec<Appointment_Participant> {
        self.value
            .get("participant")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Appointment_Participant { value: e })
            .collect::<Vec<_>>()
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A set of date ranges (potentially including times) that the appointment is
    /// preferred to be scheduled within.    The duration (usually in minutes) could
    /// also be provided to indicate the length of the appointment to fill and populate
    /// the start/end times for the actual allocated time. However, in other situations
    /// the duration may be calculated by the scheduling system.
    pub fn requested_period(&self) -> Option<Vec<Period>> {
        if let Some(Value::Array(val)) = self.value.get("requestedPeriod") {
            return Some(
                val.into_iter()
                    .map(|e| Period { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The brief description of the appointment as would be shown on a subject line in
    /// a meeting request, or appointment list. Detailed or expanded information should
    /// be put in the comment field.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// This records identifiers associated with this appointment concern that are
    /// defined by business processes and/or used to refer to it when a direct URL
    /// reference to the resource itself is not appropriate (e.g. in CDA documents, or
    /// in written / printed documentation).
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Number of minutes that the appointment is to take. This can be less than the
    /// duration between the start and end times.  For example, where the actual time of
    /// appointment is only an estimate or if a 30 minute appointment is being
    /// requested, but any time would work.  Also, if there is, for example, a planned
    /// 15 minute break in the middle of a long appointment, the duration may be 15
    /// minutes less than the difference between the start and end.
    pub fn minutes_duration(&self) -> Option<i64> {
        if let Some(val) = self.value.get("minutesDuration") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// While Appointment.comment contains information for internal use,
    /// Appointment.patientInstructions is used to capture patient facing information
    /// about the Appointment (e.g. please bring your referral or fast from 8pm night
    /// before).
    pub fn patient_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patientInstruction") {
            return Some(string);
        }
        return None;
    }

    /// The coded reason that this appointment is being scheduled. This is more clinical
    /// than administrative.
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

    /// The overall status of the Appointment. Each of the participants has their own
    /// participation status which indicates their involvement in the process, however
    /// this status indicates the shared status.
    pub fn status(&self) -> Option<AppointmentStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(AppointmentStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element { value: val });
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
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

    /// Date/Time that the appointment is to take place.
    pub fn start(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("start") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for patientInstruction
    pub fn _patient_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patientInstruction") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.specialty() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.service_category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.slot() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.supporting_information() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._start() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.appointment_type() {
            _val.validate();
        }
        if let Some(_val) = self.cancelation_reason() {
            _val.validate();
        }
        if let Some(_val) = self.service_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._minutes_duration() {
            _val.validate();
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self._created() {
            _val.validate();
        }
        let _ = self.participant().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self.requested_period() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.minutes_duration() {}
        if let Some(_val) = self.patient_instruction() {}
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._end() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.start() {}
        if let Some(_val) = self._patient_instruction() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum AppointmentStatus {
    Proposed,
    Pending,
    Booked,
    Arrived,
    Fulfilled,
    Cancelled,
    Noshow,
    EnteredInError,
    CheckedIn,
    Waitlist,
}

impl AppointmentStatus {
    pub fn from_string(string: &str) -> Option<AppointmentStatus> {
        match string {
            "proposed" => Some(AppointmentStatus::Proposed),
            "pending" => Some(AppointmentStatus::Pending),
            "booked" => Some(AppointmentStatus::Booked),
            "arrived" => Some(AppointmentStatus::Arrived),
            "fulfilled" => Some(AppointmentStatus::Fulfilled),
            "cancelled" => Some(AppointmentStatus::Cancelled),
            "noshow" => Some(AppointmentStatus::Noshow),
            "entered-in-error" => Some(AppointmentStatus::EnteredInError),
            "checked-in" => Some(AppointmentStatus::CheckedIn),
            "waitlist" => Some(AppointmentStatus::Waitlist),
            _ => None,
        }
    }
}
