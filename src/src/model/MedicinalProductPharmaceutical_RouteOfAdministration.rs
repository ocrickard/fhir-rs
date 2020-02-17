#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductPharmaceutical_TargetSpecies::MedicinalProductPharmaceutical_TargetSpecies;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A pharmaceutical product described in terms of its composition and dose form.

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_RouteOfAdministration<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductPharmaceutical_RouteOfAdministration<'_> {
    pub fn new(value: &Value) -> MedicinalProductPharmaceutical_RouteOfAdministration {
        MedicinalProductPharmaceutical_RouteOfAdministration {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Coded expression for the route.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The first dose (dose quantity) administered in humans can be specified, for a
    /// product under investigation, using a numerical value and its unit of
    /// measurement.
    pub fn first_dose(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("firstDose") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
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

    /// The maximum dose per day (maximum dose quantity to be administered in any one
    /// 24-h period) that can be administered as per the protocol referenced in the
    /// clinical trial authorisation.
    pub fn max_dose_per_day(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxDosePerDay") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The maximum dose per treatment period that can be administered as per the
    /// protocol referenced in the clinical trial authorisation.
    pub fn max_dose_per_treatment_period(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("maxDosePerTreatmentPeriod") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The maximum single dose that can be administered as per the protocol of a
    /// clinical trial can be specified using a numerical value and its unit of
    /// measurement.
    pub fn max_single_dose(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxSingleDose") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The maximum treatment period during which an Investigational Medicinal Product
    /// can be administered as per the protocol referenced in the clinical trial
    /// authorisation.
    pub fn max_treatment_period(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("maxTreatmentPeriod") {
            return Some(Duration {
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

    /// A species for which this route applies.
    pub fn target_species(&self) -> Option<Vec<MedicinalProductPharmaceutical_TargetSpecies>> {
        if let Some(Value::Array(val)) = self.value.get("targetSpecies") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductPharmaceutical_TargetSpecies {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.first_dose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max_dose_per_day() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.max_dose_per_treatment_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.max_single_dose() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.max_treatment_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.target_species() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
    pub fn build(&self) -> MedicinalProductPharmaceutical_RouteOfAdministration {
        MedicinalProductPharmaceutical_RouteOfAdministration {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductPharmaceutical_RouteOfAdministration,
    ) -> MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        code: CodeableConcept,
    ) -> MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return MedicinalProductPharmaceutical_RouteOfAdministrationBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn first_dose<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["firstDose"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max_dose_per_day<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["maxDosePerDay"] = json!(val.value);
        return self;
    }

    pub fn max_dose_per_treatment_period<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["maxDosePerTreatmentPeriod"] = json!(val.value);
        return self;
    }

    pub fn max_single_dose<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["maxSingleDose"] = json!(val.value);
        return self;
    }

    pub fn max_treatment_period<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["maxTreatmentPeriod"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn target_species<'a>(
        &'a mut self,
        val: Vec<MedicinalProductPharmaceutical_TargetSpecies>,
    ) -> &'a mut MedicinalProductPharmaceutical_RouteOfAdministrationBuilder {
        self.value["targetSpecies"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
