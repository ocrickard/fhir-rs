#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the event of a patient consuming or otherwise being administered a
/// medication.  This may be as simple as swallowing a tablet or it may be a long
/// running infusion.  Related resources tie this event to the authorizing
/// prescription, and the specific encounter between patient and health care
/// practitioner.

#[derive(Debug)]
pub struct MedicationAdministration_Dosage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationAdministration_Dosage<'_> {
    pub fn new(value: &Value) -> MedicationAdministration_Dosage {
        MedicationAdministration_Dosage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The amount of the medication given at one administration event.   Use this value
    /// when the administration is essentially an instantaneous event such as a
    /// swallowing a tablet or giving an injection.
    pub fn dose(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("dose") {
            return Some(Quantity {
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

    /// A coded value indicating the method by which the medication is intended to be or
    /// was introduced into or on the body.  This attribute will most often NOT be
    /// populated.  It is most commonly used for injections.  For example, Slow Push,
    /// Deep IV.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept {
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

    /// Identifies the speed with which the medication was or will be introduced into
    /// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or 100
    /// ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
    /// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
    pub fn rate_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("rateQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Identifies the speed with which the medication was or will be introduced into
    /// the patient.  Typically, the rate for an infusion e.g. 100 ml per 1 hour or 100
    /// ml/hr.  May also be expressed as a rate per unit of time, e.g. 500 ml per 2
    /// hours.  Other examples:  200 mcg/min or 200 mcg/1 minute; 1 liter/8 hours.
    pub fn rate_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("rateRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code specifying the route or physiological path of administration of a
    /// therapeutic agent into or onto the patient.  For example, topical, intravenous,
    /// etc.
    pub fn route(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("route") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A coded specification of the anatomic site where the medication first entered
    /// the body.  For example, "left arm".
    pub fn site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("site") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Free text dosage can be used for cases where the dosage administered is too
    /// complex to code. When coded dosage is present, the free text dosage may still be
    /// present for display to humans.    The dosage instructions should reflect the
    /// dosage of the medication that was administered.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.dose() {
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
        if let Some(_val) = self.method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rate_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.rate_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.route() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationAdministration_DosageBuilder {
    pub(crate) value: Value,
}

impl MedicationAdministration_DosageBuilder {
    pub fn build(&self) -> MedicationAdministration_Dosage {
        MedicationAdministration_Dosage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationAdministration_Dosage,
    ) -> MedicationAdministration_DosageBuilder {
        MedicationAdministration_DosageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationAdministration_DosageBuilder {
        let mut __value: Value = json!({});
        return MedicationAdministration_DosageBuilder { value: __value };
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn dose<'a>(&'a mut self, val: Quantity) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["dose"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rate_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["rateQuantity"] = json!(val.value);
        return self;
    }

    pub fn rate_ratio<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["rateRatio"] = json!(val.value);
        return self;
    }

    pub fn route<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["route"] = json!(val.value);
        return self;
    }

    pub fn site<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["site"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut MedicationAdministration_DosageBuilder {
        self.value["text"] = json!(val);
        return self;
    }
}
