#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Specifies clinical/business/etc. metadata that can be used to retrieve, index
/// and/or categorize an artifact. This metadata can either be specific to the
/// applicable population (e.g., age category, DRG) or the specific context of care
/// (e.g., venue, care setting, provider of care).

#[derive(Debug)]
pub struct UsageContext<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl UsageContext<'_> {
    pub fn new(value: &Value) -> UsageContext {
        UsageContext {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A code that identifies the type of context being specified by this usage
    /// context.
    pub fn code(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["code"]),
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

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value that defines the context specified in this context of use. The
    /// interpretation of the value is defined by the code.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct UsageContextBuilder {
    pub(crate) value: Value,
}

impl UsageContextBuilder {
    pub fn build(&self) -> UsageContext {
        UsageContext {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: UsageContext) -> UsageContextBuilder {
        UsageContextBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: Coding) -> UsageContextBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return UsageContextBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut UsageContextBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut UsageContextBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut UsageContextBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut UsageContextBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Range) -> &'a mut UsageContextBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }

    pub fn value_reference<'a>(&'a mut self, val: Reference) -> &'a mut UsageContextBuilder {
        self.value["valueReference"] = json!(val.value);
        return self;
    }
}
