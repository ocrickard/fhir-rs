#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Detailed definition of a medicinal product, typically for uses other than direct
/// patient care (e.g. regulatory use).

#[derive(Debug)]
pub struct MedicinalProduct_SpecialDesignation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProduct_SpecialDesignation<'_> {
    pub fn new(value: &Value) -> MedicinalProduct_SpecialDesignation {
        MedicinalProduct_SpecialDesignation {
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

    /// Date when the designation was granted.
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

    /// Identifier for the designation, or procedure number.
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

    /// Condition for which the medicinal use applies.
    pub fn indication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("indicationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Condition for which the medicinal use applies.
    pub fn indication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("indicationReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The intended use of the product, e.g. prevention, treatment.
    pub fn intended_use(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("intendedUse") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// Animal species for which this applies.
    pub fn species(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("species") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// For example granted, pending, expired or withdrawn.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of special designation, e.g. orphan drug, minor use.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
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
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.extension() {
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
        if let Some(_val) = self.indication_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.indication_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.intended_use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.species() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {
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
pub struct MedicinalProduct_SpecialDesignationBuilder {
    pub(crate) value: Value,
}

impl MedicinalProduct_SpecialDesignationBuilder {
    pub fn build(&self) -> MedicinalProduct_SpecialDesignation {
        MedicinalProduct_SpecialDesignation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProduct_SpecialDesignation,
    ) -> MedicinalProduct_SpecialDesignationBuilder {
        MedicinalProduct_SpecialDesignationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicinalProduct_SpecialDesignationBuilder {
        let mut __value: Value = json!({});
        return MedicinalProduct_SpecialDesignationBuilder { value: __value };
    }

    pub fn _date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn indication_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["indicationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn indication_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["indicationReference"] = json!(val.value);
        return self;
    }

    pub fn intended_use<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["intendedUse"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn species<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["species"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["status"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProduct_SpecialDesignationBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
