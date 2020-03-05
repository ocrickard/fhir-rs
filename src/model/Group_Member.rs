#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Represents a defined collection of entities that may be discussed or acted upon
/// collectively but which are not expected to act collectively, and are not
/// formally or legally recognized; i.e. a collection of entities that isn't an
/// Organization.

#[derive(Debug)]
pub struct Group_Member<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Group_Member<'_> {
    pub fn new(value: &Value) -> Group_Member {
        Group_Member {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for inactive
    pub fn _inactive(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inactive") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to the entity that is a member of the group. Must be consistent with
    /// Group.type. If the entity is another group, then the type must be the same.
    pub fn entity(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["entity"]),
        }
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

    /// A flag to indicate that the member is no longer in the group, but previously may
    /// have been a member.
    pub fn inactive(&self) -> Option<bool> {
        if let Some(val) = self.value.get("inactive") {
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

    /// The period that the member was in the group, if known.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._inactive() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.entity().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.inactive() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Group_MemberBuilder {
    pub(crate) value: Value,
}

impl Group_MemberBuilder {
    pub fn build(&self) -> Group_Member {
        Group_Member {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Group_Member) -> Group_MemberBuilder {
        Group_MemberBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(entity: Reference) -> Group_MemberBuilder {
        let mut __value: Value = json!({});
        __value["entity"] = json!(entity.value);
        return Group_MemberBuilder { value: __value };
    }

    pub fn _inactive<'a>(&'a mut self, val: Element) -> &'a mut Group_MemberBuilder {
        self.value["_inactive"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Group_MemberBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Group_MemberBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn inactive<'a>(&'a mut self, val: bool) -> &'a mut Group_MemberBuilder {
        self.value["inactive"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Group_MemberBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut Group_MemberBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }
}
