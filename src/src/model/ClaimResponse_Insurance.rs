#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides the adjudication details from the processing of a Claim
/// resource.

#[derive(Debug)]
pub struct ClaimResponse_Insurance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ClaimResponse_Insurance<'_> {
    pub fn new(value: &Value) -> ClaimResponse_Insurance {
        ClaimResponse_Insurance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for businessArrangement
    pub fn _business_arrangement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_businessArrangement") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for focal
    pub fn _focal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_focal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A business agreement number established between the provider and the insurer for
    /// special business processing purposes.
    pub fn business_arrangement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("businessArrangement") {
            return Some(string);
        }
        return None;
    }

    /// The result of the adjudication of the line items for the Coverage specified in
    /// this insurance.
    pub fn claim_response(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claimResponse") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reference to the insurance card level information contained in the Coverage
    /// resource. The coverage issuing insurer will use these details to locate the
    /// patient's actual coverage within the insurer's information system.
    pub fn coverage(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["coverage"]),
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

    /// A flag to indicate that this Coverage is to be used for adjudication of this
    /// claim when set to true.
    pub fn focal(&self) -> Option<bool> {
        if let Some(val) = self.value.get("focal") {
            return Some(val.as_bool().unwrap());
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

    /// A number to uniquely identify insurance entries and provide a sequence of
    /// coverages to convey coordination of benefit order.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._business_arrangement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._focal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.business_arrangement() {}
        if let Some(_val) = self.claim_response() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.coverage().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focal() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ClaimResponse_InsuranceBuilder {
    pub(crate) value: Value,
}

impl ClaimResponse_InsuranceBuilder {
    pub fn build(&self) -> ClaimResponse_Insurance {
        ClaimResponse_Insurance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ClaimResponse_Insurance) -> ClaimResponse_InsuranceBuilder {
        ClaimResponse_InsuranceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(coverage: Reference) -> ClaimResponse_InsuranceBuilder {
        let mut __value: Value = json!({});
        __value["coverage"] = json!(coverage.value);
        return ClaimResponse_InsuranceBuilder { value: __value };
    }

    pub fn _business_arrangement<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["_businessArrangement"] = json!(val.value);
        return self;
    }

    pub fn _focal<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["_focal"] = json!(val.value);
        return self;
    }

    pub fn _sequence<'a>(&'a mut self, val: Element) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn business_arrangement<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["businessArrangement"] = json!(val);
        return self;
    }

    pub fn claim_response<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["claimResponse"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focal<'a>(&'a mut self, val: bool) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["focal"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: i64) -> &'a mut ClaimResponse_InsuranceBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }
}
