#![allow(unused_imports, non_camel_case_types)]

use crate::model::CoverageEligibilityResponse_Item::CoverageEligibilityResponse_Item;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse_Insurance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CoverageEligibilityResponse_Insurance<'_> {
    pub fn new(value: &Value) -> CoverageEligibilityResponse_Insurance {
        CoverageEligibilityResponse_Insurance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for inforce
    pub fn _inforce(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_inforce") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The term of the benefits documented in this response.
    pub fn benefit_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("benefitPeriod") {
            return Some(Period {
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Flag indicating if the coverage provided is inforce currently if no service
    /// date(s) specified or for the whole duration of the service dates.
    pub fn inforce(&self) -> Option<bool> {
        if let Some(val) = self.value.get("inforce") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Benefits and optionally current balances, and authorization details by category
    /// or service.
    pub fn item(&self) -> Option<Vec<CoverageEligibilityResponse_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| CoverageEligibilityResponse_Item {
                        value: Cow::Borrowed(e),
                    })
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._inforce() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.benefit_period() {
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
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.inforce() {}
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CoverageEligibilityResponse_InsuranceBuilder {
    pub(crate) value: Value,
}

impl CoverageEligibilityResponse_InsuranceBuilder {
    pub fn build(&self) -> CoverageEligibilityResponse_Insurance {
        CoverageEligibilityResponse_Insurance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: CoverageEligibilityResponse_Insurance,
    ) -> CoverageEligibilityResponse_InsuranceBuilder {
        CoverageEligibilityResponse_InsuranceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(coverage: Reference) -> CoverageEligibilityResponse_InsuranceBuilder {
        let mut __value: Value = json!({});
        __value["coverage"] = json!(coverage.value);
        return CoverageEligibilityResponse_InsuranceBuilder { value: __value };
    }

    pub fn _inforce<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["_inforce"] = json!(val.value);
        return self;
    }

    pub fn benefit_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["benefitPeriod"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn inforce<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["inforce"] = json!(val);
        return self;
    }

    pub fn item<'a>(
        &'a mut self,
        val: Vec<CoverageEligibilityResponse_Item>,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_InsuranceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
