#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::SubstanceSpecification_Official::SubstanceSpecification_Official;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Name<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Name<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Name {
        SubstanceSpecification_Name {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for preferred
    pub fn _preferred(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preferred") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The use context of this name for example if there is a different name a drug
    /// active ingredient as opposed to a food colour additive.
    pub fn domain(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("domain") {
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

    /// The jurisdiction where this name applies.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
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

    /// Language of the name.
    pub fn language(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("language") {
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

    /// The actual name.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Details of the official nature of this name.
    pub fn official(&self) -> Option<Vec<SubstanceSpecification_Official>> {
        if let Some(Value::Array(val)) = self.value.get("official") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Official {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If this is the preferred name for this substance.
    pub fn preferred(&self) -> Option<bool> {
        if let Some(val) = self.value.get("preferred") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Supporting literature.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
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

    /// The status of the name.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A synonym of this name.
    pub fn synonym(&self) -> Option<Vec<SubstanceSpecification_Name>> {
        if let Some(Value::Array(val)) = self.value.get("synonym") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Name {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A translation for this name.
    pub fn translation(&self) -> Option<Vec<SubstanceSpecification_Name>> {
        if let Some(Value::Array(val)) = self.value.get("translation") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Name {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Name type.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._preferred() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.domain() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {
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
        if let Some(_val) = self.official() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.preferred() {}
        if let Some(_val) = self.source() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.synonym() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.translation() {
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
pub struct SubstanceSpecification_NameBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_NameBuilder {
    pub fn build(&self) -> SubstanceSpecification_Name {
        SubstanceSpecification_Name {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSpecification_Name) -> SubstanceSpecification_NameBuilder {
        SubstanceSpecification_NameBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_NameBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_NameBuilder { value: __value };
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _preferred<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["_preferred"] = json!(val.value);
        return self;
    }

    pub fn domain<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["domain"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["language"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn official<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Official>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["official"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn preferred<'a>(&'a mut self, val: bool) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["preferred"] = json!(val);
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["source"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn synonym<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Name>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["synonym"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn translation<'a>(
        &'a mut self,
        val: Vec<SubstanceSpecification_Name>,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["translation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_NameBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
