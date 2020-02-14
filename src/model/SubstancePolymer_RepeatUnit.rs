#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstanceAmount::SubstanceAmount;
use crate::model::SubstancePolymer_DegreeOfPolymerisation::SubstancePolymer_DegreeOfPolymerisation;
use crate::model::SubstancePolymer_StructuralRepresentation::SubstancePolymer_StructuralRepresentation;
use serde_json::value::Value;

/// Todo.

#[derive(Debug)]
pub struct SubstancePolymer_RepeatUnit<'a> {
    pub value: &'a Value,
}

impl SubstancePolymer_RepeatUnit<'_> {
    /// Todo.
    pub fn amount(&self) -> Option<SubstanceAmount> {
        if let Some(val) = self.value.get("amount") {
            return Some(SubstanceAmount { value: val });
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

    /// Todo.
    pub fn structural_representation(
        &self,
    ) -> Option<Vec<SubstancePolymer_StructuralRepresentation>> {
        if let Some(Value::Array(val)) = self.value.get("structuralRepresentation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_StructuralRepresentation { value: e })
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

    /// Todo.
    pub fn degree_of_polymerisation(&self) -> Option<Vec<SubstancePolymer_DegreeOfPolymerisation>> {
        if let Some(Value::Array(val)) = self.value.get("degreeOfPolymerisation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_DegreeOfPolymerisation { value: e })
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

    /// Extensions for repeatUnit
    pub fn _repeat_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_repeatUnit") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn repeat_unit(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("repeatUnit") {
            return Some(string);
        }
        return None;
    }

    /// Todo.
    pub fn orientation_of_polymerisation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("orientationOfPolymerisation") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.amount() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.structural_representation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.degree_of_polymerisation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._repeat_unit() {
            _val.validate();
        }
        if let Some(_val) = self.repeat_unit() {}
        if let Some(_val) = self.orientation_of_polymerisation() {
            _val.validate();
        }
        return true;
    }
}
