#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The clinical particulars - indications, contraindications etc. of a medicinal
/// product, including for regulatory purposes.

#[derive(Debug)]
pub struct MedicinalProductContraindication_OtherTherapy<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicinalProductContraindication_OtherTherapy<'_> {
    pub fn new(value: &Value) -> MedicinalProductContraindication_OtherTherapy {
        MedicinalProductContraindication_OtherTherapy {
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

    /// Reference to a specific medication (active substance, medicinal product or class
    /// of products) as part of an indication or contraindication.
    pub fn medication_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("medicationCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Reference to a specific medication (active substance, medicinal product or class
    /// of products) as part of an indication or contraindication.
    pub fn medication_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("medicationReference") {
            return Some(Reference {
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

    /// The type of relationship between the medicinal product indication or
    /// contraindication and another therapy.
    pub fn therapy_relationship_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["therapyRelationshipType"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.medication_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.medication_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.therapy_relationship_type().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicinalProductContraindication_OtherTherapyBuilder {
    pub(crate) value: Value,
}

impl MedicinalProductContraindication_OtherTherapyBuilder {
    pub fn build(&self) -> MedicinalProductContraindication_OtherTherapy {
        MedicinalProductContraindication_OtherTherapy {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicinalProductContraindication_OtherTherapy,
    ) -> MedicinalProductContraindication_OtherTherapyBuilder {
        MedicinalProductContraindication_OtherTherapyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        therapy_relationship_type: CodeableConcept,
    ) -> MedicinalProductContraindication_OtherTherapyBuilder {
        let mut __value: Value = json!({});
        __value["therapyRelationshipType"] = json!(therapy_relationship_type.value);
        return MedicinalProductContraindication_OtherTherapyBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductContraindication_OtherTherapyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut MedicinalProductContraindication_OtherTherapyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn medication_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicinalProductContraindication_OtherTherapyBuilder {
        self.value["medicationCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn medication_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut MedicinalProductContraindication_OtherTherapyBuilder {
        self.value["medicationReference"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicinalProductContraindication_OtherTherapyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
