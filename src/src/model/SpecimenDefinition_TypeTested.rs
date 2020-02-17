#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SpecimenDefinition_Container::SpecimenDefinition_Container;
use crate::model::SpecimenDefinition_Handling::SpecimenDefinition_Handling;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_TypeTested<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SpecimenDefinition_TypeTested<'_> {
    pub fn new(value: &Value) -> SpecimenDefinition_TypeTested {
        SpecimenDefinition_TypeTested {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for isDerived
    pub fn _is_derived(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isDerived") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preference
    pub fn _preference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preference") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for requirement
    pub fn _requirement(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirement") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The specimen's container.
    pub fn container(&self) -> Option<SpecimenDefinition_Container> {
        if let Some(val) = self.value.get("container") {
            return Some(SpecimenDefinition_Container {
                value: Cow::Borrowed(val),
            });
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

    /// Set of instructions for preservation/transport of the specimen at a defined
    /// temperature interval, prior the testing process.
    pub fn handling(&self) -> Option<Vec<SpecimenDefinition_Handling>> {
        if let Some(Value::Array(val)) = self.value.get("handling") {
            return Some(
                val.into_iter()
                    .map(|e| SpecimenDefinition_Handling {
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

    /// Primary of secondary specimen.
    pub fn is_derived(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isDerived") {
            return Some(val.as_bool().unwrap());
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

    /// The preference for this type of conditioned specimen.
    pub fn preference(&self) -> Option<SpecimenDefinition_TypeTestedPreference> {
        if let Some(Value::String(val)) = self.value.get("preference") {
            return Some(SpecimenDefinition_TypeTestedPreference::from_string(&val).unwrap());
        }
        return None;
    }

    /// Criterion for rejection of the specimen in its container by the laboratory.
    pub fn rejection_criterion(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("rejectionCriterion") {
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

    /// Requirements for delivery and special handling of this kind of conditioned
    /// specimen.
    pub fn requirement(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirement") {
            return Some(string);
        }
        return None;
    }

    /// The usual time that a specimen of this kind is retained after the ordered tests
    /// are completed, for the purpose of additional testing.
    pub fn retention_time(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("retentionTime") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The kind of specimen conditioned for testing expected by lab.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._is_derived() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._preference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._requirement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.container() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.handling() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.is_derived() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.preference() {}
        if let Some(_val) = self.rejection_criterion() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.requirement() {}
        if let Some(_val) = self.retention_time() {
            if !_val.validate() {
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
pub struct SpecimenDefinition_TypeTestedBuilder {
    pub(crate) value: Value,
}

impl SpecimenDefinition_TypeTestedBuilder {
    pub fn build(&self) -> SpecimenDefinition_TypeTested {
        SpecimenDefinition_TypeTested {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SpecimenDefinition_TypeTested) -> SpecimenDefinition_TypeTestedBuilder {
        SpecimenDefinition_TypeTestedBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SpecimenDefinition_TypeTestedBuilder {
        let mut __value: Value = json!({});
        return SpecimenDefinition_TypeTestedBuilder { value: __value };
    }

    pub fn _is_derived<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["_isDerived"] = json!(val.value);
        return self;
    }

    pub fn _preference<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["_preference"] = json!(val.value);
        return self;
    }

    pub fn _requirement<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["_requirement"] = json!(val.value);
        return self;
    }

    pub fn container<'a>(
        &'a mut self,
        val: SpecimenDefinition_Container,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["container"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn handling<'a>(
        &'a mut self,
        val: Vec<SpecimenDefinition_Handling>,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["handling"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn is_derived<'a>(&'a mut self, val: bool) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["isDerived"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn preference<'a>(
        &'a mut self,
        val: SpecimenDefinition_TypeTestedPreference,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["preference"] = json!(val.to_string());
        return self;
    }

    pub fn rejection_criterion<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["rejectionCriterion"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn requirement<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["requirement"] = json!(val);
        return self;
    }

    pub fn retention_time<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["retentionTime"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinition_TypeTestedBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum SpecimenDefinition_TypeTestedPreference {
    Preferred,
    Alternate,
}

impl SpecimenDefinition_TypeTestedPreference {
    pub fn from_string(string: &str) -> Option<SpecimenDefinition_TypeTestedPreference> {
        match string {
            "preferred" => Some(SpecimenDefinition_TypeTestedPreference::Preferred),
            "alternate" => Some(SpecimenDefinition_TypeTestedPreference::Alternate),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SpecimenDefinition_TypeTestedPreference::Preferred => "preferred".to_string(),
            SpecimenDefinition_TypeTestedPreference::Alternate => "alternate".to_string(),
        }
    }
}
