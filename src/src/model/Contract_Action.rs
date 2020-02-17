#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Contract_Subject::Contract_Subject;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Action<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_Action<'_> {
    pub fn new(value: &Value) -> Contract_Action {
        Contract_Action {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for contextLinkId
    pub fn _context_link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_contextLinkId") {
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

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
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

    /// Extensions for occurrenceDateTime
    pub fn _occurrence_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_occurrenceDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for performerLinkId
    pub fn _performer_link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_performerLinkId") {
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

    /// Extensions for reason
    pub fn _reason(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_reason") {
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

    /// Extensions for reasonLinkId
    pub fn _reason_link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_reasonLinkId") {
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

    /// Extensions for requesterLinkId
    pub fn _requester_link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_requesterLinkId") {
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

    /// Encounter or Episode with primary association to specified term activity.
    pub fn context(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("context") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Id [identifier??] of the clause or question text related to the requester of
    /// this action in the referenced form or QuestionnaireResponse.
    pub fn context_link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("contextLinkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// True if the term prohibits the  action.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
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

    /// Reason or purpose for the action stipulated by this Contract Provision.
    pub fn intent(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["intent"]),
        }
    }

    /// Id [identifier??] of the clause or question text related to this action in the
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

    /// Comments made about the term action made by the requester, performer, subject or
    /// other participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// When action happens.
    pub fn occurrence_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("occurrenceDateTime") {
            return Some(string);
        }
        return None;
    }

    /// When action happens.
    pub fn occurrence_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("occurrencePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// When action happens.
    pub fn occurrence_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("occurrenceTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates who or what is being asked to perform (or not perform) the ction.
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Id [identifier??] of the clause or question text related to the reason type or
    /// reference of this  action in the referenced form or QuestionnaireResponse.
    pub fn performer_link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("performerLinkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The type of role or competency of an individual desired or required to perform
    /// or not perform the action.
    pub fn performer_role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("performerRole") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of individual that is desired or required to perform or not perform the
    /// action.
    pub fn performer_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("performerType") {
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

    /// Describes why the action is to be performed or not performed in textual form.
    pub fn reason(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Rationale for the action to be performed or not performed. Describes why the
    /// action is permitted or prohibited.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
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

    /// Id [identifier??] of the clause or question text related to the reason type or
    /// reference of this  action in the referenced form or QuestionnaireResponse.
    pub fn reason_link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("reasonLinkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates another resource whose existence justifies permitting or not
    /// permitting this action.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
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

    /// Who or what initiated the action and has responsibility for its activation.
    pub fn requester(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("requester") {
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

    /// Id [identifier??] of the clause or question text related to the requester of
    /// this action in the referenced form or QuestionnaireResponse.
    pub fn requester_link_id(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("requesterLinkId") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Security labels that protects the action.
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

    /// Current state of the term action.
    pub fn status(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["status"]),
        }
    }

    /// Entity of the action.
    pub fn subject(&self) -> Option<Vec<Contract_Subject>> {
        if let Some(Value::Array(val)) = self.value.get("subject") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Subject {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Activity or service obligation to be done or not done, performed or not
    /// performed, effectuated or not by this Contract term.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._context_link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._do_not_perform() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._occurrence_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._performer_link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._reason_link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._requester_link_id() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._security_label_number() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.context_link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if !self.intent().validate() {
            return false;
        }
        if let Some(_val) = self.link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_date_time() {}
        if let Some(_val) = self.occurrence_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.occurrence_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performer_link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.performer_role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performer_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.reason_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.reason_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requester() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requester_link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.security_label_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if !self.status().validate() {
            return false;
        }
        if let Some(_val) = self.subject() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct Contract_ActionBuilder {
    pub(crate) value: Value,
}

impl Contract_ActionBuilder {
    pub fn build(&self) -> Contract_Action {
        Contract_Action {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_Action) -> Contract_ActionBuilder {
        Contract_ActionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        intent: CodeableConcept,
        status: CodeableConcept,
        fhir_type: CodeableConcept,
    ) -> Contract_ActionBuilder {
        let mut __value: Value = json!({});
        __value["intent"] = json!(intent.value);
        __value["status"] = json!(status.value);
        __value["type"] = json!(fhir_type.value);
        return Contract_ActionBuilder { value: __value };
    }

    pub fn _context_link_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_ActionBuilder {
        self.value["_contextLinkId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _do_not_perform<'a>(&'a mut self, val: Element) -> &'a mut Contract_ActionBuilder {
        self.value["_doNotPerform"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_ActionBuilder {
        self.value["_linkId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _occurrence_date_time<'a>(&'a mut self, val: Element) -> &'a mut Contract_ActionBuilder {
        self.value["_occurrenceDateTime"] = json!(val.value);
        return self;
    }

    pub fn _performer_link_id<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["_performerLinkId"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _reason<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_ActionBuilder {
        self.value["_reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _reason_link_id<'a>(&'a mut self, val: Vec<Element>) -> &'a mut Contract_ActionBuilder {
        self.value["_reasonLinkId"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _requester_link_id<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["_requesterLinkId"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _security_label_number<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["_securityLabelNumber"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn context<'a>(&'a mut self, val: Reference) -> &'a mut Contract_ActionBuilder {
        self.value["context"] = json!(val.value);
        return self;
    }

    pub fn context_link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["contextLinkId"] = json!(val);
        return self;
    }

    pub fn do_not_perform<'a>(&'a mut self, val: bool) -> &'a mut Contract_ActionBuilder {
        self.value["doNotPerform"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_ActionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_ActionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut Contract_ActionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn occurrence_date_time<'a>(&'a mut self, val: &str) -> &'a mut Contract_ActionBuilder {
        self.value["occurrenceDateTime"] = json!(val);
        return self;
    }

    pub fn occurrence_period<'a>(&'a mut self, val: Period) -> &'a mut Contract_ActionBuilder {
        self.value["occurrencePeriod"] = json!(val.value);
        return self;
    }

    pub fn occurrence_timing<'a>(&'a mut self, val: Timing) -> &'a mut Contract_ActionBuilder {
        self.value["occurrenceTiming"] = json!(val.value);
        return self;
    }

    pub fn performer<'a>(&'a mut self, val: Reference) -> &'a mut Contract_ActionBuilder {
        self.value["performer"] = json!(val.value);
        return self;
    }

    pub fn performer_link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["performerLinkId"] = json!(val);
        return self;
    }

    pub fn performer_role<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["performerRole"] = json!(val.value);
        return self;
    }

    pub fn performer_type<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["performerType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["reason"] = json!(val);
        return self;
    }

    pub fn reason_code<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["reasonCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["reasonLinkId"] = json!(val);
        return self;
    }

    pub fn reason_reference<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["reasonReference"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requester<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut Contract_ActionBuilder {
        self.value["requester"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requester_link_id<'a>(&'a mut self, val: Vec<&str>) -> &'a mut Contract_ActionBuilder {
        self.value["requesterLinkId"] = json!(val);
        return self;
    }

    pub fn security_label_number<'a>(
        &'a mut self,
        val: Vec<u64>,
    ) -> &'a mut Contract_ActionBuilder {
        self.value["securityLabelNumber"] = json!(val);
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Vec<Contract_Subject>) -> &'a mut Contract_ActionBuilder {
        self.value["subject"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
