#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A medicinal product in a container or package.

#[derive(Debug)]
pub struct MedicinalProductPackaged_BatchIdentifier<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductPackaged_BatchIdentifier<'_> {
    pub fn new(value: &Value) -> MedicinalProductPackaged_BatchIdentifier {
        MedicinalProductPackaged_BatchIdentifier {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// A number appearing on the immediate packaging (and not the outer packaging).
    pub fn immediate_packaging(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("immediatePackaging") {
            return Some(Identifier {
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

    /// A number appearing on the outer packaging of a specific batch.
    pub fn outer_packaging(&self) -> Identifier {
        Identifier {
            value: Cow::Borrowed(&self.value["outerPackaging"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.immediate_packaging() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.outer_packaging().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductPackaged_BatchIdentifierBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductPackaged_BatchIdentifierBuilder {
    pub fn build(&self) -> MedicinalProductPackaged_BatchIdentifier {
        MedicinalProductPackaged_BatchIdentifier {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductPackaged_BatchIdentifier,
    ) -> MedicinalProductPackaged_BatchIdentifierBuilder {
        MedicinalProductPackaged_BatchIdentifierBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(outer_packaging: Identifier) -> MedicinalProductPackaged_BatchIdentifierBuilder {
        let mut __value: Value = json!({});
        __value["outerPackaging"] = json!(outer_packaging.value);
        return MedicinalProductPackaged_BatchIdentifierBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPackaged_BatchIdentifierBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductPackaged_BatchIdentifierBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn immediate_packaging<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut MedicinalProductPackaged_BatchIdentifierBuilder {
        self.value["immediatePackaging"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductPackaged_BatchIdentifierBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
