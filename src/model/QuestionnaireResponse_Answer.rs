#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::QuestionnaireResponse_Item::QuestionnaireResponse_Item;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.

#[derive(Debug)]
pub struct QuestionnaireResponse_Answer<'a> {
    pub value: &'a Value,
}

impl QuestionnaireResponse_Answer<'_> {
    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUri") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueUri
    pub fn _value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDate") {
            return Some(string);
        }
        return None;
    }

    /// Nested groups and/or questions found within this particular answer.
    pub fn item(&self) -> Option<Vec<QuestionnaireResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| QuestionnaireResponse_Item { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element { value: val });
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

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for valueDecimal
    pub fn _value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("valueAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for valueDate
    pub fn _value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The answer (or one of the answers) provided by the respondent to the question.
    pub fn value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("valueCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_integer() {
            _val.validate();
        }
        if let Some(_val) = self.value_uri() {}
        if let Some(_val) = self._value_uri() {
            _val.validate();
        }
        if let Some(_val) = self.value_reference() {
            _val.validate();
        }
        if let Some(_val) = self.value_date() {}
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.value_attachment() {
            _val.validate();
        }
        if let Some(_val) = self._value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self._value_time() {
            _val.validate();
        }
        if let Some(_val) = self.value_time() {}
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self._value_string() {
            _val.validate();
        }
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self._value_date() {
            _val.validate();
        }
        if let Some(_val) = self.value_coding() {
            _val.validate();
        }
        return true;
    }
}
