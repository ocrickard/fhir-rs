#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reference to a document of any kind for any purpose. Provides metadata about
/// the document so that the document can be discovered and managed. The scope of a
/// document is any seralized object with a mime-type, so includes formal patient
/// centric documents (CDA), cliical notes, scanned paper, and non-patient specific
/// documents like policy text.

#[derive(Debug)]
pub struct DocumentReference_RelatesTo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DocumentReference_RelatesTo<'_> {
    pub fn new(value: &Value) -> DocumentReference_RelatesTo {
        DocumentReference_RelatesTo {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for code
    pub fn _code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_code") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of relationship that this document has with anther document.
    pub fn code(&self) -> Option<DocumentReference_RelatesToCode> {
        if let Some(Value::String(val)) = self.value.get("code") {
            return Some(DocumentReference_RelatesToCode::from_string(&val).unwrap());
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

    /// The target document of this relationship.
    pub fn target(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["target"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {}
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
        if !self.target().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DocumentReference_RelatesToBuilder {
    pub(crate) value: Value,
}

impl DocumentReference_RelatesToBuilder {
    pub fn build(&self) -> DocumentReference_RelatesTo {
        DocumentReference_RelatesTo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DocumentReference_RelatesTo) -> DocumentReference_RelatesToBuilder {
        DocumentReference_RelatesToBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(target: Reference) -> DocumentReference_RelatesToBuilder {
        let mut __value: Value = json!({});
        __value["target"] = json!(target.value);
        return DocumentReference_RelatesToBuilder { value: __value };
    }

    pub fn _code<'a>(&'a mut self, val: Element) -> &'a mut DocumentReference_RelatesToBuilder {
        self.value["_code"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: DocumentReference_RelatesToCode,
    ) -> &'a mut DocumentReference_RelatesToBuilder {
        self.value["code"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DocumentReference_RelatesToBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DocumentReference_RelatesToBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DocumentReference_RelatesToBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum DocumentReference_RelatesToCode {
    Replaces,
    Transforms,
    Signs,
    Appends,
}

impl DocumentReference_RelatesToCode {
    pub fn from_string(string: &str) -> Option<DocumentReference_RelatesToCode> {
        match string {
            "replaces" => Some(DocumentReference_RelatesToCode::Replaces),
            "transforms" => Some(DocumentReference_RelatesToCode::Transforms),
            "signs" => Some(DocumentReference_RelatesToCode::Signs),
            "appends" => Some(DocumentReference_RelatesToCode::Appends),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DocumentReference_RelatesToCode::Replaces => "replaces".to_string(),
            DocumentReference_RelatesToCode::Transforms => "transforms".to_string(),
            DocumentReference_RelatesToCode::Signs => "signs".to_string(),
            DocumentReference_RelatesToCode::Appends => "appends".to_string(),
        }
    }
}
