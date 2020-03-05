#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Official<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Official<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Official {
        SubstanceSpecification_Official {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Which authority uses this official name.
    pub fn authority(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("authority") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Date of official name change.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    /// The status of the official name.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authority() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
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
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_OfficialBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_OfficialBuilder {
    pub fn build(&self) -> SubstanceSpecification_Official {
        SubstanceSpecification_Official {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceSpecification_Official,
    ) -> SubstanceSpecification_OfficialBuilder {
        SubstanceSpecification_OfficialBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_OfficialBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_OfficialBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn authority<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["authority"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_OfficialBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }
}
