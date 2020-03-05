#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The shelf-life and storage information for a medicinal product item or container
/// can be described using this class.

#[derive(Debug)]
pub struct ProductShelfLife<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ProductShelfLife<'_> {
    pub fn new(value: &Value) -> ProductShelfLife {
        ProductShelfLife {
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

    /// Unique identifier for the packaged Medicinal Product.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
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

    /// The shelf life time period can be specified using a numerical value for the
    /// period of time and its unit of time measurement The unit of measurement shall be
    /// specified in accordance with ISO 11240 and the resulting terminology The symbol
    /// and the symbol identifier shall be used.
    pub fn period(&self) -> Quantity {
        Quantity {
            value: Cow::Borrowed(&self.value["period"]),
        }
    }

    /// Special precautions for storage, if any, can be specified using an appropriate
    /// controlled vocabulary The controlled term and the controlled term identifier
    /// shall be specified.
    pub fn special_precautions_for_storage(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("specialPrecautionsForStorage") {
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

    /// This describes the shelf life, taking into account various scenarios such as
    /// shelf life of the packaged Medicinal Product itself, shelf life after
    /// transformation where necessary and shelf life after the first opening of a
    /// bottle, etc. The shelf life type shall be specified using an appropriate
    /// controlled vocabulary The controlled term and the controlled term identifier
    /// shall be specified.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.period().validate() {
            return false;
        }
        if let Some(_val) = self.special_precautions_for_storage() {
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
pub struct ProductShelfLifeBuilder {
    pub(crate) value: Value,
}

impl ProductShelfLifeBuilder {
    pub fn build(&self) -> ProductShelfLife {
        ProductShelfLife {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ProductShelfLife) -> ProductShelfLifeBuilder {
        ProductShelfLifeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(period: Quantity, fhir_type: CodeableConcept) -> ProductShelfLifeBuilder {
        let mut __value: Value = json!({});
        __value["period"] = json!(period.value);
        __value["type"] = json!(fhir_type.value);
        return ProductShelfLifeBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ProductShelfLifeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ProductShelfLifeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut ProductShelfLifeBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ProductShelfLifeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn special_precautions_for_storage<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ProductShelfLifeBuilder {
        self.value["specialPrecautionsForStorage"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
