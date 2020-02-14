#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Moiety<'a> {
    pub value: &'a Value,
}

impl SubstanceSpecification_Moiety<'_> {
    /// Extensions for amountString
    pub fn _amount_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amountString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Role that the moiety is playing.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Stereochemistry type.
    pub fn stereochemistry(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("stereochemistry") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Quantitative value for this moiety.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Textual name for this moiety substance.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    /// Optical activity type.
    pub fn optical_activity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("opticalActivity") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Identifier by which this moiety substance is known.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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

    /// Molecular formula.
    pub fn molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormula") {
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

    /// Extensions for molecularFormula
    pub fn _molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormula") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Quantitative value for this moiety.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._amount_string() {
            _val.validate();
        }
        if let Some(_val) = self.role() {
            _val.validate();
        }
        if let Some(_val) = self.stereochemistry() {
            _val.validate();
        }
        if let Some(_val) = self.amount_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.optical_activity() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.molecular_formula() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._molecular_formula() {
            _val.validate();
        }
        if let Some(_val) = self.amount_string() {}
        return true;
    }
}
