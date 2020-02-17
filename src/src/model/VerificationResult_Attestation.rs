#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes validation requirements, source(s), status and dates for one or more
/// elements.

#[derive(Debug)]
pub struct VerificationResult_Attestation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl VerificationResult_Attestation<'_> {
    pub fn new(value: &Value) -> VerificationResult_Attestation {
        VerificationResult_Attestation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for proxyIdentityCertificate
    pub fn _proxy_identity_certificate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_proxyIdentityCertificate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sourceIdentityCertificate
    pub fn _source_identity_certificate(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceIdentityCertificate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The method by which attested information was submitted/retrieved (manual; API;
    /// Push).
    pub fn communication_method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("communicationMethod") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date the information was attested to.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    /// When the who is asserting on behalf of another (organization or individual).
    pub fn on_behalf_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("onBehalfOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A digital identity certificate associated with the proxy entity submitting
    /// attested information on behalf of the attestation source.
    pub fn proxy_identity_certificate(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("proxyIdentityCertificate") {
            return Some(string);
        }
        return None;
    }

    /// Signed assertion by the proxy entity indicating that they have the right to
    /// submit attested information on behalf of the attestation source.
    pub fn proxy_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("proxySignature") {
            return Some(Signature {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A digital identity certificate associated with the attestation source.
    pub fn source_identity_certificate(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceIdentityCertificate") {
            return Some(string);
        }
        return None;
    }

    /// Signed assertion by the attestation source that they have attested to the
    /// information.
    pub fn source_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("sourceSignature") {
            return Some(Signature {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The individual or organization attesting to information.
    pub fn who(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("who") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._proxy_identity_certificate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source_identity_certificate() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.communication_method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.date() {}
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
        if let Some(_val) = self.on_behalf_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.proxy_identity_certificate() {}
        if let Some(_val) = self.proxy_signature() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source_identity_certificate() {}
        if let Some(_val) = self.source_signature() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.who() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct VerificationResult_AttestationBuilder {
    pub(crate) value: Value,
}

impl VerificationResult_AttestationBuilder {
    pub fn build(&self) -> VerificationResult_Attestation {
        VerificationResult_Attestation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: VerificationResult_Attestation) -> VerificationResult_AttestationBuilder {
        VerificationResult_AttestationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> VerificationResult_AttestationBuilder {
        let mut __value: Value = json!({});
        return VerificationResult_AttestationBuilder { value: __value };
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _proxy_identity_certificate<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["_proxyIdentityCertificate"] = json!(val.value);
        return self;
    }

    pub fn _source_identity_certificate<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["_sourceIdentityCertificate"] = json!(val.value);
        return self;
    }

    pub fn communication_method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["communicationMethod"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_behalf_of<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["onBehalfOf"] = json!(val.value);
        return self;
    }

    pub fn proxy_identity_certificate<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["proxyIdentityCertificate"] = json!(val);
        return self;
    }

    pub fn proxy_signature<'a>(
        &'a mut self,
        val: Signature,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["proxySignature"] = json!(val.value);
        return self;
    }

    pub fn source_identity_certificate<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["sourceIdentityCertificate"] = json!(val);
        return self;
    }

    pub fn source_signature<'a>(
        &'a mut self,
        val: Signature,
    ) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["sourceSignature"] = json!(val.value);
        return self;
    }

    pub fn who<'a>(&'a mut self, val: Reference) -> &'a mut VerificationResult_AttestationBuilder {
        self.value["who"] = json!(val.value);
        return self;
    }
}
