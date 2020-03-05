#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicationRequest_InitialFill::MedicationRequest_InitialFill;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An order or request for both supply of the medication and the instructions for
/// administration of the medication to a patient. The resource is called
/// "MedicationRequest" rather than "MedicationPrescription" or "MedicationOrder" to
/// generalize the use across inpatient and outpatient settings, including care
/// plans, etc., and to harmonize with workflow patterns.

#[derive(Debug)]
pub struct MedicationRequest_DispenseRequest<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationRequest_DispenseRequest<'_> {
    pub fn new(value: &Value) -> MedicationRequest_DispenseRequest {
        MedicationRequest_DispenseRequest {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for numberOfRepeatsAllowed
    pub fn _number_of_repeats_allowed(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfRepeatsAllowed") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The minimum period of time that must occur between dispenses of the medication.
    pub fn dispense_interval(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("dispenseInterval") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the period time over which the supplied product is expected to be
    /// used, or the length of time the dispense is expected to last.
    pub fn expected_supply_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("expectedSupplyDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
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

    /// Indicates the quantity or duration for the first dispense of the medication.
    pub fn initial_fill(&self) -> Option<MedicationRequest_InitialFill> {
        if let Some(val) = self.value.get("initialFill") {
            return Some(MedicationRequest_InitialFill {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An integer indicating the number of times, in addition to the original dispense,
    /// (aka refills or repeats) that the patient can receive the prescribed medication.
    /// Usage Notes: This integer does not include the original order dispense. This
    /// means that if an order indicates dispense 30 tablets plus "3 repeats", then the
    /// order can be dispensed a total of 4 times and the patient can receive a total of
    /// 120 tablets.  A prescriber may explicitly say that zero refills are permitted
    /// after the initial dispense.
    pub fn number_of_repeats_allowed(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberOfRepeatsAllowed") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Indicates the intended dispensing Organization specified by the prescriber.
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The amount that is to be dispensed for one fill.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This indicates the validity period of a prescription (stale dating the
    /// Prescription).
    pub fn validity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validityPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._number_of_repeats_allowed() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.dispense_interval() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.expected_supply_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.initial_fill() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.number_of_repeats_allowed() {}
        if let Some(_val) = self.performer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.validity_period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationRequest_DispenseRequestBuilder {
    pub(crate) value: Value,
}

impl MedicationRequest_DispenseRequestBuilder {
    pub fn build(&self) -> MedicationRequest_DispenseRequest {
        MedicationRequest_DispenseRequest {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationRequest_DispenseRequest,
    ) -> MedicationRequest_DispenseRequestBuilder {
        MedicationRequest_DispenseRequestBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationRequest_DispenseRequestBuilder {
        let mut __value: Value = json!({});
        return MedicationRequest_DispenseRequestBuilder { value: __value };
    }

    pub fn _number_of_repeats_allowed<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["_numberOfRepeatsAllowed"] = json!(val.value);
        return self;
    }

    pub fn dispense_interval<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["dispenseInterval"] = json!(val.value);
        return self;
    }

    pub fn expected_supply_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["expectedSupplyDuration"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn initial_fill<'a>(
        &'a mut self,
        val: MedicationRequest_InitialFill,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["initialFill"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_of_repeats_allowed<'a>(
        &'a mut self,
        val: u64,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["numberOfRepeatsAllowed"] = json!(val);
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["performer"] = json!(val.value);
        return self;
    }

    pub fn quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn validity_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut MedicationRequest_DispenseRequestBuilder {
        self.value["validityPeriod"] = json!(val.value);
        return self;
    }
}
