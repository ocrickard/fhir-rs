#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::PlanDefinition_Condition::PlanDefinition_Condition;
use crate::model::PlanDefinition_DynamicValue::PlanDefinition_DynamicValue;
use crate::model::PlanDefinition_Participant::PlanDefinition_Participant;
use crate::model::PlanDefinition_RelatedAction::PlanDefinition_RelatedAction;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::Timing::Timing;
use crate::model::TriggerDefinition::TriggerDefinition;
use serde_json::value::Value;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_Action<'a> {
    pub value: &'a Value,
}

impl PlanDefinition_Action<'_> {
    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timingPeriod") {
            return Some(Period { value: val });
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

    /// A user-visible prefix for the action.
    pub fn prefix(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("prefix") {
            return Some(string);
        }
        return None;
    }

    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub fn related_action(&self) -> Option<Vec<PlanDefinition_RelatedAction>> {
        if let Some(Value::Array(val)) = self.value.get("relatedAction") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_RelatedAction { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of action to perform (create, update, remove).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for selectionBehavior
    pub fn _selection_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_selectionBehavior") {
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

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The title of the action displayed to a user.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Identifies goals that this action supports. The reference must be to a goal
    /// element defined within this plan definition.
    pub fn goal_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("goalId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code or group definition that describes the intended subject of the action and
    /// its children, if any.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// A description of when the action should be triggered.
    pub fn trigger(&self) -> Option<Vec<TriggerDefinition>> {
        if let Some(Value::Array(val)) = self.value.get("trigger") {
            return Some(
                val.into_iter()
                    .map(|e| TriggerDefinition { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for groupingBehavior
    pub fn _grouping_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_groupingBehavior") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for precheckBehavior
    pub fn _precheck_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precheckBehavior") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A reference to an ActivityDefinition that describes the action to be taken in
    /// detail, or a PlanDefinition that describes a series of actions to be taken.
    pub fn definition_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionUri") {
            return Some(string);
        }
        return None;
    }

    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub fn transform(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("transform") {
            return Some(string);
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Customizations that should be applied to the statically defined resource. For
    /// example, if the dosage of a medication must be computed based on the patient's
    /// weight, a customization would be used to specify an expression that calculated
    /// the weight, and the path on the resource that would contain the result.
    pub fn dynamic_value(&self) -> Option<Vec<PlanDefinition_DynamicValue>> {
        if let Some(Value::Array(val)) = self.value.get("dynamicValue") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_DynamicValue { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A text equivalent of the action to be performed. This provides a human-
    /// interpretable description of the action when the definition is consumed by
    /// a system that might not be capable of interpreting it dynamically.
    pub fn text_equivalent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textEquivalent") {
            return Some(string);
        }
        return None;
    }

    /// An expression that describes applicability criteria or start/stop conditions for
    /// the action.
    pub fn condition(&self) -> Option<Vec<PlanDefinition_Condition>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Condition { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines input data requirements for the action.
    pub fn input(&self) -> Option<Vec<DataRequirement>> {
        if let Some(Value::Array(val)) = self.value.get("input") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("timingAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// A brief description of the action used to provide a summary to display to the
    /// user.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("timingRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Defines the required behavior for the action.
    pub fn required_behavior(&self) -> Option<PlanDefinition_ActionRequiredBehavior> {
        if let Some(Value::String(val)) = self.value.get("requiredBehavior") {
            return Some(PlanDefinition_ActionRequiredBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for cardinalityBehavior
    pub fn _cardinality_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cardinalityBehavior") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for definitionUri
    pub fn _definition_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates who should participate in performing the action described.
    pub fn participant(&self) -> Option<Vec<PlanDefinition_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Participant { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code or group definition that describes the intended subject of the action and
    /// its children, if any.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("timingDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Defines the grouping behavior for the action and its children.
    pub fn grouping_behavior(&self) -> Option<PlanDefinition_ActionGroupingBehavior> {
        if let Some(Value::String(val)) = self.value.get("groupingBehavior") {
            return Some(PlanDefinition_ActionGroupingBehavior::from_string(&val).unwrap());
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

    /// Defines the selection behavior for the action and its children.
    pub fn selection_behavior(&self) -> Option<PlanDefinition_ActionSelectionBehavior> {
        if let Some(Value::String(val)) = self.value.get("selectionBehavior") {
            return Some(PlanDefinition_ActionSelectionBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for goalId
    pub fn _goal_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_goalId") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of why this action is necessary or appropriate.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Didactic or other informational resources associated with the action that can be
    /// provided to the CDS recipient. Information resources can include inline text
    /// commentary and links to web resources.
    pub fn documentation(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("documentation") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for requiredBehavior
    pub fn _required_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requiredBehavior") {
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

    /// Extensions for definitionCanonical
    pub fn _definition_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Sub actions that are contained within the action. The behavior of this action
    /// determines the functionality of the sub-actions. For example, a selection
    /// behavior of at-most-one indicates that of the sub-actions, at most one may be
    /// chosen as part of realizing the action definition.
    pub fn action(&self) -> Option<Vec<PlanDefinition_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Action { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// A code that provides meaning for the action or action group. For example, a
    /// section may have a LOINC code for the section of a documentation template.
    pub fn code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
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

    /// Extensions for textEquivalent
    pub fn _text_equivalent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textEquivalent") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Defines the outputs of the action, if any.
    pub fn output(&self) -> Option<Vec<DataRequirement>> {
        if let Some(Value::Array(val)) = self.value.get("output") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Defines whether the action should usually be preselected.
    pub fn precheck_behavior(&self) -> Option<PlanDefinition_ActionPrecheckBehavior> {
        if let Some(Value::String(val)) = self.value.get("precheckBehavior") {
            return Some(PlanDefinition_ActionPrecheckBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// Defines whether the action can be selected multiple times.
    pub fn cardinality_behavior(&self) -> Option<PlanDefinition_ActionCardinalityBehavior> {
        if let Some(Value::String(val)) = self.value.get("cardinalityBehavior") {
            return Some(PlanDefinition_ActionCardinalityBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// A reference to an ActivityDefinition that describes the action to be taken in
    /// detail, or a PlanDefinition that describes a series of actions to be taken.
    pub fn definition_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionCanonical") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.timing_period() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.related_action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._selection_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._prefix() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.goal_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.subject_reference() {
            _val.validate();
        }
        if let Some(_val) = self.timing_timing() {
            _val.validate();
        }
        if let Some(_val) = self.trigger() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._grouping_behavior() {
            _val.validate();
        }
        if let Some(_val) = self._precheck_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.definition_uri() {}
        if let Some(_val) = self.transform() {}
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self.dynamic_value() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text_equivalent() {}
        if let Some(_val) = self.condition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.input() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.timing_age() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.timing_range() {
            _val.validate();
        }
        if let Some(_val) = self.required_behavior() {}
        if let Some(_val) = self._cardinality_behavior() {
            _val.validate();
        }
        if let Some(_val) = self._definition_uri() {
            _val.validate();
        }
        if let Some(_val) = self.participant() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.timing_duration() {
            _val.validate();
        }
        if let Some(_val) = self.grouping_behavior() {}
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.selection_behavior() {}
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._goal_id() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.documentation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._required_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._definition_canonical() {
            _val.validate();
        }
        if let Some(_val) = self.action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._timing_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._text_equivalent() {
            _val.validate();
        }
        if let Some(_val) = self.output() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.precheck_behavior() {}
        if let Some(_val) = self.cardinality_behavior() {}
        if let Some(_val) = self.definition_canonical() {}
        return true;
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ActionRequiredBehavior {
    Must,
    Could,
    MustUnlessDocumented,
}

impl PlanDefinition_ActionRequiredBehavior {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ActionRequiredBehavior> {
        match string {
            "must" => Some(PlanDefinition_ActionRequiredBehavior::Must),
            "could" => Some(PlanDefinition_ActionRequiredBehavior::Could),
            "must-unless-documented" => {
                Some(PlanDefinition_ActionRequiredBehavior::MustUnlessDocumented)
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ActionGroupingBehavior {
    VisualGroup,
    LogicalGroup,
    SentenceGroup,
}

impl PlanDefinition_ActionGroupingBehavior {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ActionGroupingBehavior> {
        match string {
            "visual-group" => Some(PlanDefinition_ActionGroupingBehavior::VisualGroup),
            "logical-group" => Some(PlanDefinition_ActionGroupingBehavior::LogicalGroup),
            "sentence-group" => Some(PlanDefinition_ActionGroupingBehavior::SentenceGroup),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ActionSelectionBehavior {
    Any,
    All,
    AllOrNone,
    ExactlyOne,
    AtMostOne,
    OneOrMore,
}

impl PlanDefinition_ActionSelectionBehavior {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ActionSelectionBehavior> {
        match string {
            "any" => Some(PlanDefinition_ActionSelectionBehavior::Any),
            "all" => Some(PlanDefinition_ActionSelectionBehavior::All),
            "all-or-none" => Some(PlanDefinition_ActionSelectionBehavior::AllOrNone),
            "exactly-one" => Some(PlanDefinition_ActionSelectionBehavior::ExactlyOne),
            "at-most-one" => Some(PlanDefinition_ActionSelectionBehavior::AtMostOne),
            "one-or-more" => Some(PlanDefinition_ActionSelectionBehavior::OneOrMore),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ActionPrecheckBehavior {
    Yes,
    No,
}

impl PlanDefinition_ActionPrecheckBehavior {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ActionPrecheckBehavior> {
        match string {
            "yes" => Some(PlanDefinition_ActionPrecheckBehavior::Yes),
            "no" => Some(PlanDefinition_ActionPrecheckBehavior::No),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum PlanDefinition_ActionCardinalityBehavior {
    Single,
    Multiple,
}

impl PlanDefinition_ActionCardinalityBehavior {
    pub fn from_string(string: &str) -> Option<PlanDefinition_ActionCardinalityBehavior> {
        match string {
            "single" => Some(PlanDefinition_ActionCardinalityBehavior::Single),
            "multiple" => Some(PlanDefinition_ActionCardinalityBehavior::Multiple),
            _ => None,
        }
    }
}
