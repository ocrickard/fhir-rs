#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Property<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Property<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Property {
        SubstanceSpecification_Property {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for amountString
    pub fn _amount_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amountString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for parameters
    pub fn _parameters(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_parameters") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Quantitative value for this property.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Quantitative value for this property.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    /// A category for this property, e.g. Physical, Chemical, Enzymatic.
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Property type e.g. viscosity, pH, isoelectric point.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A substance upon which a defining property depends (e.g. for solubility: in
    /// water, in alcohol).
    pub fn defining_substance_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("definingSubstanceCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A substance upon which a defining property depends (e.g. for solubility: in
    /// water, in alcohol).
    pub fn defining_substance_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("definingSubstanceReference") {
            return Some(Reference {
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

    /// Parameters that were used in the measurement of a property (e.g. for viscosity:
    /// measured at 20C with a pH of 7.1).
    pub fn parameters(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("parameters") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._amount_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._parameters() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_string() {}
        if let Some(_val) = self.category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.defining_substance_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.defining_substance_reference() {
            if !_val.validate() {
                return false;
            }
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
        if let Some(_val) = self.parameters() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_PropertyBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_PropertyBuilder {
    pub fn build(&self) -> SubstanceSpecification_Property {
        SubstanceSpecification_Property {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: SubstanceSpecification_Property,
    ) -> SubstanceSpecification_PropertyBuilder {
        SubstanceSpecification_PropertyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_PropertyBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_PropertyBuilder { value: __value };
    }

    pub fn _amount_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["_amountString"] = json!(val.value);
        return self;
    }

    pub fn _parameters<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["_parameters"] = json!(val.value);
        return self;
    }

    pub fn amount_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["amountQuantity"] = json!(val.value);
        return self;
    }

    pub fn amount_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["amountString"] = json!(val);
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["category"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn defining_substance_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["definingSubstanceCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn defining_substance_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["definingSubstanceReference"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parameters<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSpecification_PropertyBuilder {
        self.value["parameters"] = json!(val);
        return self;
    }
}
