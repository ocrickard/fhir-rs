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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A booking of a healthcare event among patient(s), practitioner(s), related
/// person(s) and/or device(s) for a specific date/time. This may result in one or
/// more Encounter(s).

#[derive(Debug)]
pub struct Appointment<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Appointment<'_> {
    pub fn new(value: &Value) -> Appointment {
        Appointment {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for minutesDuration
    pub fn _minutes_duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minutesDuration") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for patientInstruction
    pub fn _patient_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patientInstruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
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

    /// The style of appointment or patient that has been booked in the slot (not
    /// service type).
    pub fn appointment_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("appointmentType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The service request this appointment is allocated to assess (e.g. incoming
    /// referral or procedure request).
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The coded reason for the appointment being cancelled. This is often used in
    /// reporting/billing/futher processing to determine if further actions are
    /// required, or specific fees apply.
    pub fn cancelation_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("cancelationReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The brief description of the appointment as would be shown on a subject line in
    /// a meeting request, or appointment list. Detailed or expanded information should
    /// be put in the comment field.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
            .map(|e| Appointment_Participant {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// The priority of the appointment. Can be used to make informed decisions if
    /// needing to re-prioritize appointments. (The iCal Standard specifies 0 as
    /// undefined, 1 as highest, 9 as lowest priority).
    pub fn priority(&self) -> Option<u64> {
        if let Some(val) = self.value.get("priority") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// The coded reason that this appointment is being scheduled. This is more clinical
    /// than administrative.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
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

    /// Reason the appointment has been scheduled to take place, as specified using
    /// information from another resource. When the patient arrives and the encounter
    /// begins it may be used as the admission diagnosis. The indication will typically
    /// be a Condition (with other resources referenced in the evidence.detail), or a
    /// Procedure.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Period {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specific service that is to be performed during this appointment.
    pub fn service_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("serviceType") {
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

    /// The slots from the participants' schedules that will be filled by the
    /// appointment.
    pub fn slot(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("slot") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specialty of a practitioner that would be required to perform the service
    /// requested in this appointment.
    pub fn specialty(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialty") {
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

    /// Date/Time that the appointment is to take place.
    pub fn start(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("start") {
            return Some(string);
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

    /// Additional information to support the appointment provided when making the
    /// appointment.
    pub fn supporting_information(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInformation") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._created() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._end() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._minutes_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._patient_instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.appointment_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cancelation_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.minutes_duration() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self
            .participant()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.patient_instruction() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.reason_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requested_period() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.service_category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.service_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.slot() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specialty() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.start() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.supporting_information() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AppointmentBuilder {
    pub(crate) value: Value,
}

impl AppointmentBuilder {
    pub fn build(&self) -> Appointment {
        Appointment {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Appointment) -> AppointmentBuilder {
        AppointmentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(participant: Vec<Appointment_Participant>) -> AppointmentBuilder {
        let mut __value: Value = json!({});
        __value["participant"] =
            json!(participant.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return AppointmentBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _created<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_created"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _end<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_end"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _minutes_duration<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_minutesDuration"] = json!(val.value);
        return self;
    }

    pub fn _patient_instruction<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_patientInstruction"] = json!(val.value);
        return self;
    }

    pub fn _priority<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_priority"] = json!(val.value);
        return self;
    }

    pub fn _start<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_start"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut AppointmentBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn appointment_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut AppointmentBuilder {
        self.value["appointmentType"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut AppointmentBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cancelation_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AppointmentBuilder {
        self.value["cancelationReason"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut AppointmentBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["created"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn end<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["end"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AppointmentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut AppointmentBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut AppointmentBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn minutes_duration<'a>(&'a mut self, val: i64) -> &'a mut AppointmentBuilder {
        self.value["minutesDuration"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut AppointmentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn patient_instruction<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["patientInstruction"] = json!(val);
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: u64) -> &'a mut AppointmentBuilder {
        self.value["priority"] = json!(val);
        return self;
    }

    pub fn reason_code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AppointmentBuilder {
        self.value["reasonCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_reference<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut AppointmentBuilder {
        self.value["reasonReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requested_period<'a>(&'a mut self, val: Vec<Period>) -> &'a mut AppointmentBuilder {
        self.value["requestedPeriod"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn service_category<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut AppointmentBuilder {
        self.value["serviceCategory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn service_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AppointmentBuilder {
        self.value["serviceType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn slot<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut AppointmentBuilder {
        self.value["slot"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specialty<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut AppointmentBuilder {
        self.value["specialty"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn start<'a>(&'a mut self, val: &str) -> &'a mut AppointmentBuilder {
        self.value["start"] = json!(val);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: AppointmentStatus) -> &'a mut AppointmentBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn supporting_information<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut AppointmentBuilder {
        self.value["supportingInformation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut AppointmentBuilder {
        self.value["text"] = json!(val.value);
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            AppointmentStatus::Proposed => "proposed".to_string(),
            AppointmentStatus::Pending => "pending".to_string(),
            AppointmentStatus::Booked => "booked".to_string(),
            AppointmentStatus::Arrived => "arrived".to_string(),
            AppointmentStatus::Fulfilled => "fulfilled".to_string(),
            AppointmentStatus::Cancelled => "cancelled".to_string(),
            AppointmentStatus::Noshow => "noshow".to_string(),
            AppointmentStatus::EnteredInError => "entered-in-error".to_string(),
            AppointmentStatus::CheckedIn => "checked-in".to_string(),
            AppointmentStatus::Waitlist => "waitlist".to_string(),
        }
    }
}
