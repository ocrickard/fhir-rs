#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Dosage_DoseAndRate::Dosage_DoseAndRate;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Ratio::Ratio;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// Indicates how the medication is/was taken or should be taken by the patient.

#[derive(Debug)]
pub struct Dosage<'a> {
    pub value: &'a Value,
}

impl Dosage<'_> {
    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Technique for administering medication.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept { value: val });
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

    /// Supplemental instructions to the patient on how to take the medication  (e.g.
    /// "with meals" or"take half to one hour before food") or warnings for the patient
    /// about the medication (e.g. "may cause drowsiness" or "avoid exposure of skin to
    /// direct sunlight or sunlamps").
    pub fn additional_instruction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("additionalInstruction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Body site to administer to.
    pub fn site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("site") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// How drug should enter body.
    pub fn route(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("route") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for asNeededBoolean
    pub fn _as_needed_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_asNeededBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The amount of medication administered.
    pub fn dose_and_rate(&self) -> Option<Vec<Dosage_DoseAndRate>> {
        if let Some(Value::Array(val)) = self.value.get("doseAndRate") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage_DoseAndRate { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Free text dosage instructions e.g. SIG.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Instructions in terms that are understood by the patient or consumer.
    pub fn patient_instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patientInstruction") {
            return Some(string);
        }
        return None;
    }

    /// Indicates whether the Medication is only taken when needed within a specific
    /// dosing schedule (Boolean option), or it indicates the precondition for taking
    /// the Medication (CodeableConcept).
    pub fn as_needed_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("asNeededBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates whether the Medication is only taken when needed within a specific
    /// dosing schedule (Boolean option), or it indicates the precondition for taking
    /// the Medication (CodeableConcept).
    pub fn as_needed_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("asNeededCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for patientInstruction
    pub fn _patient_instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patientInstruction") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Upper limit on medication per unit of time.
    pub fn max_dose_per_period(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("maxDosePerPeriod") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// When medication should be administered.
    pub fn timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timing") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// Upper limit on medication per lifetime of the patient.
    pub fn max_dose_per_lifetime(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxDosePerLifetime") {
            return Some(Quantity { value: val });
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

    /// Indicates the order in which the dosage instructions should be applied or
    /// interpreted.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Upper limit on medication per administration.
    pub fn max_dose_per_administration(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxDosePerAdministration") {
            return Some(Quantity { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._text() {
            _val.validate();
        }
        if let Some(_val) = self.method() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.additional_instruction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.site() {
            _val.validate();
        }
        if let Some(_val) = self.route() {
            _val.validate();
        }
        if let Some(_val) = self._sequence() {
            _val.validate();
        }
        if let Some(_val) = self._as_needed_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.dose_and_rate() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.patient_instruction() {}
        if let Some(_val) = self.as_needed_boolean() {}
        if let Some(_val) = self.as_needed_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._patient_instruction() {
            _val.validate();
        }
        if let Some(_val) = self.max_dose_per_period() {
            _val.validate();
        }
        if let Some(_val) = self.timing() {
            _val.validate();
        }
        if let Some(_val) = self.max_dose_per_lifetime() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.max_dose_per_administration() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}
