#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.

#[derive(Debug)]
pub struct Group_Characteristic<'a> {
    pub value: &'a Value,
}

impl Group_Characteristic<'_> {
    /// Extensions for exclude
    pub fn _exclude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exclude") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
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

    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The period over which the characteristic is tested; e.g. the patient had an
    /// operation during the month of June.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A code that identifies the kind of trait being asserted.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["code"],
        }
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

    /// If true, indicates the characteristic is one that is NOT held by members of the
    /// group.
    pub fn exclude(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exclude") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the trait that holds (or does not hold - see 'exclude') for members
    /// of the group.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._exclude() {
            _val.validate();
        }
        if let Some(_val) = self.value_range() {
            _val.validate();
        }
        if let Some(_val) = self.value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value_reference() {
            _val.validate();
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.period() {
            _val.validate();
        }
        let _ = self.code().validate();
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
        if let Some(_val) = self.exclude() {}
        if let Some(_val) = self._value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        return true;
    }
}
