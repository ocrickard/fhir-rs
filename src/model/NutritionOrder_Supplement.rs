#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A request to supply a diet, formula feeding (enteral) or oral nutritional
/// supplement to a patient/resident.

#[derive(Debug)]
pub struct NutritionOrder_Supplement<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NutritionOrder_Supplement<'_> {
    pub fn new(value: &Value) -> NutritionOrder_Supplement {
        NutritionOrder_Supplement {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for instruction
    pub fn _instruction(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instruction") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for productName
    pub fn _product_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_productName") {
            return Some(Element {
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

    /// Free text or additional instructions or information pertaining to the oral
    /// supplement.
    pub fn instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instruction") {
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

    /// The product or brand name of the nutritional supplement such as "Acme Protein
    /// Shake".
    pub fn product_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("productName") {
            return Some(string);
        }
        return None;
    }

    /// The amount of the nutritional supplement to be given.
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time period and frequency at which the supplement(s) should be given.  The
    /// supplement should be given for the combination of all schedules if more than one
    /// schedule is present.
    pub fn schedule(&self) -> Option<Vec<Timing>> {
        if let Some(Value::Array(val)) = self.value.get("schedule") {
            return Some(
                val.into_iter()
                    .map(|e| Timing {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The kind of nutritional supplement product required such as a high protein or
    /// pediatric clear liquid supplement.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._instruction() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._product_name() {
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
        if let Some(_val) = self.instruction() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.product_name() {}
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.schedule() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct NutritionOrder_SupplementBuilder {
    pub(crate) value: Value,
}

impl NutritionOrder_SupplementBuilder {
    pub fn build(&self) -> NutritionOrder_Supplement {
        NutritionOrder_Supplement {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NutritionOrder_Supplement) -> NutritionOrder_SupplementBuilder {
        NutritionOrder_SupplementBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NutritionOrder_SupplementBuilder {
        let mut __value: Value = json!({});
        return NutritionOrder_SupplementBuilder { value: __value };
    }

    pub fn _instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["_instruction"] = json!(val.value);
        return self;
    }

    pub fn _product_name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["_productName"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn instruction<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["instruction"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn product_name<'a>(&'a mut self, val: &str) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["productName"] = json!(val);
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn schedule<'a>(
        &'a mut self,
        val: Vec<Timing>,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["schedule"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut NutritionOrder_SupplementBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
