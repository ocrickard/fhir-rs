#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Signature::Signature;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract_Signer<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract_Signer<'_> {
    pub fn new(value: &Value) -> Contract_Signer {
        Contract_Signer {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Party which is a signator to this Contract.
    pub fn party(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["party"]),
        }
    }

    /// Legally binding Contract DSIG signature contents in Base64.
    pub fn signature(&self) -> Vec<Signature> {
        self.value
            .get("signature")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Signature {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// Role of this Contract signer, e.g. notary, grantee.
    pub fn fhir_type(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
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
        if !self.party().validate() {
            return false;
        }
        if !self
            .signature()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if !self.fhir_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Contract_SignerBuilder {
    pub(crate) value: Value,
}

impl Contract_SignerBuilder {
    pub fn build(&self) -> Contract_Signer {
        Contract_Signer {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract_Signer) -> Contract_SignerBuilder {
        Contract_SignerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        party: Reference,
        signature: Vec<Signature>,
        fhir_type: Coding,
    ) -> Contract_SignerBuilder {
        let mut __value: Value = json!({});
        __value["party"] = json!(party.value);
        __value["signature"] = json!(signature.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["type"] = json!(fhir_type.value);
        return Contract_SignerBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Contract_SignerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Contract_SignerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Contract_SignerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
