#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Todo.

#[derive(Debug)]
pub struct SubstanceReferenceInformation_Classification<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceReferenceInformation_Classification<'_> {
    pub fn new(value: &Value) -> SubstanceReferenceInformation_Classification {
        SubstanceReferenceInformation_Classification {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Todo.
    pub fn classification(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("classification") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Todo.
    pub fn domain(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("domain") {
            return Some(CodeableConcept {
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

    /// Todo.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Todo.
    pub fn subtype(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subtype") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.classification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.domain() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.subtype() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceReferenceInformation_ClassificationBuilder {
    pub(crate) value: Value,
}

impl SubstanceReferenceInformation_ClassificationBuilder {
    pub fn build(&self) -> SubstanceReferenceInformation_Classification {
        SubstanceReferenceInformation_Classification {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceReferenceInformation_Classification,
    ) -> SubstanceReferenceInformation_ClassificationBuilder {
        SubstanceReferenceInformation_ClassificationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceReferenceInformation_ClassificationBuilder {
        let mut __value: Value = json!({});
        return SubstanceReferenceInformation_ClassificationBuilder { value: __value };
    }

    pub fn classification<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["classification"] = json!(val.value);
        return self;
    }

    pub fn domain<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["domain"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subtype<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceReferenceInformation_ClassificationBuilder {
        self.value["subtype"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
