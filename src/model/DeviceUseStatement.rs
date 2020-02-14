#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A record of a device being used by a patient where the record is the result of a
/// report from the patient or another clinician.

#[derive(Debug)]
pub struct DeviceUseStatement<'a> {
    pub value: &'a Value,
}

impl DeviceUseStatement<'_> {
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

    /// How often the device was used.
    pub fn timing_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timingPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Details about the device statement that were not represented at all or
    /// sufficiently in one of the attributes provided in a class. These may include for
    /// example a comment, an instruction, or a note associated with the statement.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
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

    /// The time at which the statement was made/recorded.
    pub fn recorded_on(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recordedOn") {
            return Some(string);
        }
        return None;
    }

    /// A code representing the patient or other source's judgment about the state of
    /// the device used that this statement is about.  Generally this will be active or
    /// completed.
    pub fn status(&self) -> Option<DeviceUseStatementStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(DeviceUseStatementStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// How often the device was used.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The details of the device used.
    pub fn device(&self) -> Reference {
        Reference {
            value: &self.value["device"],
        }
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

    /// Allows linking the DeviceUseStatement to the underlying Request, or to other
    /// information that supports or is used to derive the DeviceUseStatement.
    pub fn derived_from(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates the anotomic location on the subject's body where the device was used
    /// ( i.e. the target).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept { value: val });
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

    /// An external identifier for this statement such as an IRI.
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

    /// Extensions for recordedOn
    pub fn _recorded_on(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recordedOn") {
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

    /// Indicates another resource whose existence justifies this DeviceUseStatement.
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

    /// Who reported the device was being used by the patient.
    pub fn source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("source") {
            return Some(Reference { value: val });
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
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

    /// Reason or justification for the use of the device.
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

    /// How often the device was used.
    pub fn timing_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for timingDateTime
    pub fn _timing_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The patient who used the device.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A plan, proposal or order that is fulfilled in whole or in part by this
    /// DeviceUseStatement.
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
        if let Some(_val) = self.timing_period() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.recorded_on() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.timing_timing() {
            _val.validate();
        }
        let _ = self.device().validate();
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.derived_from() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.body_site() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._recorded_on() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.source() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self._timing_date_time() {
            _val.validate();
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum DeviceUseStatementStatus {
    Active,
    Completed,
    EnteredInError,
    Intended,
    Stopped,
    OnHold,
}

impl DeviceUseStatementStatus {
    pub fn from_string(string: &str) -> Option<DeviceUseStatementStatus> {
        match string {
            "active" => Some(DeviceUseStatementStatus::Active),
            "completed" => Some(DeviceUseStatementStatus::Completed),
            "entered-in-error" => Some(DeviceUseStatementStatus::EnteredInError),
            "intended" => Some(DeviceUseStatementStatus::Intended),
            "stopped" => Some(DeviceUseStatementStatus::Stopped),
            "on-hold" => Some(DeviceUseStatementStatus::OnHold),
            _ => None,
        }
    }
}
