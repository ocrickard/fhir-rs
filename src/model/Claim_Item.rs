#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Claim_Detail::Claim_Detail;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Money::Money;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A provider issued list of professional services and products which have been
/// provided, or are to be provided, to a patient which is sent to an insurer for
/// reimbursement.

#[derive(Debug)]
pub struct Claim_Item<'a> {
    pub value: &'a Value,
}

impl Claim_Item<'_> {
    /// Procedures applicable for this service or product.
    pub fn procedure_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("procedureSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
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

    /// Physical service site on the patient (limb, tooth, etc.).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The type of revenue or cost center providing the product and/or service.
    pub fn revenue(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("revenue") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("servicedPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for procedureSequence
    pub fn _procedure_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_procedureSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("locationCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Where the product or service was provided.
    pub fn location_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("locationAddress") {
            return Some(Address { value: val });
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

    /// Exceptions, special conditions and supporting information applicable for this
    /// service or product.
    pub fn information_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("informationSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A region or surface of the bodySite, e.g. limb region or tooth surface(s).
    pub fn sub_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subSite") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date or dates when the service or product was supplied, performed or
    /// completed.
    pub fn serviced_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("servicedDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for servicedDate
    pub fn _serviced_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_servicedDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A claim detail line. Either a simple (a product or service) or a 'group' of sub-
    /// details which are simple items.
    pub fn detail(&self) -> Option<Vec<Claim_Detail>> {
        if let Some(Value::Array(val)) = self.value.get("detail") {
            return Some(
                val.into_iter()
                    .map(|e| Claim_Detail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies the program under which this may be recovered.
    pub fn program_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("programCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for diagnosisSequence
    pub fn _diagnosis_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_diagnosisSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// CareTeam members related to this service or product.
    pub fn care_team_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("careTeamSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Encounters during which this Claim was created or to which the creation of
    /// this record is tightly associated.
    pub fn encounter(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("encounter") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A real number that represents a multiplier used in determining the overall value
    /// of services delivered and/or goods received. The concept of a Factor allows for
    /// a discount or surcharge multiplier to be applied to a monetary amount.
    pub fn factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("factor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// If the item is not a group then this is the fee for the product or service,
    /// otherwise this is the total of the fees for the details of the group.
    pub fn unit_price(&self) -> Option<Money> {
        if let Some(val) = self.value.get("unitPrice") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// The quantity times the unit price for an additional service or product or
    /// charge.
    pub fn net(&self) -> Option<Money> {
        if let Some(val) = self.value.get("net") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// Unique Device Identifiers associated with this line item.
    pub fn udi(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("udi") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Diagnosis applicable for this service or product.
    pub fn diagnosis_sequence(&self) -> Option<Vec<i64>> {
        if let Some(Value::Array(val)) = self.value.get("diagnosisSequence") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_i64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for careTeamSequence
    pub fn _care_team_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_careTeamSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// Extensions for informationSequence
    pub fn _information_sequence(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_informationSequence") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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

    /// Where the product or service was provided.
    pub fn location_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("locationReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A number to uniquely identify item entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// When the value is a group code then this item collects a set of related claim
    /// details, otherwise this contains the product, service, drug or other billing
    /// code for the item.
    pub fn product_or_service(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["productOrService"],
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.procedure_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.body_site() {
            _val.validate();
        }
        if let Some(_val) = self.revenue() {
            _val.validate();
        }
        if let Some(_val) = self.serviced_period() {
            _val.validate();
        }
        if let Some(_val) = self._procedure_sequence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.location_address() {
            _val.validate();
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.information_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.sub_site() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.serviced_date() {}
        if let Some(_val) = self._serviced_date() {
            _val.validate();
        }
        if let Some(_val) = self._sequence() {
            _val.validate();
        }
        if let Some(_val) = self.detail() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.program_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._factor() {
            _val.validate();
        }
        if let Some(_val) = self._diagnosis_sequence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.care_team_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.encounter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.factor() {}
        if let Some(_val) = self.unit_price() {
            _val.validate();
        }
        if let Some(_val) = self.net() {
            _val.validate();
        }
        if let Some(_val) = self.udi() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.diagnosis_sequence() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._care_team_sequence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._information_sequence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.validate();
        }
        if let Some(_val) = self.modifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.location_reference() {
            _val.validate();
        }
        if let Some(_val) = self.sequence() {}
        let _ = self.product_or_service().validate();
        return true;
    }
}
