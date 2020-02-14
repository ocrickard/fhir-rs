#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::SpecimenDefinition_Additive::SpecimenDefinition_Additive;
use serde_json::value::Value;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_Container<'a> {
    pub value: &'a Value,
}

impl SpecimenDefinition_Container<'_> {
    /// Special processing that should be applied to the container for this kind of
    /// specimen.
    pub fn preparation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preparation") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for preparation
    pub fn _preparation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preparation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The minimum volume to be conditioned in the container.
    pub fn minimum_volume_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minimumVolumeString") {
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

    /// The minimum volume to be conditioned in the container.
    pub fn minimum_volume_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("minimumVolumeQuantity") {
            return Some(Quantity { value: val });
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

    /// The capacity (volume or other measure) of this kind of container.
    pub fn capacity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("capacity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Color of container cap.
    pub fn cap(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("cap") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for minimumVolumeString
    pub fn _minimum_volume_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minimumVolumeString") {
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

    /// Substance introduced in the kind of container to preserve, maintain or enhance
    /// the specimen. Examples: Formalin, Citrate, EDTA.
    pub fn additive(&self) -> Option<Vec<SpecimenDefinition_Additive>> {
        if let Some(Value::Array(val)) = self.value.get("additive") {
            return Some(
                val.into_iter()
                    .map(|e| SpecimenDefinition_Additive { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The type of material of the container.
    pub fn material(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("material") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The type of container used to contain this kind of specimen.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The textual description of the kind of container.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.preparation() {}
        if let Some(_val) = self._preparation() {
            _val.validate();
        }
        if let Some(_val) = self.minimum_volume_string() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.minimum_volume_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.capacity() {
            _val.validate();
        }
        if let Some(_val) = self.cap() {
            _val.validate();
        }
        if let Some(_val) = self._minimum_volume_string() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.additive() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.material() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        return true;
    }
}
