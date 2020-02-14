#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::QuestionnaireResponse_Answer::QuestionnaireResponse_Answer;
use serde_json::value::Value;

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.

#[derive(Debug)]
pub struct QuestionnaireResponse_Item<'a> {
    pub value: &'a Value,
}

impl QuestionnaireResponse_Item<'_> {
    /// Text that is displayed above the contents of the group or as the text of the
    /// question being answered.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Questions or sub-groups nested beneath a question or group.
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

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element { value: val });
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

    /// The respondent's answer(s) to the question.
    pub fn answer(&self) -> Option<Vec<QuestionnaireResponse_Answer>> {
        if let Some(Value::Array(val)) = self.value.get("answer") {
            return Some(
                val.into_iter()
                    .map(|e| QuestionnaireResponse_Answer { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to an [[[ElementDefinition]]] that provides the details for the
    /// item.
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// The item from the Questionnaire that corresponds to this item in the
    /// QuestionnaireResponse resource.
    pub fn link_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("linkId") {
            return Some(string);
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

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_linkId") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._text() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.answer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.link_id() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._definition() {
            _val.validate();
        }
        if let Some(_val) = self._link_id() {
            _val.validate();
        }
        return true;
    }
}
