#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition_QuantitativeDetails<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ObservationDefinition_QuantitativeDetails<'_> {
    pub fn new(value: &Value) -> ObservationDefinition_QuantitativeDetails {
        ObservationDefinition_QuantitativeDetails {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for conversionFactor
    pub fn _conversion_factor(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conversionFactor") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for decimalPrecision
    pub fn _decimal_precision(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_decimalPrecision") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Factor for converting value expressed with SI unit to value expressed with
    /// customary unit.
    pub fn conversion_factor(&self) -> Option<f64> {
        if let Some(val) = self.value.get("conversionFactor") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Customary unit used to report quantitative results of observations conforming to
    /// this ObservationDefinition.
    pub fn customary_unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("customaryUnit") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Number of digits after decimal separator when the results of such observations
    /// are of type Quantity.
    pub fn decimal_precision(&self) -> Option<i64> {
        if let Some(val) = self.value.get("decimalPrecision") {
            return Some(val.as_i64().unwrap());
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

    /// SI unit used to report quantitative results of observations conforming to this
    /// ObservationDefinition.
    pub fn unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unit") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._conversion_factor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._decimal_precision() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.conversion_factor() {}
        if let Some(_val) = self.customary_unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.decimal_precision() {}
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
        if let Some(_val) = self.unit() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ObservationDefinition_QuantitativeDetailsBuilder {
    pub(crate) value: Value,
}

impl ObservationDefinition_QuantitativeDetailsBuilder {
    pub fn build(&self) -> ObservationDefinition_QuantitativeDetails {
        ObservationDefinition_QuantitativeDetails {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ObservationDefinition_QuantitativeDetails,
    ) -> ObservationDefinition_QuantitativeDetailsBuilder {
        ObservationDefinition_QuantitativeDetailsBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ObservationDefinition_QuantitativeDetailsBuilder {
        let mut __value: Value = json!({});
        return ObservationDefinition_QuantitativeDetailsBuilder { value: __value };
    }

    pub fn _conversion_factor<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["_conversionFactor"] = json!(val.value);
        return self;
    }

    pub fn _decimal_precision<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["_decimalPrecision"] = json!(val.value);
        return self;
    }

    pub fn conversion_factor<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["conversionFactor"] = json!(val);
        return self;
    }

    pub fn customary_unit<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["customaryUnit"] = json!(val.value);
        return self;
    }

    pub fn decimal_precision<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["decimalPrecision"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn unit<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinition_QuantitativeDetailsBuilder {
        self.value["unit"] = json!(val.value);
        return self;
    }
}
