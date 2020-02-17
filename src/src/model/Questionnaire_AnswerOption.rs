#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.

#[derive(Debug)]
pub struct Questionnaire_AnswerOption<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Questionnaire_AnswerOption<'_> {
    pub fn new(value: &Value) -> Questionnaire_AnswerOption {
        Questionnaire_AnswerOption {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for initialSelected
    pub fn _initial_selected(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initialSelected") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDate
    pub fn _value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
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

    /// Indicates whether the answer value is selected when the list of possible answers
    /// is initially shown.
    pub fn initial_selected(&self) -> Option<bool> {
        if let Some(val) = self.value.get("initialSelected") {
            return Some(val.as_bool().unwrap());
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

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("valueCoding") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDate") {
            return Some(string);
        }
        return None;
    }

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// A potential answer that's allowed as the answer to this question.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._initial_selected() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_time() {
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
        if let Some(_val) = self.initial_selected() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.value_coding() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_date() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_time() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Questionnaire_AnswerOptionBuilder {
    pub(crate) value: Value,
}

impl Questionnaire_AnswerOptionBuilder {
    pub fn build(&self) -> Questionnaire_AnswerOption {
        Questionnaire_AnswerOption {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Questionnaire_AnswerOption) -> Questionnaire_AnswerOptionBuilder {
        Questionnaire_AnswerOptionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Questionnaire_AnswerOptionBuilder {
        let mut __value: Value = json!({});
        return Questionnaire_AnswerOptionBuilder { value: __value };
    }

    pub fn _initial_selected<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["_initialSelected"] = json!(val.value);
        return self;
    }

    pub fn _value_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["_valueDate"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn _value_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["_valueTime"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn initial_selected<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["initialSelected"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_coding<'a>(
        &'a mut self,
        val: Coding,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueCoding"] = json!(val.value);
        return self;
    }

    pub fn value_date<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueDate"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueReference"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }

    pub fn value_time<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_AnswerOptionBuilder {
        self.value["valueTime"] = json!(val);
        return self;
    }
}
