#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification_Moiety<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceSpecification_Moiety<'_> {
    pub fn new(value: &Value) -> SubstanceSpecification_Moiety {
        SubstanceSpecification_Moiety {
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

    /// Extensions for molecularFormula
    pub fn _molecular_formula(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_molecularFormula") {
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

    /// Quantitative value for this moiety.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Quantitative value for this moiety.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
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

    /// Identifier by which this moiety substance is known.
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

    /// Molecular formula.
    pub fn molecular_formula(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("molecularFormula") {
            return Some(string);
        }
        return None;
    }

    /// Textual name for this moiety substance.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Optical activity type.
    pub fn optical_activity(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("opticalActivity") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Role that the moiety is playing.
    pub fn role(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("role") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Stereochemistry type.
    pub fn stereochemistry(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("stereochemistry") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._amount_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._molecular_formula() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
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
        if let Some(_val) = self.molecular_formula() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.optical_activity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.role() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.stereochemistry() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceSpecification_MoietyBuilder {
    pub(crate) value: Value,
}

impl SubstanceSpecification_MoietyBuilder {
    pub fn build(&self) -> SubstanceSpecification_Moiety {
        SubstanceSpecification_Moiety {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceSpecification_Moiety) -> SubstanceSpecification_MoietyBuilder {
        SubstanceSpecification_MoietyBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceSpecification_MoietyBuilder {
        let mut __value: Value = json!({});
        return SubstanceSpecification_MoietyBuilder { value: __value };
    }

    pub fn _amount_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["_amountString"] = json!(val.value);
        return self;
    }

    pub fn _molecular_formula<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["_molecularFormula"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn amount_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["amountQuantity"] = json!(val.value);
        return self;
    }

    pub fn amount_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["amountString"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn molecular_formula<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["molecularFormula"] = json!(val);
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn optical_activity<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["opticalActivity"] = json!(val.value);
        return self;
    }

    pub fn role<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["role"] = json!(val.value);
        return self;
    }

    pub fn stereochemistry<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceSpecification_MoietyBuilder {
        self.value["stereochemistry"] = json!(val.value);
        return self;
    }
}
