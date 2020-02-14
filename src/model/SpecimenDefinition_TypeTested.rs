#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SpecimenDefinition_Container::SpecimenDefinition_Container;
use crate::model::SpecimenDefinition_Handling::SpecimenDefinition_Handling;
use serde_json::value::Value;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_TypeTested<'a> {
    pub value: &'a Value,
}

impl SpecimenDefinition_TypeTested<'_> {
    /// Extensions for preference
    pub fn _preference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preference") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for isDerived
    pub fn _is_derived(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDerived") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The preference for this type of conditioned specimen.
    pub fn preference(&self) -> Option<SpecimenDefinition_TypeTestedPreference> {
        if let Some(Value::String(val)) = self.value.get("preference") {
            return Some(SpecimenDefinition_TypeTestedPreference::from_string(&val).unwrap());
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

    /// Requirements for delivery and special handling of this kind of conditioned
    /// specimen.
    pub fn requirement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirement") {
            return Some(string);
        }
        return None;
    }

    /// The kind of specimen conditioned for testing expected by lab.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for requirement
    pub fn _requirement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirement") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The usual time that a specimen of this kind is retained after the ordered tests
    /// are completed, for the purpose of additional testing.
    pub fn retention_time(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("retentionTime") {
            return Some(Duration { value: val });
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

    /// Primary of secondary specimen.
    pub fn is_derived(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDerived") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The specimen's container.
    pub fn container(&self) -> Option<SpecimenDefinition_Container> {
        if let Some(val) = self.value.get("container") {
            return Some(SpecimenDefinition_Container { value: val });
        }
        return None;
    }

    /// Criterion for rejection of the specimen in its container by the laboratory.
    pub fn rejection_criterion(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("rejectionCriterion") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Set of instructions for preservation/transport of the specimen at a defined
    /// temperature interval, prior the testing process.
    pub fn handling(&self) -> Option<Vec<SpecimenDefinition_Handling>> {
        if let Some(Value::Array(val)) = self.value.get("handling") {
            return Some(
                val.into_iter()
                    .map(|e| SpecimenDefinition_Handling { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._preference() {
            _val.validate();
        }
        if let Some(_val) = self._is_derived() {
            _val.validate();
        }
        if let Some(_val) = self.preference() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.requirement() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._requirement() {
            _val.validate();
        }
        if let Some(_val) = self.retention_time() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.is_derived() {}
        if let Some(_val) = self.container() {
            _val.validate();
        }
        if let Some(_val) = self.rejection_criterion() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.handling() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum SpecimenDefinition_TypeTestedPreference {
    Preferred,
    Alternate,
}

impl SpecimenDefinition_TypeTestedPreference {
    pub fn from_string(string: &str) -> Option<SpecimenDefinition_TypeTestedPreference> {
        match string {
            "preferred" => Some(SpecimenDefinition_TypeTestedPreference::Preferred),
            "alternate" => Some(SpecimenDefinition_TypeTestedPreference::Alternate),
            _ => None,
        }
    }
}
