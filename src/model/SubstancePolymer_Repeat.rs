#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstancePolymer_RepeatUnit::SubstancePolymer_RepeatUnit;
use serde_json::value::Value;

/// Todo.

#[derive(Debug)]
pub struct SubstancePolymer_Repeat<'a> {
    pub value: &'a Value,
}

impl SubstancePolymer_Repeat<'_> {
    /// Todo.
    pub fn repeat_unit(&self) -> Option<Vec<SubstancePolymer_RepeatUnit>> {
        if let Some(Value::Array(val)) = self.value.get("repeatUnit") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_RepeatUnit { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Todo.
    pub fn number_of_units(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfUnits") {
            return Some(val.as_i64().unwrap());
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
    pub fn repeat_unit_amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("repeatUnitAmountType") {
            return Some(CodeableConcept { value: val });
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

    /// Todo.
    pub fn average_molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("averageMolecularFormula") {
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

    /// Extensions for numberOfUnits
    pub fn _number_of_units(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfUnits") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for averageMolecularFormula
    pub fn _average_molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_averageMolecularFormula") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.repeat_unit() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.number_of_units() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.repeat_unit_amount_type() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.average_molecular_formula() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._number_of_units() {
            _val.validate();
        }
        if let Some(_val) = self._average_molecular_formula() {
            _val.validate();
        }
        return true;
    }
}
