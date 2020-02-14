#![allow(unused_imports, non_camel_case_types)]

use crate::model::AuditEvent_Agent::AuditEvent_Agent;
use crate::model::AuditEvent_Entity::AuditEvent_Entity;
use crate::model::AuditEvent_Source::AuditEvent_Source;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A record of an event made for purposes of maintaining a security log. Typical
/// uses include detection of intrusion attempts and monitoring for inappropriate
/// usage.

#[derive(Debug)]
pub struct AuditEvent<'a> {
    pub value: &'a Value,
}

impl AuditEvent<'_> {
    /// Extensions for outcomeDesc
    pub fn _outcome_desc(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcomeDesc") {
            return Some(Element { value: val });
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

    /// The purposeOfUse (reason) that was used during the event being recorded.
    pub fn purpose_of_event(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("purposeOfEvent") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// The time when the event was recorded.
    pub fn recorded(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("recorded") {
            return Some(string);
        }
        return None;
    }

    /// Identifier for the category of event.
    pub fn subtype(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("subtype") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An actor taking an active role in the event or activity that is logged.
    pub fn agent(&self) -> Vec<AuditEvent_Agent> {
        self.value
            .get("agent")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| AuditEvent_Agent { value: e })
            .collect::<Vec<_>>()
    }

    /// The system that is reporting the event.
    pub fn source(&self) -> AuditEvent_Source {
        AuditEvent_Source {
            value: &self.value["source"],
        }
    }

    /// Extensions for action
    pub fn _action(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_action") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicator for type of action performed during the event that generated the
    /// audit.
    pub fn action(&self) -> Option<AuditEventAction> {
        if let Some(Value::String(val)) = self.value.get("action") {
            return Some(AuditEventAction::from_string(&val).unwrap());
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

    /// A free text description of the outcome of the event.
    pub fn outcome_desc(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("outcomeDesc") {
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

    /// Identifier for a family of the event.  For example, a menu item, program, rule,
    /// policy, function code, application name or URL. It identifies the performed
    /// function.
    pub fn fhir_type(&self) -> Coding {
        Coding {
            value: &self.value["type"],
        }
    }

    /// Specific instances of data or objects that have been accessed.
    pub fn entity(&self) -> Option<Vec<AuditEvent_Entity>> {
        if let Some(Value::Array(val)) = self.value.get("entity") {
            return Some(
                val.into_iter()
                    .map(|e| AuditEvent_Entity { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the activity occurred.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Indicates whether the event succeeded or failed.
    pub fn outcome(&self) -> Option<AuditEventOutcome> {
        if let Some(Value::String(val)) = self.value.get("outcome") {
            return Some(AuditEventOutcome::from_string(&val).unwrap());
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for recorded
    pub fn _recorded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recorded") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._outcome_desc() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.purpose_of_event() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.recorded() {}
        if let Some(_val) = self.subtype() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.agent().into_iter().for_each(|e| {
            e.validate();
        });
        let _ = self.source().validate();
        if let Some(_val) = self._action() {
            _val.validate();
        }
        if let Some(_val) = self.action() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.outcome_desc() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self.entity() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.period() {
            _val.validate();
        }
        if let Some(_val) = self.outcome() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._outcome() {
            _val.validate();
        }
        if let Some(_val) = self._recorded() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum AuditEventAction {
    C,
    R,
    U,
    D,
    E,
}

impl AuditEventAction {
    pub fn from_string(string: &str) -> Option<AuditEventAction> {
        match string {
            "C" => Some(AuditEventAction::C),
            "R" => Some(AuditEventAction::R),
            "U" => Some(AuditEventAction::U),
            "D" => Some(AuditEventAction::D),
            "E" => Some(AuditEventAction::E),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum AuditEventOutcome {
    Zero,
    Four,
    Eight,
    Twelve,
}

impl AuditEventOutcome {
    pub fn from_string(string: &str) -> Option<AuditEventOutcome> {
        match string {
            "0" => Some(AuditEventOutcome::Zero),
            "4" => Some(AuditEventOutcome::Four),
            "8" => Some(AuditEventOutcome::Eight),
            "12" => Some(AuditEventOutcome::Twelve),
            _ => None,
        }
    }
}
