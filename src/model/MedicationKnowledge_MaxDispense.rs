#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_MaxDispense<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_MaxDispense<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_MaxDispense {
        MedicationKnowledge_MaxDispense {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The period that applies to the maximum number of units.
    pub fn period(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("period") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The maximum number of units of the medication that can be dispensed.
    pub fn quantity(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["quantity"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.quantity().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_MaxDispenseBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_MaxDispenseBuilder {
    pub fn build(&self) -> MedicationKnowledge_MaxDispense {
        MedicationKnowledge_MaxDispense {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationKnowledge_MaxDispense,
    ) -> MedicationKnowledge_MaxDispenseBuilder {
        MedicationKnowledge_MaxDispenseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(quantity: Quantity) -> MedicationKnowledge_MaxDispenseBuilder {
        let mut __value: Value = json!({});
        __value["quantity"] = json!(quantity.value);
        return MedicationKnowledge_MaxDispenseBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_MaxDispenseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_MaxDispenseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_MaxDispenseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut MedicationKnowledge_MaxDispenseBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }
}
