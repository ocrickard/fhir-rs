#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_Handling<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SpecimenDefinition_Handling<'_> {
    pub fn new(value: &Value) -> SpecimenDefinition_Handling {
        SpecimenDefinition_Handling {
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

    /// Additional textual instructions for the preservation or transport of the
    /// specimen. For instance, 'Protect from light exposure'.
    pub fn instruction(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instruction") {
            return Some(string);
        }
        return None;
    }

    /// The maximum time interval of preservation of the specimen with these conditions.
    pub fn max_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("maxDuration") {
            return Some(Duration {
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

    /// It qualifies the interval of temperature, which characterizes an occurrence of
    /// handling. Conditions that are not related to temperature may be handled in the
    /// instruction element.
    pub fn temperature_qualifier(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("temperatureQualifier") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The temperature interval for this set of handling instructions.
    pub fn temperature_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("temperatureRange") {
            return Some(Range {
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
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.instruction() {}
        if let Some(_val) = self.max_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.temperature_qualifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.temperature_range() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SpecimenDefinition_HandlingBuilder {
    pub(crate) value: Value,
}

impl SpecimenDefinition_HandlingBuilder {
    pub fn build(&self) -> SpecimenDefinition_Handling {
        SpecimenDefinition_Handling {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SpecimenDefinition_Handling) -> SpecimenDefinition_HandlingBuilder {
        SpecimenDefinition_HandlingBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SpecimenDefinition_HandlingBuilder {
        let mut __value: Value = json!({});
        return SpecimenDefinition_HandlingBuilder { value: __value };
    }

    pub fn _instruction<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["_instruction"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn instruction<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["instruction"] = json!(val);
        return self;
    }

    pub fn max_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["maxDuration"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn temperature_qualifier<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["temperatureQualifier"] = json!(val.value);
        return self;
    }

    pub fn temperature_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut SpecimenDefinition_HandlingBuilder {
        self.value["temperatureRange"] = json!(val.value);
        return self;
    }
}
