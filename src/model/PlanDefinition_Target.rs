#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use serde_json::value::Value;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_Target<'a> {
    pub value: &'a Value,
}

impl PlanDefinition_Target<'_> {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
    /// range can be specified. When a low value is missing, it indicates that the goal
    /// is achieved at any value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any value at or
    /// above the low value.
    pub fn detail_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("detailRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
    /// range can be specified. When a low value is missing, it indicates that the goal
    /// is achieved at any value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any value at or
    /// above the low value.
    pub fn detail_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("detailCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The target value of the measure to be achieved to signify fulfillment of the
    /// goal, e.g. 150 pounds or 7.0%. Either the high or low or both values of the
    /// range can be specified. When a low value is missing, it indicates that the goal
    /// is achieved at any value at or below the high value. Similarly, if the high
    /// value is missing, it indicates that the goal is achieved at any value at or
    /// above the low value.
    pub fn detail_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("detailQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Indicates the timeframe after the start of the goal in which the goal should be
    /// met.
    pub fn due(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("due") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// The parameter whose value is to be tracked, e.g. body weight, blood pressure, or
    /// hemoglobin A1c level.
    pub fn measure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("measure") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.detail_range() {
            _val.validate();
        }
        if let Some(_val) = self.detail_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.detail_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.due() {
            _val.validate();
        }
        if let Some(_val) = self.measure() {
            _val.validate();
        }
        return true;
    }
}
