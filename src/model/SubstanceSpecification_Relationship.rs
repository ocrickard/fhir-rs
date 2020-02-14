#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Relationship<'a> {
    pub value: &'a Value,
}

impl SubstanceSpecification_Relationship<'_> {
    /// For example "salt to parent", "active moiety", "starting material".
    pub fn relationship(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("relationship") {
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

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("amountRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// An operator for the amount, for example "average", "approximately", "less than".
    pub fn amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("amountType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A pointer to another substance, as a resource or just a representational code.
    pub fn substance_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("substanceReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for isDefining
    pub fn _is_defining(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDefining") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// For use when the numeric.
    pub fn amount_ratio_low_limit(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("amountRatioLowLimit") {
            return Some(Ratio { value: val });
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

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    /// A numeric factor for the relationship, for instance to express that the salt of
    /// a substance has some percentage of the active substance in relation to some
    /// other.
    pub fn amount_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("amountRatio") {
            return Some(Ratio { value: val });
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

    /// A pointer to another substance, as a resource or just a representational code.
    pub fn substance_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substanceCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// For example where an enzyme strongly bonds with a particular substance, this is
    /// a defining relationship for that enzyme, out of several possible substance
    /// relationships.
    pub fn is_defining(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDefining") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.relationship() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.amount_range() {
            _val.validate();
        }
        if let Some(_val) = self.amount_type() {
            _val.validate();
        }
        if let Some(_val) = self.substance_reference() {
            _val.validate();
        }
        if let Some(_val) = self.amount_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._is_defining() {
            _val.validate();
        }
        if let Some(_val) = self.amount_ratio_low_limit() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.amount_string() {}
        if let Some(_val) = self.amount_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.source() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.substance_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._amount_string() {
            _val.validate();
        }
        if let Some(_val) = self.is_defining() {}
        return true;
    }
}
