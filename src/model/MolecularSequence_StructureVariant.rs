#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MolecularSequence_Inner::MolecularSequence_Inner;
use crate::model::MolecularSequence_Outer::MolecularSequence_Outer;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_StructureVariant<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_StructureVariant<'_> {
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

    /// Used to indicate if the outer and inner start-end values have the same meaning.
    pub fn exact(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exact") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for length
    pub fn _length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_length") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Structural variant outer.
    pub fn outer(&self) -> Option<MolecularSequence_Outer> {
        if let Some(val) = self.value.get("outer") {
            return Some(MolecularSequence_Outer { value: val });
        }
        return None;
    }

    /// Extensions for exact
    pub fn _exact(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exact") {
            return Some(Element { value: val });
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

    /// Length of the variant chromosome.
    pub fn length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("length") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Structural variant inner.
    pub fn inner(&self) -> Option<MolecularSequence_Inner> {
        if let Some(val) = self.value.get("inner") {
            return Some(MolecularSequence_Inner { value: val });
        }
        return None;
    }

    /// Information about chromosome structure variation DNA change type.
    pub fn variant_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("variantType") {
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
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.exact() {}
        if let Some(_val) = self._length() {
            _val.validate();
        }
        if let Some(_val) = self.outer() {
            _val.validate();
        }
        if let Some(_val) = self._exact() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.length() {}
        if let Some(_val) = self.inner() {
            _val.validate();
        }
        if let Some(_val) = self.variant_type() {
            _val.validate();
        }
        return true;
    }
}
