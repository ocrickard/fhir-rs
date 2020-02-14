#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Contract_Answer::Contract_Answer;
use crate::model::Contract_Party::Contract_Party;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Offer<'a> {
    pub value: &'a Value,
}

impl Contract_Offer<'_> {
    /// The owner of an asset has the residual control rights over the asset: the right
    /// to decide all usages of the asset in any way not inconsistent with a prior
    /// contract, custom, or law (Hart, 1995, p. 30).
    pub fn topic(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("topic") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Unique identifier for this particular Contract Provision.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// How the decision about a Contract was conveyed.
    pub fn decision_mode(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("decisionMode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Human readable form of this Contract Offer.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Security labels that protects the offer.
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Response to offer text.
    pub fn answer(&self) -> Option<Vec<Contract_Answer>> {
        if let Some(Value::Array(val)) = self.value.get("answer") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Answer { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_linkId") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of Contract Provision such as specific requirements, purposes for actions,
    /// obligations, prohibitions, e.g. life time maximum benefit.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for securityLabelNumber
    pub fn _security_label_number(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_securityLabelNumber") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of choice made by accepting party with respect to an offer made by an
    /// offeror/ grantee.
    pub fn decision(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("decision") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Offer Recipient.
    pub fn party(&self) -> Option<Vec<Contract_Party>> {
        if let Some(Value::Array(val)) = self.value.get("party") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Party { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The id of the clause or question text of the offer in the referenced
    /// questionnaire/response.
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.topic() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._text() {
            _val.validate();
        }
        if let Some(_val) = self.decision_mode() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.security_label_number() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.answer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._link_id() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._security_label_number() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.decision() {
            _val.validate();
        }
        if let Some(_val) = self.party() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.link_id() {
            _val.into_iter().for_each(|_e| {});
        }
        return true;
    }
}
