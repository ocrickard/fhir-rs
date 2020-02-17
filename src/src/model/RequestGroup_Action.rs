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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup_Action<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RequestGroup_Action<'_> {
    pub fn new(value: &Value) -> RequestGroup_Action {
        RequestGroup_Action {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for cardinalityBehavior
    pub fn _cardinality_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cardinalityBehavior") {
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

    /// Extensions for groupingBehavior
    pub fn _grouping_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_groupingBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for precheckBehavior
    pub fn _precheck_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precheckBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
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

    /// Extensions for requiredBehavior
    pub fn _required_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requiredBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for selectionBehavior
    pub fn _selection_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_selectionBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for textEquivalent
    pub fn _text_equivalent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_textEquivalent") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for timingDateTime
    pub fn _timing_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Sub actions.
    pub fn action(&self) -> Option<Vec<RequestGroup_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_Action {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// A code that provides meaning for the action or action group. For example, a
    /// section may have a LOINC code for a section of a documentation template.
    pub fn code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
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

    /// An expression that describes applicability criteria, or start/stop conditions
    /// for the action.
    pub fn condition(&self) -> Option<Vec<RequestGroup_Condition>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_Condition {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Didactic or other informational resources associated with the action that can be
    /// provided to the CDS recipient. Information resources can include inline text
    /// commentary and links to web resources.
    pub fn documentation(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("documentation") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The participant that should perform or be responsible for this action.
    pub fn participant(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
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

    /// Defines whether the action should usually be preselected.
    pub fn precheck_behavior(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("precheckBehavior") {
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

    /// Indicates how quickly the action should be addressed with respect to other
    /// actions.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// A relationship to another action such as "before" or "30-60 minutes after start
    /// of".
    pub fn related_action(&self) -> Option<Vec<RequestGroup_RelatedAction>> {
        if let Some(Value::Array(val)) = self.value.get("relatedAction") {
            return Some(
                val.into_iter()
                    .map(|e| RequestGroup_RelatedAction {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The resource that is the target of the action (e.g. CommunicationRequest).
    pub fn resource(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("resource") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
    pub fn timing_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("timingAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
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

    /// An optional value describing when the action should be performed.
    pub fn timing_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("timingDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timingPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("timingRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An optional value describing when the action should be performed.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
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

    /// The type of action to perform (create, update, remove).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._cardinality_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._grouping_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._precheck_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._prefix() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._priority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._required_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._selection_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text_equivalent() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._timing_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cardinality_behavior() {}
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.documentation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.grouping_behavior() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precheck_behavior() {}
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.related_action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.required_behavior() {}
        if let Some(_val) = self.resource() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.selection_behavior() {}
        if let Some(_val) = self.text_equivalent() {}
        if let Some(_val) = self.timing_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self.timing_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.timing_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct RequestGroup_ActionBuilder {
    pub(crate) value: Value,
}

impl RequestGroup_ActionBuilder {
    pub fn build(&self) -> RequestGroup_Action {
        RequestGroup_Action {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RequestGroup_Action) -> RequestGroup_ActionBuilder {
        RequestGroup_ActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RequestGroup_ActionBuilder {
        let mut __value: Value = json!({});
        return RequestGroup_ActionBuilder { value: __value };
    }

    pub fn _cardinality_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_cardinalityBehavior"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _grouping_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_groupingBehavior"] = json!(val.value);
        return self;
    }

    pub fn _precheck_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_precheckBehavior"] = json!(val.value);
        return self;
    }

    pub fn _prefix<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_prefix"] = json!(val.value);
        return self;
    }

    pub fn _priority<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_priority"] = json!(val.value);
        return self;
    }

    pub fn _required_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_requiredBehavior"] = json!(val.value);
        return self;
    }

    pub fn _selection_behavior<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_selectionBehavior"] = json!(val.value);
        return self;
    }

    pub fn _text_equivalent<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_textEquivalent"] = json!(val.value);
        return self;
    }

    pub fn _timing_date_time<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_timingDateTime"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_ActionBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn action<'a>(
        &'a mut self,
        val: Vec<RequestGroup_Action>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["action"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cardinality_behavior<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["cardinalityBehavior"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut RequestGroup_ActionBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn condition<'a>(
        &'a mut self,
        val: Vec<RequestGroup_Condition>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["condition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn documentation<'a>(
        &'a mut self,
        val: Vec<RelatedArtifact>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["documentation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut RequestGroup_ActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn grouping_behavior<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["groupingBehavior"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precheck_behavior<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["precheckBehavior"] = json!(val);
        return self;
    }

    pub fn prefix<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["prefix"] = json!(val);
        return self;
    }

    pub fn priority<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["priority"] = json!(val);
        return self;
    }

    pub fn related_action<'a>(
        &'a mut self,
        val: Vec<RequestGroup_RelatedAction>,
    ) -> &'a mut RequestGroup_ActionBuilder {
        self.value["relatedAction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn required_behavior<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["requiredBehavior"] = json!(val);
        return self;
    }

    pub fn resource<'a>(&'a mut self, val: Reference) -> &'a mut RequestGroup_ActionBuilder {
        self.value["resource"] = json!(val.value);
        return self;
    }

    pub fn selection_behavior<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["selectionBehavior"] = json!(val);
        return self;
    }

    pub fn text_equivalent<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["textEquivalent"] = json!(val);
        return self;
    }

    pub fn timing_age<'a>(&'a mut self, val: Age) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingAge"] = json!(val.value);
        return self;
    }

    pub fn timing_date_time<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingDateTime"] = json!(val);
        return self;
    }

    pub fn timing_duration<'a>(&'a mut self, val: Duration) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingDuration"] = json!(val.value);
        return self;
    }

    pub fn timing_period<'a>(&'a mut self, val: Period) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingPeriod"] = json!(val.value);
        return self;
    }

    pub fn timing_range<'a>(&'a mut self, val: Range) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingRange"] = json!(val.value);
        return self;
    }

    pub fn timing_timing<'a>(&'a mut self, val: Timing) -> &'a mut RequestGroup_ActionBuilder {
        self.value["timingTiming"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_ActionBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut RequestGroup_ActionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
