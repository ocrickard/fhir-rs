#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::value::Value;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_RelatedAction<'a> {
    pub value: &'a Value,
}

impl PlanDefinition_RelatedAction<'_> {
    /// The element id of the related action.
    pub fn action_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actionId") {
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

    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub fn offset_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("offsetDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub fn offset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("offsetRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Extensions for actionId
    pub fn _action_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actionId") {
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

    /// Extensions for relationship
    pub fn _relationship(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relationship") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The relationship of this action to the related action.
    pub fn relationship(&self) -> Option<PlanDefinition_RelatedActionRelationship> {
        if let Some(Value::String(val)) = self.value.get("relationship") {
            return Some(PlanDefinition_RelatedActionRelationship::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.action_id() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.offset_duration() {
            _val.validate();
        }
        if let Some(_val) = self.offset_range() {
            _val.validate();
        }
        if let Some(_val) = self._action_id() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._relationship() {
            _val.validate();
        }
        if let Some(_val) = self.relationship() {}
        return true;
    }
}

#[derive(Debug)]
pub enum PlanDefinition_RelatedActionRelationship {
    BeforeStart,
    Before,
    BeforeEnd,
    ConcurrentWithStart,
    Concurrent,
    ConcurrentWithEnd,
    AfterStart,
    After,
    AfterEnd,
}

impl PlanDefinition_RelatedActionRelationship {
    pub fn from_string(string: &str) -> Option<PlanDefinition_RelatedActionRelationship> {
        match string {
            "before-start" => Some(PlanDefinition_RelatedActionRelationship::BeforeStart),
            "before" => Some(PlanDefinition_RelatedActionRelationship::Before),
            "before-end" => Some(PlanDefinition_RelatedActionRelationship::BeforeEnd),
            "concurrent-with-start" => {
                Some(PlanDefinition_RelatedActionRelationship::ConcurrentWithStart)
            }
            "concurrent" => Some(PlanDefinition_RelatedActionRelationship::Concurrent),
            "concurrent-with-end" => {
                Some(PlanDefinition_RelatedActionRelationship::ConcurrentWithEnd)
            }
            "after-start" => Some(PlanDefinition_RelatedActionRelationship::AfterStart),
            "after" => Some(PlanDefinition_RelatedActionRelationship::After),
            "after-end" => Some(PlanDefinition_RelatedActionRelationship::AfterEnd),
            _ => None,
        }
    }
}
