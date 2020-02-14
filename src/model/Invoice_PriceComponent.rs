#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use serde_json::value::Value;

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.

#[derive(Debug)]
pub struct Invoice_PriceComponent<'a> {
    pub value: &'a Value,
}

impl Invoice_PriceComponent<'_> {
    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The factor that has been applied on the base price for calculating this
    /// component.
    pub fn factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for factor
    pub fn _factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_factor") {
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

    /// The amount calculated for this component.
    pub fn amount(&self) -> Option<Money> {
        if let Some(val) = self.value.get("amount") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// This code identifies the type of the component.
    pub fn fhir_type(&self) -> Option<Invoice_PriceComponentType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Invoice_PriceComponentType::from_string(&val).unwrap());
        }
        return None;
    }

    /// A code that identifies the component. Codes may be used to differentiate between
    /// kinds of taxes, surcharges, discounts etc.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self._factor() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.amount() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Invoice_PriceComponentType {
    Base,
    Surcharge,
    Deduction,
    Discount,
    Tax,
    Informational,
}

impl Invoice_PriceComponentType {
    pub fn from_string(string: &str) -> Option<Invoice_PriceComponentType> {
        match string {
            "base" => Some(Invoice_PriceComponentType::Base),
            "surcharge" => Some(Invoice_PriceComponentType::Surcharge),
            "deduction" => Some(Invoice_PriceComponentType::Deduction),
            "discount" => Some(Invoice_PriceComponentType::Discount),
            "tax" => Some(Invoice_PriceComponentType::Tax),
            "informational" => Some(Invoice_PriceComponentType::Informational),
            _ => None,
        }
    }
}
