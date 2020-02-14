#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::SubstanceSpecification_Isotope::SubstanceSpecification_Isotope;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;
use crate::model::SubstanceSpecification_Representation::SubstanceSpecification_Representation;
use serde_json::value::Value;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Structure<'a> {
    pub value: &'a Value,
}

impl SubstanceSpecification_Structure<'_> {
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

    /// Stereochemistry type.
    pub fn stereochemistry(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("stereochemistry") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Applicable for single substances that contain a radionuclide or a non-natural
    /// isotopic ratio.
    pub fn isotope(&self) -> Option<Vec<SubstanceSpecification_Isotope>> {
        if let Some(Value::Array(val)) = self.value.get("isotope") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Isotope { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specified per moiety according to the Hill system, i.e. first C, then H, then
    /// alphabetical, each moiety separated by a dot.
    pub fn molecular_formula_by_moiety(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormulaByMoiety") {
            return Some(string);
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<SubstanceSpecification_MolecularWeight> {
        if let Some(val) = self.value.get("molecularWeight") {
            return Some(SubstanceSpecification_MolecularWeight { value: val });
        }
        return None;
    }

    /// Extensions for molecularFormulaByMoiety
    pub fn _molecular_formula_by_moiety(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormulaByMoiety") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Supporting literature.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Molecular formula.
    pub fn molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormula") {
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

    /// Extensions for molecularFormula
    pub fn _molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormula") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Molecular structural representation.
    pub fn representation(&self) -> Option<Vec<SubstanceSpecification_Representation>> {
        if let Some(Value::Array(val)) = self.value.get("representation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Representation { value: e })
                    .collect::<Vec<_>>(),
            );
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
        if let Some(_val) = self.stereochemistry() {
            _val.validate();
        }
        if let Some(_val) = self.isotope() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.molecular_formula_by_moiety() {}
        if let Some(_val) = self.molecular_weight() {
            _val.validate();
        }
        if let Some(_val) = self._molecular_formula_by_moiety() {
            _val.validate();
        }
        if let Some(_val) = self.source() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.optical_activity() {
            _val.validate();
        }
        if let Some(_val) = self.molecular_formula() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._molecular_formula() {
            _val.validate();
        }
        if let Some(_val) = self.representation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
