#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Ratio::Ratio;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct MedicinalProductIngredient_ReferenceStrength<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductIngredient_ReferenceStrength<'_> {
    pub fn new(value: &Value) -> MedicinalProductIngredient_ReferenceStrength {
        MedicinalProductIngredient_ReferenceStrength {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for measurementPoint
    pub fn _measurement_point(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_measurementPoint") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The country or countries for which the strength range applies.
    pub fn country(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("country") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
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

    /// For when strength is measured at a particular point or distance.
    pub fn measurement_point(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("measurementPoint") {
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

    /// Strength expressed in terms of a reference substance.
    pub fn strength(&self) -> Ratio {
        Ratio {
            value: Cow::Borrowed(&self.value["strength"]),
        }
    }

    /// Strength expressed in terms of a reference substance.
    pub fn strength_low_limit(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("strengthLowLimit") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Relevant reference substance.
    pub fn substance(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substance") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._measurement_point() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.country() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.measurement_point() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.strength().validate() {
            return false;
        }
        if let Some(_val) = self.strength_low_limit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.substance() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductIngredient_ReferenceStrengthBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductIngredient_ReferenceStrengthBuilder {
    pub fn build(&self) -> MedicinalProductIngredient_ReferenceStrength {
        MedicinalProductIngredient_ReferenceStrength {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductIngredient_ReferenceStrength,
    ) -> MedicinalProductIngredient_ReferenceStrengthBuilder {
        MedicinalProductIngredient_ReferenceStrengthBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(strength: Ratio) -> MedicinalProductIngredient_ReferenceStrengthBuilder {
        let mut __value: Value = json!({});
        __value["strength"] = json!(strength.value);
        return MedicinalProductIngredient_ReferenceStrengthBuilder { value: __value };
    }

    pub fn _measurement_point<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["_measurementPoint"] = json!(val.value);
        return self;
    }

    pub fn country<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["country"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn measurement_point<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["measurementPoint"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn strength_low_limit<'a>(
        &'a mut self,
        val: Ratio,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["strengthLowLimit"] = json!(val.value);
        return self;
    }

    pub fn substance<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductIngredient_ReferenceStrengthBuilder {
        self.value["substance"] = json!(val.value);
        return self;
    }
}
