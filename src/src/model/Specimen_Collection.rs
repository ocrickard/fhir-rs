#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen_Collection<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Specimen_Collection<'_> {
    pub fn new(value: &Value) -> Specimen_Collection {
        Specimen_Collection {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for collectedDateTime
    pub fn _collected_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_collectedDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Anatomical location from which the specimen was collected (if subject is a
    /// patient). This is the target site.  This element is not used for environmental
    /// specimens.
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Time when specimen was collected from subject - the physiologically relevant
    /// time.
    pub fn collected_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("collectedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Person who collected the specimen.
    pub fn collector(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("collector") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The span of time over which the collection of a specimen occurred.
    pub fn duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("duration") {
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

    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fn fasting_status_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fastingStatusCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Abstinence or reduction from some or all food, drink, or both, for a period of
    /// time prior to sample collection.
    pub fn fasting_status_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("fastingStatusDuration") {
            return Some(Duration {
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

    /// A coded value specifying the technique that is used to perform the procedure.
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

    /// The quantity of specimen collected; for instance the volume of a blood sample,
    /// or the physical measurement of an anatomic pathology sample.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._collected_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collected_date_time() {}
        if let Some(_val) = self.collected_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collector() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fasting_status_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fasting_status_duration() {
            if !_val.validate() {
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
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Specimen_CollectionBuilder {
    pub(crate) value: Value,
}

impl Specimen_CollectionBuilder {
    pub fn build(&self) -> Specimen_Collection {
        Specimen_Collection {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Specimen_Collection) -> Specimen_CollectionBuilder {
        Specimen_CollectionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Specimen_CollectionBuilder {
        let mut __value: Value = json!({});
        return Specimen_CollectionBuilder { value: __value };
    }

    pub fn _collected_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Specimen_CollectionBuilder {
        self.value["_collectedDateTime"] = json!(val.value);
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Specimen_CollectionBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn collected_date_time<'a>(&'a mut self, val: &str) -> &'a mut Specimen_CollectionBuilder {
        self.value["collectedDateTime"] = json!(val);
        return self;
    }

    pub fn collected_period<'a>(&'a mut self, val: Period) -> &'a mut Specimen_CollectionBuilder {
        self.value["collectedPeriod"] = json!(val.value);
        return self;
    }

    pub fn collector<'a>(&'a mut self, val: Reference) -> &'a mut Specimen_CollectionBuilder {
        self.value["collector"] = json!(val.value);
        return self;
    }

    pub fn duration<'a>(&'a mut self, val: Duration) -> &'a mut Specimen_CollectionBuilder {
        self.value["duration"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Specimen_CollectionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fasting_status_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Specimen_CollectionBuilder {
        self.value["fastingStatusCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn fasting_status_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut Specimen_CollectionBuilder {
        self.value["fastingStatusDuration"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Specimen_CollectionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn method<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Specimen_CollectionBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Specimen_CollectionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Specimen_CollectionBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }
}
