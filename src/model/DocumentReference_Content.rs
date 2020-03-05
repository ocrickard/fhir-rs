#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reference to a document of any kind for any purpose. Provides metadata about
/// the document so that the document can be discovered and managed. The scope of a
/// document is any seralized object with a mime-type, so includes formal patient
/// centric documents (CDA), cliical notes, scanned paper, and non-patient specific
/// documents like policy text.

#[derive(Debug)]
pub struct DocumentReference_Content<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl DocumentReference_Content<'_> {
    pub fn new(value: &Value) -> DocumentReference_Content {
        DocumentReference_Content {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The document or URL of the document along with critical metadata to prove
    /// content has integrity.
    pub fn attachment(&self) -> Attachment {
        Attachment {
            value: Cow::Borrowed(&self.value["attachment"]),
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

    /// An identifier of the document encoding, structure, and template that the
    /// document conforms to beyond the base format indicated in the mimeType.
    pub fn format(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("format") {
            return Some(Coding {
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
        if !self.attachment().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.format() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct DocumentReference_ContentBuilder {
    pub(crate) value: Value,
}

impl DocumentReference_ContentBuilder {
    pub fn build(&self) -> DocumentReference_Content {
        DocumentReference_Content {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: DocumentReference_Content) -> DocumentReference_ContentBuilder {
        DocumentReference_ContentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(attachment: Attachment) -> DocumentReference_ContentBuilder {
        let mut __value: Value = json!({});
        __value["attachment"] = json!(attachment.value);
        return DocumentReference_ContentBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DocumentReference_ContentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn format<'a>(&'a mut self, val: Coding) -> &'a mut DocumentReference_ContentBuilder {
        self.value["format"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut DocumentReference_ContentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut DocumentReference_ContentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
