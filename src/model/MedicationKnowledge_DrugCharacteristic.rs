#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_DrugCharacteristic<'a> {
    pub value: &'a Value,
}

impl MedicationKnowledge_DrugCharacteristic<'_> {
    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// A code specifying which characteristic of the medicine is being described (for
    /// example, colour, shape, imprint).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
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

    /// Description of the characteristic.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for valueBase64Binary
    pub fn _value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBase64Binary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Description of the characteristic.
    pub fn value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueBase64Binary") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_string() {
            _val.validate();
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
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
        if let Some(_val) = self.value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._value_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.value_base_6_4_binary() {}
        if let Some(_val) = self.id() {}
        return true;
    }
}
