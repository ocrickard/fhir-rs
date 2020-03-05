#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CoverageEligibilityResponse_Benefit::CoverageEligibilityResponse_Benefit;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse_Item<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CoverageEligibilityResponse_Item<'_> {
    pub fn new(value: &Value) -> CoverageEligibilityResponse_Item {
        CoverageEligibilityResponse_Item {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for authorizationRequired
    pub fn _authorization_required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authorizationRequired") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for authorizationUrl
    pub fn _authorization_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_authorizationUrl") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for excluded
    pub fn _excluded(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_excluded") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A boolean flag indicating whether a preauthorization is required prior to actual
    /// service delivery.
    pub fn authorization_required(&self) -> Option<bool> {
        if let Some(val) = self.value.get("authorizationRequired") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Codes or comments regarding information or actions associated with the
    /// preauthorization.
    pub fn authorization_supporting(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("authorizationSupporting") {
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

    /// A web location for obtaining requirements or descriptive information regarding
    /// the preauthorization.
    pub fn authorization_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("authorizationUrl") {
            return Some(string);
        }
        return None;
    }

    /// Benefits used to date.
    pub fn benefit(&self) -> Option<Vec<CoverageEligibilityResponse_Benefit>> {
        if let Some(Value::Array(val)) = self.value.get("benefit") {
            return Some(
                val.into_iter()
                    .map(|e| CoverageEligibilityResponse_Benefit {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Code to identify the general type of benefits under which products and services
    /// are provided.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A richer description of the benefit or services covered.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// True if the indicated class of service is excluded from the plan, missing or
    /// False indicates the product or service is included in the coverage.
    pub fn excluded(&self) -> Option<bool> {
        if let Some(val) = self.value.get("excluded") {
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

    /// Item typification or modifiers codes to convey additional context for the
    /// product or service.
    pub fn modifier(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("modifier") {
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

    /// A short name or tag for the benefit.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Is a flag to indicate whether the benefits refer to in-network providers or out-
    /// of-network providers.
    pub fn network(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("network") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This contains the product, service, drug or other billing code for the item.
    pub fn product_or_service(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productOrService") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The practitioner who is eligible for the provision of the product or service.
    pub fn provider(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("provider") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The term or period of the values such as 'maximum lifetime benefit' or 'maximum
    /// annual visits'.
    pub fn term(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("term") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates if the benefits apply to an individual or to the family.
    pub fn unit(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unit") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._authorization_required() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._authorization_url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._excluded() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authorization_required() {}
        if let Some(_val) = self.authorization_supporting() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.authorization_url() {}
        if let Some(_val) = self.benefit() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.excluded() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.network() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.product_or_service() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.provider() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.term() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CoverageEligibilityResponse_ItemBuilder {
    pub(crate) value: Value,
}

impl CoverageEligibilityResponse_ItemBuilder {
    pub fn build(&self) -> CoverageEligibilityResponse_Item {
        CoverageEligibilityResponse_Item {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: CoverageEligibilityResponse_Item,
    ) -> CoverageEligibilityResponse_ItemBuilder {
        CoverageEligibilityResponse_ItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CoverageEligibilityResponse_ItemBuilder {
        let mut __value: Value = json!({});
        return CoverageEligibilityResponse_ItemBuilder { value: __value };
    }

    pub fn _authorization_required<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["_authorizationRequired"] = json!(val.value);
        return self;
    }

    pub fn _authorization_url<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["_authorizationUrl"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _excluded<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["_excluded"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn authorization_required<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["authorizationRequired"] = json!(val);
        return self;
    }

    pub fn authorization_supporting<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["authorizationSupporting"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn authorization_url<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["authorizationUrl"] = json!(val);
        return self;
    }

    pub fn benefit<'a>(
        &'a mut self,
        val: Vec<CoverageEligibilityResponse_Benefit>,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["benefit"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn excluded<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["excluded"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["modifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn network<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["network"] = json!(val.value);
        return self;
    }

    pub fn product_or_service<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["productOrService"] = json!(val.value);
        return self;
    }

    pub fn provider<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["provider"] = json!(val.value);
        return self;
    }

    pub fn term<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["term"] = json!(val.value);
        return self;
    }

    pub fn unit<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponse_ItemBuilder {
        self.value["unit"] = json!(val.value);
        return self;
    }
}
