#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::SubstanceAmount_ReferenceRange::SubstanceAmount_ReferenceRange;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Chemical substances are a single substance type whose primary defining element
/// is the molecular structure. Chemical substances shall be defined on the basis of
/// their complete covalent molecular structure; the presence of a salt (counter-
/// ion) and/or solvates (water, alcohols) is also captured. Purity, grade,
/// physical form or particle size are not taken into account in the definition of a
/// chemical substance or in the assignment of a Substance ID.

#[derive(Debug)]
pub struct SubstanceAmount<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceAmount<'_> {
    pub fn new(value: &Value) -> SubstanceAmount {
        SubstanceAmount {
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

    /// Extensions for amountText
    pub fn _amount_text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_amountText") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used to capture quantitative values for a variety of elements. If only limits
    /// are given, the arithmetic mean would be the average. If only a single definite
    /// value for a given element is given, it would be captured in this field.
    pub fn amount_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("amountQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used to capture quantitative values for a variety of elements. If only limits
    /// are given, the arithmetic mean would be the average. If only a single definite
    /// value for a given element is given, it would be captured in this field.
    pub fn amount_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("amountRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used to capture quantitative values for a variety of elements. If only limits
    /// are given, the arithmetic mean would be the average. If only a single definite
    /// value for a given element is given, it would be captured in this field.
    pub fn amount_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountString") {
            return Some(string);
        }
        return None;
    }

    /// A textual comment on a numeric value.
    pub fn amount_text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("amountText") {
            return Some(string);
        }
        return None;
    }

    /// Most elements that require a quantitative value will also have a field called
    /// amount type. Amount type should always be specified because the actual value of
    /// the amount is often dependent on it. EXAMPLE: In capturing the actual relative
    /// amounts of substances or molecular fragments it is essential to indicate whether
    /// the amount refers to a mole ratio or weight ratio. For any given element an
    /// effort should be made to use same the amount type for all related definitional
    /// elements.
    pub fn amount_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("amountType") {
            return Some(CodeableConcept {
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

    /// Reference range of possible or expected values.
    pub fn reference_range(&self) -> Option<SubstanceAmount_ReferenceRange> {
        if let Some(val) = self.value.get("referenceRange") {
            return Some(SubstanceAmount_ReferenceRange {
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
        if let Some(_val) = self._amount_text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.amount_string() {}
        if let Some(_val) = self.amount_text() {}
        if let Some(_val) = self.amount_type() {
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
        if let Some(_val) = self.reference_range() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceAmountBuilder {
    pub(crate) value: Value,
}

impl SubstanceAmountBuilder {
    pub fn build(&self) -> SubstanceAmount {
        SubstanceAmount {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceAmount) -> SubstanceAmountBuilder {
        SubstanceAmountBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceAmountBuilder {
        let mut __value: Value = json!({});
        return SubstanceAmountBuilder { value: __value };
    }

    pub fn _amount_string<'a>(&'a mut self, val: Element) -> &'a mut SubstanceAmountBuilder {
        self.value["_amountString"] = json!(val.value);
        return self;
    }

    pub fn _amount_text<'a>(&'a mut self, val: Element) -> &'a mut SubstanceAmountBuilder {
        self.value["_amountText"] = json!(val.value);
        return self;
    }

    pub fn amount_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut SubstanceAmountBuilder {
        self.value["amountQuantity"] = json!(val.value);
        return self;
    }

    pub fn amount_range<'a>(&'a mut self, val: Range) -> &'a mut SubstanceAmountBuilder {
        self.value["amountRange"] = json!(val.value);
        return self;
    }

    pub fn amount_string<'a>(&'a mut self, val: &str) -> &'a mut SubstanceAmountBuilder {
        self.value["amountString"] = json!(val);
        return self;
    }

    pub fn amount_text<'a>(&'a mut self, val: &str) -> &'a mut SubstanceAmountBuilder {
        self.value["amountText"] = json!(val);
        return self;
    }

    pub fn amount_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SubstanceAmountBuilder {
        self.value["amountType"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SubstanceAmountBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceAmountBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceAmountBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reference_range<'a>(
        &'a mut self,
        val: SubstanceAmount_ReferenceRange,
    ) -> &'a mut SubstanceAmountBuilder {
        self.value["referenceRange"] = json!(val.value);
        return self;
    }
}
