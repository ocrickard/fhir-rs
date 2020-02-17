#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstanceAmount::SubstanceAmount;
use crate::model::SubstancePolymer_DegreeOfPolymerisation::SubstancePolymer_DegreeOfPolymerisation;
use crate::model::SubstancePolymer_StructuralRepresentation::SubstancePolymer_StructuralRepresentation;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Todo.

#[derive(Debug)]
pub struct SubstancePolymer_RepeatUnit<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstancePolymer_RepeatUnit<'_> {
    pub fn new(value: &Value) -> SubstancePolymer_RepeatUnit {
        SubstancePolymer_RepeatUnit {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for repeatUnit
    pub fn _repeat_unit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_repeatUnit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn amount(&self) -> Option<SubstanceAmount> {
        if let Some(val) = self.value.get("amount") {
            return Some(SubstanceAmount {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn degree_of_polymerisation(&self) -> Option<Vec<SubstancePolymer_DegreeOfPolymerisation>> {
        if let Some(Value::Array(val)) = self.value.get("degreeOfPolymerisation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_DegreeOfPolymerisation {
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
    pub fn orientation_of_polymerisation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("orientationOfPolymerisation") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn repeat_unit(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("repeatUnit") {
            return Some(string);
        }
        return None;
    }

    /// Todo.
    pub fn structural_representation(
        &self,
    ) -> Option<Vec<SubstancePolymer_StructuralRepresentation>> {
        if let Some(Value::Array(val)) = self.value.get("structuralRepresentation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstancePolymer_StructuralRepresentation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._repeat_unit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.degree_of_polymerisation() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.orientation_of_polymerisation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.repeat_unit() {}
        if let Some(_val) = self.structural_representation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstancePolymer_RepeatUnitBuilder {
    pub(crate) value: Value,
}

impl SubstancePolymer_RepeatUnitBuilder {
    pub fn build(&self) -> SubstancePolymer_RepeatUnit {
        SubstancePolymer_RepeatUnit {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstancePolymer_RepeatUnit) -> SubstancePolymer_RepeatUnitBuilder {
        SubstancePolymer_RepeatUnitBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstancePolymer_RepeatUnitBuilder {
        let mut __value: Value = json!({});
        return SubstancePolymer_RepeatUnitBuilder { value: __value };
    }

    pub fn _repeat_unit<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["_repeatUnit"] = json!(val.value);
        return self;
    }

    pub fn amount<'a>(
        &'a mut self,
        val: SubstanceAmount,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["amount"] = json!(val.value);
        return self;
    }

    pub fn degree_of_polymerisation<'a>(
        &'a mut self,
        val: Vec<SubstancePolymer_DegreeOfPolymerisation>,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["degreeOfPolymerisation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn orientation_of_polymerisation<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["orientationOfPolymerisation"] = json!(val.value);
        return self;
    }

    pub fn repeat_unit<'a>(&'a mut self, val: &str) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["repeatUnit"] = json!(val);
        return self;
    }

    pub fn structural_representation<'a>(
        &'a mut self,
        val: Vec<SubstancePolymer_StructuralRepresentation>,
    ) -> &'a mut SubstancePolymer_RepeatUnitBuilder {
        self.value["structuralRepresentation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
