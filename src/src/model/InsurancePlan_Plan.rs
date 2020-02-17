#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::InsurancePlan_GeneralCost::InsurancePlan_GeneralCost;
use crate::model::InsurancePlan_SpecificCost::InsurancePlan_SpecificCost;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Plan<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_Plan<'_> {
    pub fn new(value: &Value) -> InsurancePlan_Plan {
        InsurancePlan_Plan {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The geographic region in which a health insurance plan's benefits apply.
    pub fn coverage_area(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("coverageArea") {
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

    /// Overall costs associated with the plan.
    pub fn general_cost(&self) -> Option<Vec<InsurancePlan_GeneralCost>> {
        if let Some(Value::Array(val)) = self.value.get("generalCost") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_GeneralCost {
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

    /// Business identifiers assigned to this health insurance plan which remain
    /// constant as the resource is updated and propagates from server to server.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
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

    /// Reference to the network that providing the type of coverage.
    pub fn network(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("network") {
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

    /// Costs associated with the coverage provided by the product.
    pub fn specific_cost(&self) -> Option<Vec<InsurancePlan_SpecificCost>> {
        if let Some(Value::Array(val)) = self.value.get("specificCost") {
            return Some(
                val.into_iter()
                    .map(|e| InsurancePlan_SpecificCost {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of plan. For example, "Platinum" or "High Deductable".
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.coverage_area() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.general_cost() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.network() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specific_cost() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
pub struct InsurancePlan_PlanBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_PlanBuilder {
    pub fn build(&self) -> InsurancePlan_Plan {
        InsurancePlan_Plan {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_Plan) -> InsurancePlan_PlanBuilder {
        InsurancePlan_PlanBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> InsurancePlan_PlanBuilder {
        let mut __value: Value = json!({});
        return InsurancePlan_PlanBuilder { value: __value };
    }

    pub fn coverage_area<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["coverageArea"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn general_cost<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_GeneralCost>,
    ) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["generalCost"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn network<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["network"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specific_cost<'a>(
        &'a mut self,
        val: Vec<InsurancePlan_SpecificCost>,
    ) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["specificCost"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut InsurancePlan_PlanBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
