#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).

#[derive(Debug)]
pub struct MedicinalProduct_SpecialDesignation<'a> {
    pub value: &'a Value,
}

impl MedicinalProduct_SpecialDesignation<'_> {
    /// Animal species for which this applies.
    pub fn species(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("species") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The type of special designation, e.g. orphan drug, minor use.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
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

    /// Identifier for the designation, or procedure number.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The intended use of the product, e.g. prevention, treatment.
    pub fn intended_use(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("intendedUse") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Date when the designation was granted.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// For example granted, pending, expired or withdrawn.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Condition for which the medicinal use applies.
    pub fn indication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("indicationReference") {
            return Some(Reference { value: val });
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

    /// Condition for which the medicinal use applies.
    pub fn indication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("indicationCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.species() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.intended_use() {
            _val.validate();
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.status() {
            _val.validate();
        }
        if let Some(_val) = self.indication_reference() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.indication_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._date() {
            _val.validate();
        }
        return true;
    }
}
