#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.

#[derive(Debug)]
pub struct Questionnaire_EnableWhen<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Questionnaire_EnableWhen<'_> {
    pub fn new(value: &Value) -> Questionnaire_EnableWhen {
        Questionnaire_EnableWhen {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for answerBoolean
    pub fn _answer_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerDate
    pub fn _answer_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerDateTime
    pub fn _answer_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerDecimal
    pub fn _answer_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerDecimal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerInteger
    pub fn _answer_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerString
    pub fn _answer_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for answerTime
    pub fn _answer_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_answerTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for operator
    pub fn _operator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_operator") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for question
    pub fn _question(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_question") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("answerBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("answerCoding") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("answerDate") {
            return Some(string);
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("answerDateTime") {
            return Some(string);
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("answerDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("answerInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("answerQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("answerReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("answerString") {
            return Some(string);
        }
        return None;
    }

    /// A value that the referenced question is tested using the specified operator in
    /// order for the item to be enabled.
    pub fn answer_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("answerTime") {
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

    /// Specifies the criteria by which the question is enabled.
    pub fn operator(&self) -> Option<Questionnaire_EnableWhenOperator> {
        if let Some(Value::String(val)) = self.value.get("operator") {
            return Some(Questionnaire_EnableWhenOperator::from_string(&val).unwrap());
        }
        return None;
    }

    /// The linkId for the question whose answer (or lack of answer) governs whether
    /// this item is enabled.
    pub fn question(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("question") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._answer_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_decimal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._answer_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._operator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._question() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer_boolean() {}
        if let Some(_val) = self.answer_coding() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer_date() {}
        if let Some(_val) = self.answer_date_time() {}
        if let Some(_val) = self.answer_decimal() {}
        if let Some(_val) = self.answer_integer() {}
        if let Some(_val) = self.answer_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer_string() {}
        if let Some(_val) = self.answer_time() {}
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
        if let Some(_val) = self.operator() {}
        if let Some(_val) = self.question() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Questionnaire_EnableWhenBuilder {
    pub(crate) value: Value,
}

impl Questionnaire_EnableWhenBuilder {
    pub fn build(&self) -> Questionnaire_EnableWhen {
        Questionnaire_EnableWhen {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Questionnaire_EnableWhen) -> Questionnaire_EnableWhenBuilder {
        Questionnaire_EnableWhenBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Questionnaire_EnableWhenBuilder {
        let mut __value: Value = json!({});
        return Questionnaire_EnableWhenBuilder { value: __value };
    }

    pub fn _answer_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerBoolean"] = json!(val.value);
        return self;
    }

    pub fn _answer_date<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerDate"] = json!(val.value);
        return self;
    }

    pub fn _answer_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerDateTime"] = json!(val.value);
        return self;
    }

    pub fn _answer_decimal<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerDecimal"] = json!(val.value);
        return self;
    }

    pub fn _answer_integer<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerInteger"] = json!(val.value);
        return self;
    }

    pub fn _answer_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerString"] = json!(val.value);
        return self;
    }

    pub fn _answer_time<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_answerTime"] = json!(val.value);
        return self;
    }

    pub fn _operator<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_operator"] = json!(val.value);
        return self;
    }

    pub fn _question<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["_question"] = json!(val.value);
        return self;
    }

    pub fn answer_boolean<'a>(&'a mut self, val: bool) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerBoolean"] = json!(val);
        return self;
    }

    pub fn answer_coding<'a>(&'a mut self, val: Coding) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerCoding"] = json!(val.value);
        return self;
    }

    pub fn answer_date<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerDate"] = json!(val);
        return self;
    }

    pub fn answer_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerDateTime"] = json!(val);
        return self;
    }

    pub fn answer_decimal<'a>(&'a mut self, val: f64) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerDecimal"] = json!(val);
        return self;
    }

    pub fn answer_integer<'a>(&'a mut self, val: f64) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerInteger"] = json!(val);
        return self;
    }

    pub fn answer_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerQuantity"] = json!(val.value);
        return self;
    }

    pub fn answer_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerReference"] = json!(val.value);
        return self;
    }

    pub fn answer_string<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerString"] = json!(val);
        return self;
    }

    pub fn answer_time<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["answerTime"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operator<'a>(
        &'a mut self,
        val: Questionnaire_EnableWhenOperator,
    ) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["operator"] = json!(val.to_string());
        return self;
    }

    pub fn question<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_EnableWhenBuilder {
        self.value["question"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum Questionnaire_EnableWhenOperator {
    Exists,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Questionnaire_EnableWhenOperator {
    pub fn from_string(string: &str) -> Option<Questionnaire_EnableWhenOperator> {
        match string {
            "exists" => Some(Questionnaire_EnableWhenOperator::Exists),
            "=" => Some(Questionnaire_EnableWhenOperator::Equal),
            "!=" => Some(Questionnaire_EnableWhenOperator::NotEqual),
            ">" => Some(Questionnaire_EnableWhenOperator::GreaterThan),
            "<" => Some(Questionnaire_EnableWhenOperator::LessThan),
            ">=" => Some(Questionnaire_EnableWhenOperator::GreaterThanOrEqual),
            "<=" => Some(Questionnaire_EnableWhenOperator::LessThanOrEqual),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Questionnaire_EnableWhenOperator::Exists => "exists".to_string(),
            Questionnaire_EnableWhenOperator::Equal => "=".to_string(),
            Questionnaire_EnableWhenOperator::NotEqual => "!=".to_string(),
            Questionnaire_EnableWhenOperator::GreaterThan => ">".to_string(),
            Questionnaire_EnableWhenOperator::LessThan => "<".to_string(),
            Questionnaire_EnableWhenOperator::GreaterThanOrEqual => ">=".to_string(),
            Questionnaire_EnableWhenOperator::LessThanOrEqual => "<=".to_string(),
        }
    }
}
