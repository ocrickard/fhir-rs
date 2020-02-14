#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::PlanDefinition_Target::PlanDefinition_Target;
use crate::model::RelatedArtifact::RelatedArtifact;
use serde_json::value::Value;

/// This resource allows for the definition of various types of plans as a sharable,
/// consumable, and executable artifact. The resource is general enough to support
/// the description of a broad range of clinical artifacts such as clinical decision
/// support rules, order sets and protocols.

#[derive(Debug)]
pub struct PlanDefinition_Goal<'a> {
    pub value: &'a Value,
}

impl PlanDefinition_Goal<'_> {
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

    /// Indicates what should be done and within what timeframe.
    pub fn target(&self) -> Option<Vec<PlanDefinition_Target>> {
        if let Some(Value::Array(val)) = self.value.get("target") {
            return Some(
                val.into_iter()
                    .map(|e| PlanDefinition_Target { value: e })
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

    /// The event after which the goal should begin being pursued.
    pub fn start(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("start") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Identifies the expected level of importance associated with reaching/sustaining
    /// the defined goal.
    pub fn priority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("priority") {
            return Some(CodeableConcept { value: val });
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

    /// Didactic or other informational resources associated with the goal that provide
    /// further supporting information about the goal. Information resources can include
    /// inline text commentary and links to web resources.
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

    /// Human-readable and/or coded description of a specific desired objective of care,
    /// such as "control blood pressure" or "negotiate an obstacle course" or "dance
    /// with child at wedding".
    pub fn description(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["description"],
        }
    }

    /// Identifies problems, conditions, issues, or concerns the goal is intended to
    /// address.
    pub fn addresses(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("addresses") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates a category the goal falls within.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.target() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.start() {
            _val.validate();
        }
        if let Some(_val) = self.priority() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.documentation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.description().validate();
        if let Some(_val) = self.addresses() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.validate();
        }
        return true;
    }
}
