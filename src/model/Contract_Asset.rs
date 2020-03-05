#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Contract_Answer::Contract_Answer;
use crate::model::Contract_Context::Contract_Context;
use crate::model::Contract_ValuedItem::Contract_ValuedItem;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Asset<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_Asset<'_> {
    pub fn new(value: &Value) -> Contract_Asset {
        Contract_Asset {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_condition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_linkId") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for securityLabelNumber
    pub fn _security_label_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_securityLabelNumber") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Response to assets.
    pub fn answer(&self) -> Option<Vec<Contract_Answer>> {
        if let Some(Value::Array(val)) = self.value.get("answer") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Answer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Description of the quality and completeness of the asset that imay be a factor
    /// in its valuation.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// Circumstance of the asset.
    pub fn context(&self) -> Option<Vec<Contract_Context>> {
        if let Some(Value::Array(val)) = self.value.get("context") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Context {
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

    /// Id [identifier??] of the clause or question text about the asset in the
    /// referenced form or QuestionnaireResponse.
    pub fn link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("linkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// Asset relevant contractual time period.
    pub fn period(&self) -> Option<Vec<Period>> {
        if let Some(Value::Array(val)) = self.value.get("period") {
            return Some(
                val.into_iter()
                    .map(|e| Period {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of Asset availability for use or ownership.
    pub fn period_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("periodType") {
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

    /// Specifies the applicability of the term to an asset resource instance, and
    /// instances it refers to orinstances that refer to it, and/or are owned by the
    /// offeree.
    pub fn relationship(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("relationship") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Differentiates the kind of the asset .
    pub fn scope(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scope") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Security labels that protects the asset.
    pub fn security_label_number(&self) -> Option<Vec<u64>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabelNumber") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_u64().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be a subtype or part of an offered asset.
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

    /// Clause or question text (Prose Object) concerning the asset in a linked form,
    /// such as a QuestionnaireResponse used in the formation of the contract.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Target entity type about which the term may be concerned.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
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

    /// Associated entities.
    pub fn type_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("typeReference") {
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

    /// Time period of asset use.
    pub fn use_period(&self) -> Option<Vec<Period>> {
        if let Some(Value::Array(val)) = self.value.get("usePeriod") {
            return Some(
                val.into_iter()
                    .map(|e| Period {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Contract Valued Item List.
    pub fn valued_item(&self) -> Option<Vec<Contract_ValuedItem>> {
        if let Some(Value::Array(val)) = self.value.get("valuedItem") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_ValuedItem {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._security_label_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self.context() {
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
        if let Some(_val) = self.link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.period() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.period_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.scope() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.security_label_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.subtype() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.type_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.use_period() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.valued_item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_AssetBuilder {
    pub(crate) value: Value,
}

impl Contract_AssetBuilder {
    pub fn build(&self) -> Contract_Asset {
        Contract_Asset {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_Asset) -> Contract_AssetBuilder {
        Contract_AssetBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Contract_AssetBuilder {
        let mut __value: Value = json!({});
        return Contract_AssetBuilder { value: __value };
    }

    pub fn _condition<'a>(&'a mut self, val: Element) -> &'a mut Contract_AssetBuilder {
        self.value["_condition"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_AssetBuilder {
        self.value["_linkId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _security_label_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Contract_AssetBuilder {
        self.value["_securityLabelNumber"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut Contract_AssetBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn answer<'a>(&'a mut self, val: Vec<Contract_Answer>) -> &'a mut Contract_AssetBuilder {
        self.value["answer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn condition<'a>(&'a mut self, val: &str) -> &'a mut Contract_AssetBuilder {
        self.value["condition"] = json!(val);
        return self;
    }

    pub fn context<'a>(&'a mut self, val: Vec<Contract_Context>) -> &'a mut Contract_AssetBuilder {
        self.value["context"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_AssetBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_AssetBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_AssetBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_AssetBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Vec<Period>) -> &'a mut Contract_AssetBuilder {
        self.value["period"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Contract_AssetBuilder {
        self.value["periodType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn relationship<'a>(&'a mut self, val: Coding) -> &'a mut Contract_AssetBuilder {
        self.value["relationship"] = json!(val.value);
        return self;
    }

    pub fn scope<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Contract_AssetBuilder {
        self.value["scope"] = json!(val.value);
        return self;
    }

    pub fn security_label_number<'a>(&'a mut self, val: Vec<u64>) -> &'a mut Contract_AssetBuilder {
        self.value["securityLabelNumber"] = json!(val);
        return self;
    }

    pub fn subtype<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Contract_AssetBuilder {
        self.value["subtype"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut Contract_AssetBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut Contract_AssetBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn type_reference<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Contract_AssetBuilder {
        self.value["typeReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn use_period<'a>(&'a mut self, val: Vec<Period>) -> &'a mut Contract_AssetBuilder {
        self.value["usePeriod"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn valued_item<'a>(
        &'a mut self,
        val: Vec<Contract_ValuedItem>,
    ) -> &'a mut Contract_AssetBuilder {
        self.value["valuedItem"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
