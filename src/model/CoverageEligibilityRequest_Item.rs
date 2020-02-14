#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CoverageEligibilityRequest_Diagnosis::CoverageEligibilityRequest_Diagnosis;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// The CoverageEligibilityRequest provides patient and insurance coverage
/// information to an insurer for them to respond, in the form of an
/// CoverageEligibilityResponse, with information regarding whether the stated
/// coverage is valid and in-force and optionally to provide the insurance details
/// of the policy.

#[derive(Debug)]
pub struct CoverageEligibilityRequest_Item<'a> {
    pub value: &'a Value,
}

impl CoverageEligibilityRequest_Item<'_> {
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

    /// Extensions for supportingInfoSequence
    pub fn _supporting_info_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_supportingInfoSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Patient diagnosis for which care is sought.
    pub fn diagnosis(&self) -> Option<Vec<CoverageEligibilityRequest_Diagnosis>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosis") {
            return Some(
                val.into_iter()
                    .map(|e| CoverageEligibilityRequest_Diagnosis { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Exceptions, special conditions and supporting information applicable for this
    /// service or product line.
    pub fn supporting_info_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfoSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This contains the product, service, drug or other billing code for the item.
    pub fn product_or_service(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productOrService") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The practitioner who is responsible for the product or service to be rendered to
    /// the patient.
    pub fn provider(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("provider") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The amount charged to the patient by the provider for a single unit.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money { value: val });
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

    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Facility where the services will be provided.
    pub fn facility(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("facility") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The plan/proposal/order describing the proposed service in detail.
    pub fn detail(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of repetitions of a service or product.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._supporting_info_sequence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.diagnosis() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.supporting_info_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.product_or_service() {
            _val.validate();
        }
        if let Some(_val) = self.modifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.provider() {
            _val.validate();
        }
        if let Some(_val) = self.unit_price() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.validate();
        }
        if let Some(_val) = self.facility() {
            _val.validate();
        }
        if let Some(_val) = self.detail() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        return true;
    }
}
