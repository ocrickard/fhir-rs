#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A list is a curated collection of resources.

#[derive(Debug)]
pub struct List_Entry<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl List_Entry<'_> {
    pub fn new(value: &Value) -> List_Entry {
        List_Entry {
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

    /// Extensions for deleted
    pub fn _deleted(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deleted") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When this item was added to the list.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// True if this item is marked as deleted in the list.
    pub fn deleted(&self) -> Option<bool> {
        if let Some(val) = self.value.get("deleted") {
            return Some(val.as_bool().unwrap());
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

    /// The flag allows the system constructing the list to indicate the role and
    /// significance of the item in the list.
    pub fn flag(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("flag") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A reference to the actual resource from which data was derived.
    pub fn item(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["item"]),
        }
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._deleted() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.deleted() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.flag() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self.item().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct List_EntryBuilder {
    pub(crate) value: Value,
}

impl List_EntryBuilder {
    pub fn build(&self) -> List_Entry {
        List_Entry {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: List_Entry) -> List_EntryBuilder {
        List_EntryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(item: Reference) -> List_EntryBuilder {
        let mut __value: Value = json!({});
        __value["item"] = json!(item.value);
        return List_EntryBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut List_EntryBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _deleted<'a>(&'a mut self, val: Element) -> &'a mut List_EntryBuilder {
        self.value["_deleted"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut List_EntryBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn deleted<'a>(&'a mut self, val: bool) -> &'a mut List_EntryBuilder {
        self.value["deleted"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut List_EntryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn flag<'a>(&'a mut self, val: CodeableConcept) -> &'a mut List_EntryBuilder {
        self.value["flag"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut List_EntryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut List_EntryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
