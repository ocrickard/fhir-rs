#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Invoice_PriceComponent::Invoice_PriceComponent;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Invoice containing collected ChargeItems from an Account with calculated
/// individual and total price for Billing purpose.

#[derive(Debug)]
pub struct Invoice_LineItem<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Invoice_LineItem<'_> {
    pub fn new(value: &Value) -> Invoice_LineItem {
        Invoice_LineItem {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The ChargeItem contains information such as the billing code, date, amount etc.
    /// If no further details are required for the lineItem, inline billing codes can be
    /// added using the CodeableConcept data type instead of the Reference.
    pub fn charge_item_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("chargeItemCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The ChargeItem contains information such as the billing code, date, amount etc.
    /// If no further details are required for the lineItem, inline billing codes can be
    /// added using the CodeableConcept data type instead of the Reference.
    pub fn charge_item_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("chargeItemReference") {
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

    /// The price for a ChargeItem may be calculated as a base price with
    /// surcharges/deductions that apply in certain conditions. A ChargeItemDefinition
    /// resource that defines the prices, factors and conditions that apply to a billing
    /// code is currently under development. The priceComponent element can be used to
    /// offer transparency to the recipient of the Invoice as to how the prices have
    /// been calculated.
    pub fn price_component(&self) -> Option<Vec<Invoice_PriceComponent>> {
        if let Some(Value::Array(val)) = self.value.get("priceComponent") {
            return Some(
                val.into_iter()
                    .map(|e| Invoice_PriceComponent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Sequence in which the items appear on the invoice.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.charge_item_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.charge_item_reference() {
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
        if let Some(_val) = self.price_component() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Invoice_LineItemBuilder {
    pub(crate) value: Value,
}

impl Invoice_LineItemBuilder {
    pub fn build(&self) -> Invoice_LineItem {
        Invoice_LineItem {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Invoice_LineItem) -> Invoice_LineItemBuilder {
        Invoice_LineItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Invoice_LineItemBuilder {
        let mut __value: Value = json!({});
        return Invoice_LineItemBuilder { value: __value };
    }

    pub fn _sequence<'a>(&'a mut self, val: Element) -> &'a mut Invoice_LineItemBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn charge_item_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Invoice_LineItemBuilder {
        self.value["chargeItemCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn charge_item_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Invoice_LineItemBuilder {
        self.value["chargeItemReference"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Invoice_LineItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Invoice_LineItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Invoice_LineItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn price_component<'a>(
        &'a mut self,
        val: Vec<Invoice_PriceComponent>,
    ) -> &'a mut Invoice_LineItemBuilder {
        self.value["priceComponent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: i64) -> &'a mut Invoice_LineItemBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }
}
