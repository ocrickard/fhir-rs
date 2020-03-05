#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::InsurancePlan_Benefit::InsurancePlan_Benefit;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details of a Health Insurance product/plan provided by an organization.

#[derive(Debug)]
pub struct InsurancePlan_Coverage<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl InsurancePlan_Coverage<'_> {
    pub fn new(value: &Value) -> InsurancePlan_Coverage {
        InsurancePlan_Coverage {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Specific benefits under this type of coverage.
    pub fn benefit(&self) -> Vec<InsurancePlan_Benefit> {
        self.value
            .get("benefit")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| InsurancePlan_Benefit {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// Type of coverage  (Medical; Dental; Mental Health; Substance Abuse; Vision;
    /// Drug; Short Term; Long Term Care; Hospice; Home Health).
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if !self
            .benefit()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
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
        if let Some(_val) = self.network() {
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
pub struct InsurancePlan_CoverageBuilder {
    pub(crate) value: Value,
}

impl InsurancePlan_CoverageBuilder {
    pub fn build(&self) -> InsurancePlan_Coverage {
        InsurancePlan_Coverage {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: InsurancePlan_Coverage) -> InsurancePlan_CoverageBuilder {
        InsurancePlan_CoverageBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        benefit: Vec<InsurancePlan_Benefit>,
        fhir_type: CodeableConcept,
    ) -> InsurancePlan_CoverageBuilder {
        let mut __value: Value = json!({});
        __value["benefit"] = json!(benefit.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["type"] = json!(fhir_type.value);
        return InsurancePlan_CoverageBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_CoverageBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut InsurancePlan_CoverageBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut InsurancePlan_CoverageBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn network<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut InsurancePlan_CoverageBuilder {
        self.value["network"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
