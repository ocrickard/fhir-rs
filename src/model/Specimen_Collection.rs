#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen_Collection<'a> {
    pub value: &'a Value,
}

impl Specimen_Collection<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Person who collected the specimen.
    pub fn collector(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("collector") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Time when specimen was collected from subject - the physiologically relevant
    /// time.
    pub fn collected_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("collectedPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The quantity of specimen collected; for instance the volume of a blood sample,
    /// or the physical measurement of an anatomic pathology sample.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// A coded value specifying the technique that is used to perform the procedure.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fn fasting_status_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fastingStatusCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fn fasting_status_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("fastingStatusDuration") {
            return Some(Duration { value: val });
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

    /// The span of time over which the collection of a specimen occurred.
    pub fn duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("duration") {
            return Some(Duration { value: val });
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

    /// Anatomical location from which the specimen was collected (if subject is a
    /// patient). This is the target site.  This element is not used for environmental
    /// specimens.
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Time when specimen was collected from subject - the physiologically relevant
    /// time.
    pub fn collected_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("collectedDateTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.collector() {
            _val.validate();
        }
        if let Some(_val) = self.collected_period() {
            _val.validate();
        }
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.method() {
            _val.validate();
        }
        if let Some(_val) = self.fasting_status_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.fasting_status_duration() {
            _val.validate();
        }
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
        if let Some(_val) = self.duration() {
            _val.validate();
        }
        if let Some(_val) = self._collected_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.body_site() {
            _val.validate();
        }
        if let Some(_val) = self.collected_date_time() {}
        return true;
    }
}
