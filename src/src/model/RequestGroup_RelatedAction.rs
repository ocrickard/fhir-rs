#![allow(unused_imports, non_camel_case_types)]

use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A group of related requests that can be used to capture intended activities that
/// have inter-dependencies such as "give this medication after that one".

#[derive(Debug)]
pub struct RequestGroup_RelatedAction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl RequestGroup_RelatedAction<'_> {
    pub fn new(value: &Value) -> RequestGroup_RelatedAction {
        RequestGroup_RelatedAction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actionId
    pub fn _action_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actionId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for relationship
    pub fn _relationship(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relationship") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The element id of the action this is related to.
    pub fn action_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actionId") {
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

    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub fn offset_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("offsetDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A duration or range of durations to apply to the relationship. For example, 30-
    /// 60 minutes before.
    pub fn offset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("offsetRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The relationship of this action to the related action.
    pub fn relationship(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("relationship") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._action_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._relationship() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.action_id() {}
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
        if let Some(_val) = self.offset_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.offset_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {}
        return true;
    }
}

#[derive(Debug)]
pub struct RequestGroup_RelatedActionBuilder {
    pub(crate) value: Value,
}

impl RequestGroup_RelatedActionBuilder {
    pub fn build(&self) -> RequestGroup_RelatedAction {
        RequestGroup_RelatedAction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: RequestGroup_RelatedAction) -> RequestGroup_RelatedActionBuilder {
        RequestGroup_RelatedActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> RequestGroup_RelatedActionBuilder {
        let mut __value: Value = json!({});
        return RequestGroup_RelatedActionBuilder { value: __value };
    }

    pub fn _action_id<'a>(&'a mut self, val: Element) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["_actionId"] = json!(val.value);
        return self;
    }

    pub fn _relationship<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["_relationship"] = json!(val.value);
        return self;
    }

    pub fn action_id<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["actionId"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn offset_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["offsetDuration"] = json!(val.value);
        return self;
    }

    pub fn offset_range<'a>(&'a mut self, val: Range) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["offsetRange"] = json!(val.value);
        return self;
    }

    pub fn relationship<'a>(&'a mut self, val: &str) -> &'a mut RequestGroup_RelatedActionBuilder {
        self.value["relationship"] = json!(val);
        return self;
    }
}
