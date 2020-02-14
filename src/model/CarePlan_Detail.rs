#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.

#[derive(Debug)]
pub struct CarePlan_Detail<'a> {
    pub value: &'a Value,
}

impl CarePlan_Detail<'_> {
    /// Identifies the quantity expected to be consumed in a given day.
    pub fn daily_amount(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("dailyAmount") {
            return Some(Quantity { value: val });
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

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("scheduledTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// Identifies who's expected to be involved in the activity.
    pub fn performer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The URL pointing to an externally maintained protocol, guideline, questionnaire
    /// or other definition that is adhered to in whole or in part by this CarePlan
    /// activity.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// A description of the kind of resource the in-line definition of a care plan
    /// activity is representing.  The CarePlan.activity.detail is an in-line definition
    /// when a resource is not referenced using CarePlan.activity.reference.  For
    /// example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest.
    pub fn kind(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("kind") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined protocol, guideline, questionnaire or other
    /// definition that is adhered to in whole or in part by this CarePlan activity.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("scheduledString") {
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

    /// Detailed description of the type of planned activity; e.g. what lab test, what
    /// procedure, what kind of encounter.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Provides the rationale that drove the inclusion of this particular activity as
    /// part of the plan or the reason why the activity was prohibited.
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

    /// Identifies the facility where the activity will occur; e.g. home, hospital,
    /// specific clinic, etc.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Internal reference that identifies the goals that this activity is intended to
    /// contribute towards meeting.
    pub fn goal(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("goal") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Provides reason why the activity isn't yet started, is on hold, was cancelled,
    /// etc.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// If true, indicates that the described activity is one that must NOT be engaged
    /// in when following the plan.  If false, or missing, indicates that the described
    /// activity is one that should be engaged in when following the plan.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Identifies the food, drug or other product to be consumed or supplied in the
    /// activity.
    pub fn product_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("productReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Indicates another resource, such as the health condition(s), whose existence
    /// justifies this request and drove the inclusion of this particular activity as
    /// part of the plan.
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

    /// Identifies what progress is being made for the specific activity.
    pub fn status(&self) -> Option<CarePlan_DetailStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(CarePlan_DetailStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn scheduled_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("scheduledPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Identifies the quantity expected to be supplied, administered or consumed by the
    /// subject.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Identifies the food, drug or other product to be consumed or supplied in the
    /// activity.
    pub fn product_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for scheduledString
    pub fn _scheduled_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_scheduledString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// This provides a textual description of constraints on the intended activity
    /// occurrence, including relation to other activities.  It may also include
    /// objectives, pre-conditions and end-conditions.  Finally, it may convey specifics
    /// about the activity such as body site, method, route, etc.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.daily_amount() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.scheduled_timing() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self._kind() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.scheduled_string() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.goal() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.product_reference() {
            _val.validate();
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._do_not_perform() {
            _val.validate();
        }
        if let Some(_val) = self.scheduled_period() {
            _val.validate();
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.product_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._scheduled_string() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        return true;
    }
}

#[derive(Debug)]
pub enum CarePlan_DetailStatus {
    NotStarted,
    Scheduled,
    InProgress,
    OnHold,
    Completed,
    Cancelled,
    Stopped,
    Unknown,
    EnteredInError,
}

impl CarePlan_DetailStatus {
    pub fn from_string(string: &str) -> Option<CarePlan_DetailStatus> {
        match string {
            "not-started" => Some(CarePlan_DetailStatus::NotStarted),
            "scheduled" => Some(CarePlan_DetailStatus::Scheduled),
            "in-progress" => Some(CarePlan_DetailStatus::InProgress),
            "on-hold" => Some(CarePlan_DetailStatus::OnHold),
            "completed" => Some(CarePlan_DetailStatus::Completed),
            "cancelled" => Some(CarePlan_DetailStatus::Cancelled),
            "stopped" => Some(CarePlan_DetailStatus::Stopped),
            "unknown" => Some(CarePlan_DetailStatus::Unknown),
            "entered-in-error" => Some(CarePlan_DetailStatus::EnteredInError),
            _ => None,
        }
    }
}
