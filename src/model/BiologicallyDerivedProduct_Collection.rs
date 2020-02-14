#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_Collection<'a> {
    pub value: &'a Value,
}

impl BiologicallyDerivedProduct_Collection<'_> {
    /// The patient or entity, such as a hospital or vendor in the case of a
    /// processed/manipulated/manufactured product, providing the product.
    pub fn source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("source") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Time of product collection.
    pub fn collected_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("collectedPeriod") {
            return Some(Period { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for collectedDateTime
    pub fn _collected_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_collectedDateTime") {
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

    /// Healthcare professional who is performing the collection.
    pub fn collector(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("collector") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Time of product collection.
    pub fn collected_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("collectedDateTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.source() {
            _val.validate();
        }
        if let Some(_val) = self.collected_period() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._collected_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.collector() {
            _val.validate();
        }
        if let Some(_val) = self.collected_date_time() {}
        return true;
    }
}
