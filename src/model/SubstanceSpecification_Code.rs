#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Code<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Code<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Code {
        SubstanceSpecification_Code {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for statusDate
    pub fn _status_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_statusDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specific code.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Any comment can be provided in this field, if necessary.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// Supporting literature.
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

    /// Status of the code assignment.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date at which the code status is changed as part of the terminology
    /// maintenance.
    pub fn status_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("statusDate") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
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
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status_date() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_CodeBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_CodeBuilder {
    pub fn build(&self) -> SubstanceSpecification_Code {
        SubstanceSpecification_Code {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSpecification_Code) -> SubstanceSpecification_CodeBuilder {
        SubstanceSpecification_CodeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_CodeBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_CodeBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _status_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["_statusDate"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn status_date<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_CodeBuilder {
        self.value["statusDate"] = json!(val);
        return self;
    }
}
