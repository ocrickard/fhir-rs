#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_Processing<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl BiologicallyDerivedProduct_Processing<'_> {
    pub fn new(value: &Value) -> BiologicallyDerivedProduct_Processing {
        BiologicallyDerivedProduct_Processing {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for timeDateTime
    pub fn _time_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timeDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Substance added during processing.
    pub fn additive(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("additive") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Description of of processing.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// Procesing code.
    pub fn procedure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("procedure") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Time of processing.
    pub fn time_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timeDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Time of processing.
    pub fn time_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._time_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additive() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.procedure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.time_date_time() {}
        if let Some(_val) = self.time_period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_ProcessingBuilder {
    pub(crate) value: Value,
}

impl BiologicallyDerivedProduct_ProcessingBuilder {
    pub fn build(&self) -> BiologicallyDerivedProduct_Processing {
        BiologicallyDerivedProduct_Processing {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: BiologicallyDerivedProduct_Processing,
    ) -> BiologicallyDerivedProduct_ProcessingBuilder {
        BiologicallyDerivedProduct_ProcessingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BiologicallyDerivedProduct_ProcessingBuilder {
        let mut __value: Value = json!({});
        return BiologicallyDerivedProduct_ProcessingBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _time_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["_timeDateTime"] = json!(val.value);
        return self;
    }

    pub fn additive<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["additive"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn procedure<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["procedure"] = json!(val.value);
        return self;
    }

    pub fn time_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["timeDateTime"] = json!(val);
        return self;
    }

    pub fn time_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut BiologicallyDerivedProduct_ProcessingBuilder {
        self.value["timePeriod"] = json!(val.value);
        return self;
    }
}
