#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::RequestGroup_Condition::RequestGroup_Condition;
use crate::model::RequestGroup_RelatedAction::RequestGroup_RelatedAction;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup_Action<'a> {
    pub value: &'a Value,
}

impl RequestGroup_Action<'_> {
    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub fn related_action(&self) -> Option<Vec<RequestGroup_RelatedAction>> {
        if let Some(Value::Array(val)) = self.value.get("relatedAction") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_RelatedAction { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The resource that is the target of the action (e.g. CommunicationRequest).
    pub fn resource(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("resource") {
            return Some(Reference { value: val });
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

    /// The type of action to perform (create, update, remove).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for cardinalityBehavior
    pub fn _cardinality_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cardinalityBehavior") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A short description of the action used to provide a summary to display to the
    /// user.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// An optional value describing when the action should be performed.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// An optional value describing when the action should be performed.
    pub fn timing_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("timingRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Defines the grouping behavior for the action and its children.
    pub fn grouping_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("groupingBehavior") {
            return Some(string);
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

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Defines expectations around whether an action is required.
    pub fn required_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requiredBehavior") {
            return Some(string);
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

    /// An expression that describes applicability criteria, or start/stop conditions
    /// for the action.
    pub fn condition(&self) -> Option<Vec<RequestGroup_Condition>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_Condition { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code that provides meaning for the action or action group. For example, a
    /// section may have a LOINC code for a section of a documentation template.
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

    /// Defines whether the action should usually be preselected.
    pub fn precheck_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("precheckBehavior") {
            return Some(string);
        }
        return None;
    }

    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// Defines whether the action can be selected multiple times.
    pub fn cardinality_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("cardinalityBehavior") {
            return Some(string);
        }
        return None;
    }

    /// The participant that should perform or be responsible for this action.
    pub fn participant(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Defines the selection behavior for the action and its children.
    pub fn selection_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("selectionBehavior") {
            return Some(string);
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

    /// A text equivalent of the action to be performed. This provides a human-
    /// interpretable description of the action when the definition is consumed by
    /// a system that might not be capable of interpreting it dynamically.
    pub fn text_equivalent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("textEquivalent") {
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

    /// The title of the action displayed to a user.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
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

    /// Extensions for selectionBehavior
    pub fn _selection_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_selectionBehavior") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Sub actions.
    pub fn action(&self) -> Option<Vec<RequestGroup_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_Action { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.related_action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._precheck_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.resource() {
            _val.validate();
        }
        if let Some(_val) = self.timing_age() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._required_behavior() {
            _val.validate();
        }
        if let Some(_val) = self._cardinality_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._timing_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.timing_timing() {
            _val.validate();
        }
        if let Some(_val) = self.documentation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.timing_range() {
            _val.validate();
        }
        if let Some(_val) = self.grouping_behavior() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self._prefix() {
            _val.validate();
        }
        if let Some(_val) = self.required_behavior() {}
        if let Some(_val) = self._text_equivalent() {
            _val.validate();
        }
        if let Some(_val) = self.condition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.precheck_behavior() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.cardinality_behavior() {}
        if let Some(_val) = self.participant() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.timing_duration() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.selection_behavior() {}
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self.text_equivalent() {}
        if let Some(_val) = self.timing_period() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._grouping_behavior() {
            _val.validate();
        }
        if let Some(_val) = self._selection_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.action() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
