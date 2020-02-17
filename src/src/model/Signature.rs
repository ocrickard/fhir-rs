#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A signature along with supporting context. The signature may be a digital
/// signature that is cryptographic in nature, or some other signature acceptable to
/// the domain. This other signature may be as simple as a graphical image
/// representing a hand-written signature, or a signature ceremony Different
/// signature approaches have different utilities.

#[derive(Debug)]
pub struct Signature<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Signature<'_> {
    pub fn new(value: &Value) -> Signature {
        Signature {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for data
    pub fn _data(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_data") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sigFormat
    pub fn _sig_format(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sigFormat") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for targetFormat
    pub fn _target_format(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetFormat") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for when
    pub fn _when(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_when") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The base64 encoding of the Signature content. When signature is not recorded
    /// electronically this element would be empty.
    pub fn data(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("data") {
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

    /// A reference to an application-usable description of the identity that is
    /// represented by the signature.
    pub fn on_behalf_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("onBehalfOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A mime type that indicates the technical format of the signature. Important mime
    /// types are application/signature+xml for X ML DigSig, application/jose for JWS,
    /// and image/* for a graphical image of a signature, etc.
    pub fn sig_format(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sigFormat") {
            return Some(string);
        }
        return None;
    }

    /// A mime type that indicates the technical format of the target resources signed
    /// by the signature.
    pub fn target_format(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetFormat") {
            return Some(string);
        }
        return None;
    }

    /// An indication of the reason that the entity signed this document. This may be
    /// explicitly included as part of the signature information and can be used when
    /// determining accountability for various actions concerning the document.
    pub fn fhir_type(&self) -> Vec<Coding> {
        self.value
            .get("type")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Coding {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// When the digital signature was signed.
    pub fn when(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("when") {
            return Some(string);
        }
        return None;
    }

    /// A reference to an application-usable description of the identity that signed
    /// (e.g. the signature used their private key).
    pub fn who(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["who"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sig_format() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._target_format() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._when() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.data() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.on_behalf_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sig_format() {}
        if let Some(_val) = self.target_format() {}
        if !self
            .fhir_type()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.when() {}
        if !self.who().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SignatureBuilder {
    pub(crate) value: Value,
}

impl SignatureBuilder {
    pub fn build(&self) -> Signature {
        Signature {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Signature) -> SignatureBuilder {
        SignatureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: Vec<Coding>, who: Reference) -> SignatureBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["who"] = json!(who.value);
        return SignatureBuilder { value: __value };
    }

    pub fn _data<'a>(&'a mut self, val: Element) -> &'a mut SignatureBuilder {
        self.value["_data"] = json!(val.value);
        return self;
    }

    pub fn _sig_format<'a>(&'a mut self, val: Element) -> &'a mut SignatureBuilder {
        self.value["_sigFormat"] = json!(val.value);
        return self;
    }

    pub fn _target_format<'a>(&'a mut self, val: Element) -> &'a mut SignatureBuilder {
        self.value["_targetFormat"] = json!(val.value);
        return self;
    }

    pub fn _when<'a>(&'a mut self, val: Element) -> &'a mut SignatureBuilder {
        self.value["_when"] = json!(val.value);
        return self;
    }

    pub fn data<'a>(&'a mut self, val: &str) -> &'a mut SignatureBuilder {
        self.value["data"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SignatureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SignatureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn on_behalf_of<'a>(&'a mut self, val: Reference) -> &'a mut SignatureBuilder {
        self.value["onBehalfOf"] = json!(val.value);
        return self;
    }

    pub fn sig_format<'a>(&'a mut self, val: &str) -> &'a mut SignatureBuilder {
        self.value["sigFormat"] = json!(val);
        return self;
    }

    pub fn target_format<'a>(&'a mut self, val: &str) -> &'a mut SignatureBuilder {
        self.value["targetFormat"] = json!(val);
        return self;
    }

    pub fn when<'a>(&'a mut self, val: &str) -> &'a mut SignatureBuilder {
        self.value["when"] = json!(val);
        return self;
    }
}
