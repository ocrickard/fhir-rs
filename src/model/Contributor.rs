#![allow(unused_imports, non_camel_case_types)]

use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A contributor to the content of a knowledge asset, including authors, editors,
/// reviewers, and endorsers.

#[derive(Debug)]
pub struct Contributor<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contributor<'_> {
    pub fn new(value: &Value) -> Contributor {
        Contributor {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Contact details to assist a user in finding and communicating with the
    /// contributor.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
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

    /// The name of the individual or organization responsible for the contribution.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The type of contributor.
    pub fn fhir_type(&self) -> Option<ContributorType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(ContributorType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contact() {
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ContributorBuilder {
    pub(crate) value: Value,
}

impl ContributorBuilder {
    pub fn build(&self) -> Contributor {
        Contributor {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contributor) -> ContributorBuilder {
        ContributorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ContributorBuilder {
        let mut __value: Value = json!({});
        return ContributorBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ContributorBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ContributorBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactDetail>) -> &'a mut ContributorBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ContributorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ContributorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ContributorBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: ContributorType) -> &'a mut ContributorBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum ContributorType {
    Author,
    Editor,
    Reviewer,
    Endorser,
}

impl ContributorType {
    pub fn from_string(string: &str) -> Option<ContributorType> {
        match string {
            "author" => Some(ContributorType::Author),
            "editor" => Some(ContributorType::Editor),
            "reviewer" => Some(ContributorType::Reviewer),
            "endorser" => Some(ContributorType::Endorser),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContributorType::Author => "author".to_string(),
            ContributorType::Editor => "editor".to_string(),
            ContributorType::Reviewer => "reviewer".to_string(),
            ContributorType::Endorser => "endorser".to_string(),
        }
    }
}
