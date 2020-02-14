#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Todo.

#[derive(Debug)]
pub struct SubstanceReferenceInformation_Target<'a> {
    pub value: &'a Value,
}

impl SubstanceReferenceInformation_Target<'_> {
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

    /// Todo.
    pub fn interaction(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("interaction") {
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
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn organism_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("organismType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn target(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("target") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn amount_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("amountRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for amountString
    pub fn _amount_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amountString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("amountType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Todo.
    pub fn organism(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("organism") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Todo.
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.interaction() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.organism_type() {
            _val.validate();
        }
        if let Some(_val) = self.target() {
            _val.validate();
        }
        if let Some(_val) = self.amount_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.amount_range() {
            _val.validate();
        }
        if let Some(_val) = self.amount_string() {}
        if let Some(_val) = self._amount_string() {
            _val.validate();
        }
        if let Some(_val) = self.amount_type() {
            _val.validate();
        }
        if let Some(_val) = self.organism() {
            _val.validate();
        }
        if let Some(_val) = self.source() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
