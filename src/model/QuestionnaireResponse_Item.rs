#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::QuestionnaireResponse_Answer::QuestionnaireResponse_Answer;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of questions and their answers. The questions are ordered and
/// grouped into coherent subsets, corresponding to the structure of the grouping of
/// the questionnaire being responded to.

#[derive(Debug)]
pub struct QuestionnaireResponse_Item<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl QuestionnaireResponse_Item<'_> {
    pub fn new(value: &Value) -> QuestionnaireResponse_Item {
        QuestionnaireResponse_Item {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_linkId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The respondent's answer(s) to the question.
    pub fn answer(&self) -> Option<Vec<QuestionnaireResponse_Answer>> {
        if let Some(Value::Array(val)) = self.value.get("answer") {
            return Some(
                val.into_iter()
                    .map(|e| QuestionnaireResponse_Answer {
                        value: Cow::Borrowed(e),
                    })
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

    /// Questions or sub-groups nested beneath a question or group.
    pub fn item(&self) -> Option<Vec<QuestionnaireResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| QuestionnaireResponse_Item {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Text that is displayed above the contents of the group or as the text of the
    /// question being answered.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.link_id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {}
        return true;
    }
}

#[derive(Debug)]
pub struct QuestionnaireResponse_ItemBuilder {
    pub(crate) value: Value,
}

impl QuestionnaireResponse_ItemBuilder {
    pub fn build(&self) -> QuestionnaireResponse_Item {
        QuestionnaireResponse_Item {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: QuestionnaireResponse_Item) -> QuestionnaireResponse_ItemBuilder {
        QuestionnaireResponse_ItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> QuestionnaireResponse_ItemBuilder {
        let mut __value: Value = json!({});
        return QuestionnaireResponse_ItemBuilder { value: __value };
    }

    pub fn _definition<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["_definition"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Element) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["_linkId"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn answer<'a>(
        &'a mut self,
        val: Vec<QuestionnaireResponse_Answer>,
    ) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["answer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn definition<'a>(&'a mut self, val: &str) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["definition"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn item<'a>(
        &'a mut self,
        val: Vec<QuestionnaireResponse_Item>,
    ) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: &str) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut QuestionnaireResponse_ItemBuilder {
        self.value["text"] = json!(val);
        return self;
    }
}
