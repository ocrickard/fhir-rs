#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Contract_Action::Contract_Action;
use crate::model::Contract_Asset::Contract_Asset;
use crate::model::Contract_Offer::Contract_Offer;
use crate::model::Contract_SecurityLabel::Contract_SecurityLabel;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Term<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_Term<'_> {
    pub fn new(value: &Value) -> Contract_Term {
        Contract_Term {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An actor taking a role in an activity for which it can be assigned some degree
    /// of responsibility for the activity taking place.
    pub fn action(&self) -> Option<Vec<Contract_Action>> {
        if let Some(Value::Array(val)) = self.value.get("action") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Action {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Relevant time or time-period when this Contract Provision is applicable.
    pub fn applies(&self) -> Option<Period> {
        if let Some(val) = self.value.get("applies") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Contract Term Asset List.
    pub fn asset(&self) -> Option<Vec<Contract_Asset>> {
        if let Some(Value::Array(val)) = self.value.get("asset") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Asset {
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

    /// Nested group of Contract Provisions.
    pub fn group(&self) -> Option<Vec<Contract_Term>> {
        if let Some(Value::Array(val)) = self.value.get("group") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Term {
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

    /// Unique identifier for this particular Contract Provision.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When this Contract Provision was issued.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
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

    /// The matter of concern in the context of this provision of the agrement.
    pub fn offer(&self) -> Contract_Offer {
        Contract_Offer {
            value: Cow::Borrowed(&self.value["offer"]),
        }
    }

    /// Security labels that protect the handling of information about the term and its
    /// elements, which may be specifically identified..
    pub fn security_label(&self) -> Option<Vec<Contract_SecurityLabel>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabel") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_SecurityLabel {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A specialized legal clause or condition based on overarching contract type.
    pub fn sub_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Statement of a provision in a policy or a contract.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// The entity that the term applies to.
    pub fn topic_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("topicCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The entity that the term applies to.
    pub fn topic_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("topicReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A legal clause or condition contained within a contract that requires one or
    /// both parties to perform a particular requirement by some specified time or
    /// prevents one or both parties from performing a particular requirement by some
    /// specified time.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._issued() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.action() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.applies() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.asset() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.group() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.offer().validate() {
            return false;
        }
        if let Some(_val) = self.security_label() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sub_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.topic_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.topic_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_TermBuilder {
    pub(crate) value: Value,
}

impl Contract_TermBuilder {
    pub fn build(&self) -> Contract_Term {
        Contract_Term {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_Term) -> Contract_TermBuilder {
        Contract_TermBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(offer: Contract_Offer) -> Contract_TermBuilder {
        let mut __value: Value = json!({});
        __value["offer"] = json!(offer.value);
        return Contract_TermBuilder { value: __value };
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut Contract_TermBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut Contract_TermBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn action<'a>(&'a mut self, val: Vec<Contract_Action>) -> &'a mut Contract_TermBuilder {
        self.value["action"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn applies<'a>(&'a mut self, val: Period) -> &'a mut Contract_TermBuilder {
        self.value["applies"] = json!(val.value);
        return self;
    }

    pub fn asset<'a>(&'a mut self, val: Vec<Contract_Asset>) -> &'a mut Contract_TermBuilder {
        self.value["asset"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_TermBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn group<'a>(&'a mut self, val: Vec<Contract_Term>) -> &'a mut Contract_TermBuilder {
        self.value["group"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_TermBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut Contract_TermBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut Contract_TermBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_TermBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn security_label<'a>(
        &'a mut self,
        val: Vec<Contract_SecurityLabel>,
    ) -> &'a mut Contract_TermBuilder {
        self.value["securityLabel"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sub_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Contract_TermBuilder {
        self.value["subType"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut Contract_TermBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn topic_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Contract_TermBuilder {
        self.value["topicCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn topic_reference<'a>(&'a mut self, val: Reference) -> &'a mut Contract_TermBuilder {
        self.value["topicReference"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Contract_TermBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
