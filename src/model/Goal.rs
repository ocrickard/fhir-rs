#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Goal_Target::Goal_Target;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Describes the intended objective(s) for a patient, group or organization care,
/// for example, weight loss, restoring an activity of daily living, obtaining herd
/// immunity via immunization, meeting a process improvement objective, etc.

#[derive(Debug)]
pub struct Goal<'a> {
    pub value: &'a Value,
}

impl Goal<'_> {
    /// Captures the reason for the current status.
    pub fn status_reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusReason") {
            return Some(string);
        }
        return None;
    }

    /// The identified conditions and other health record elements that are intended to
    /// be addressed by the goal.
    pub fn addresses(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("addresses") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Indicates what should be done by when.
    pub fn target(&self) -> Option<Vec<Goal_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| Goal_Target { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for statusReason
    pub fn _status_reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusReason") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for startDate
    pub fn _start_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_startDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for lifecycleStatus
    pub fn _lifecycle_status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lifecycleStatus") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies the patient, group or organization for whom the goal is being
    /// established.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
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

    /// Business identifiers assigned to this goal by the performer or other systems
    /// which remain constant as the resource is updated and propagates from server to
    /// server.
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Details of what's changed (or not changed).
    pub fn outcome_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("outcomeReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates a category the goal falls within.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Identifies when the current status.  I.e. When initially created, when achieved,
    /// when cancelled, etc.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    /// Any comments related to the goal.
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

    /// Describes the progression, or lack thereof, towards the goal against the target.
    pub fn achievement_status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("achievementStatus") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The state of the goal throughout its lifecycle.
    pub fn lifecycle_status(&self) -> Option<GoalLifecycleStatus> {
        if let Some(Value::String(val)) = self.value.get("lifecycleStatus") {
            return Some(GoalLifecycleStatus::from_string(&val).unwrap());
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Identifies the mutually agreed level of importance associated with
    /// reaching/sustaining the goal.
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Identifies the change (or lack of change) at the point when the status of the
    /// goal is assessed.
    pub fn outcome_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("outcomeCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Human-readable and/or coded description of a specific desired objective of care,
    /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
    /// with child at wedding".
    pub fn description(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["description"],
        }
    }

    /// The date or event after which the goal should begin being pursued.
    pub fn start_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("startDate") {
            return Some(string);
        }
        return None;
    }

    /// Indicates whose goal this is - patient goal, practitioner goal, etc.
    pub fn expressed_by(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("expressedBy") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The date or event after which the goal should begin being pursued.
    pub fn start_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("startCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.status_reason() {}
        if let Some(_val) = self.addresses() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status_reason() {
            _val.validate();
        }
        if let Some(_val) = self._start_date() {
            _val.validate();
        }
        if let Some(_val) = self._lifecycle_status() {
            _val.validate();
        }
        let _ = self.subject().validate();
        if let Some(_val) = self._status_date() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.outcome_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.status_date() {}
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.achievement_status() {
            _val.validate();
        }
        if let Some(_val) = self.lifecycle_status() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.priority() {
            _val.validate();
        }
        if let Some(_val) = self.outcome_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.description().validate();
        if let Some(_val) = self.start_date() {}
        if let Some(_val) = self.expressed_by() {
            _val.validate();
        }
        if let Some(_val) = self.start_codeable_concept() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum GoalLifecycleStatus {
    Proposed,
    Planned,
    Accepted,
    Active,
    OnHold,
    Completed,
    Cancelled,
    EnteredInError,
    Rejected,
}

impl GoalLifecycleStatus {
    pub fn from_string(string: &str) -> Option<GoalLifecycleStatus> {
        match string {
            "proposed" => Some(GoalLifecycleStatus::Proposed),
            "planned" => Some(GoalLifecycleStatus::Planned),
            "accepted" => Some(GoalLifecycleStatus::Accepted),
            "active" => Some(GoalLifecycleStatus::Active),
            "on-hold" => Some(GoalLifecycleStatus::OnHold),
            "completed" => Some(GoalLifecycleStatus::Completed),
            "cancelled" => Some(GoalLifecycleStatus::Cancelled),
            "entered-in-error" => Some(GoalLifecycleStatus::EnteredInError),
            "rejected" => Some(GoalLifecycleStatus::Rejected),
            _ => None,
        }
    }
}
