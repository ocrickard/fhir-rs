#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstancePolymer_RepeatUnit::SubstancePolymer_RepeatUnit;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Todo.

#[derive(Debug)]
pub struct SubstancePolymer_Repeat<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstancePolymer_Repeat<'_> {
    pub fn new(value: &Value) -> SubstancePolymer_Repeat {
        SubstancePolymer_Repeat {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for averageMolecularFormula
    pub fn _average_molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_averageMolecularFormula") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfUnits
    pub fn _number_of_units(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfUnits") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn average_molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("averageMolecularFormula") {
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

    /// Todo.
    pub fn number_of_units(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfUnits") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Todo.
    pub fn repeat_unit(&self) -> Option<Vec<SubstancePolymer_RepeatUnit>> {
        if let Some(Value::Array(val)) = self.value.get("repeatUnit") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_RepeatUnit {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Todo.
    pub fn repeat_unit_amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("repeatUnitAmountType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._average_molecular_formula() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_units() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.average_molecular_formula() {}
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
        if let Some(_val) = self.number_of_units() {}
        if let Some(_val) = self.repeat_unit() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.repeat_unit_amount_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstancePolymer_RepeatBuilder {
    pub(crate) value: Value,
}

impl SubstancePolymer_RepeatBuilder {
    pub fn build(&self) -> SubstancePolymer_Repeat {
        SubstancePolymer_Repeat {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstancePolymer_Repeat) -> SubstancePolymer_RepeatBuilder {
        SubstancePolymer_RepeatBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstancePolymer_RepeatBuilder {
        let mut __value: Value = json!({});
        return SubstancePolymer_RepeatBuilder { value: __value };
    }

    pub fn _average_molecular_formula<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["_averageMolecularFormula"] = json!(val.value);
        return self;
    }

    pub fn _number_of_units<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["_numberOfUnits"] = json!(val.value);
        return self;
    }

    pub fn average_molecular_formula<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["averageMolecularFormula"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_of_units<'a>(&'a mut self, val: i64) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["numberOfUnits"] = json!(val);
        return self;
    }

    pub fn repeat_unit<'a>(
        &'a mut self,
        val: Vec<SubstancePolymer_RepeatUnit>,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["repeatUnit"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn repeat_unit_amount_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_RepeatBuilder {
        self.value["repeatUnitAmountType"] = json!(val.value);
        return self;
    }
}
