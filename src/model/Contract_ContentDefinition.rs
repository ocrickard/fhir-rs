#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_ContentDefinition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_ContentDefinition<'_> {
    pub fn new(value: &Value) -> Contract_ContentDefinition {
        Contract_ContentDefinition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publicationDate
    pub fn _publication_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publicationDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publicationStatus
    pub fn _publication_status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publicationStatus") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A copyright statement relating to Contract precursor content. Copyright
    /// statements are generally legal restrictions on the use and publishing of the
    /// Contract precursor content.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
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

    /// The date (and optionally time) when the contract was published. The date must
    /// change when the business version changes and it must change if the status code
    /// changes. In addition, it should change when the substantive content of the
    /// contract changes.
    pub fn publication_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publicationDate") {
            return Some(string);
        }
        return None;
    }

    /// amended | appended | cancelled | disputed | entered-in-error | executable |
    /// executed | negotiable | offered | policy | rejected | renewed | revoked |
    /// resolved | terminated.
    pub fn publication_status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publicationStatus") {
            return Some(string);
        }
        return None;
    }

    /// The  individual or organization that published the Contract precursor content.
    pub fn publisher(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("publisher") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Detailed Precusory content type.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Precusory content structure and use, i.e., a boilerplate, template, application
    /// for a contract such as an insurance policy or benefits under a program, e.g.,
    /// workers compensation.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publication_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publication_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.copyright() {}
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
        if let Some(_val) = self.publication_date() {}
        if let Some(_val) = self.publication_status() {}
        if let Some(_val) = self.publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sub_type() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_ContentDefinitionBuilder {
    pub(crate) value: Value,
}

impl Contract_ContentDefinitionBuilder {
    pub fn build(&self) -> Contract_ContentDefinition {
        Contract_ContentDefinition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_ContentDefinition) -> Contract_ContentDefinitionBuilder {
        Contract_ContentDefinitionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> Contract_ContentDefinitionBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return Contract_ContentDefinitionBuilder { value: __value };
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _publication_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["_publicationDate"] = json!(val.value);
        return self;
    }

    pub fn _publication_status<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["_publicationStatus"] = json!(val.value);
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn publication_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["publicationDate"] = json!(val);
        return self;
    }

    pub fn publication_status<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["publicationStatus"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["publisher"] = json!(val.value);
        return self;
    }

    pub fn sub_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Contract_ContentDefinitionBuilder {
        self.value["subType"] = json!(val.value);
        return self;
    }
}
