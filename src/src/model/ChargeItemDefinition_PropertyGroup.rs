#![allow(unused_imports, non_camel_case_types)]

use crate::model::ChargeItemDefinition_Applicability::ChargeItemDefinition_Applicability;
use crate::model::ChargeItemDefinition_PriceComponent::ChargeItemDefinition_PriceComponent;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The ChargeItemDefinition resource provides the properties that apply to the
/// (billing) codes necessary to calculate costs and prices. The properties may
/// differ largely depending on type and realm, therefore this resource gives only a
/// rough structure and requires profiling for each type of billing code system.

#[derive(Debug)]
pub struct ChargeItemDefinition_PropertyGroup<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ChargeItemDefinition_PropertyGroup<'_> {
    pub fn new(value: &Value) -> ChargeItemDefinition_PropertyGroup {
        ChargeItemDefinition_PropertyGroup {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Expressions that describe applicability criteria for the priceComponent.
    pub fn applicability(&self) -> Option<Vec<ChargeItemDefinition_Applicability>> {
        if let Some(Value::Array(val)) = self.value.get("applicability") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItemDefinition_Applicability {
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
    /// offer transparency to the recipient of the Invoice of how the prices have been
    /// calculated.
    pub fn price_component(&self) -> Option<Vec<ChargeItemDefinition_PriceComponent>> {
        if let Some(Value::Array(val)) = self.value.get("priceComponent") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItemDefinition_PriceComponent {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.applicability() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct ChargeItemDefinition_PropertyGroupBuilder {
    pub(crate) value: Value,
}

impl ChargeItemDefinition_PropertyGroupBuilder {
    pub fn build(&self) -> ChargeItemDefinition_PropertyGroup {
        ChargeItemDefinition_PropertyGroup {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ChargeItemDefinition_PropertyGroup,
    ) -> ChargeItemDefinition_PropertyGroupBuilder {
        ChargeItemDefinition_PropertyGroupBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ChargeItemDefinition_PropertyGroupBuilder {
        let mut __value: Value = json!({});
        return ChargeItemDefinition_PropertyGroupBuilder { value: __value };
    }

    pub fn applicability<'a>(
        &'a mut self,
        val: Vec<ChargeItemDefinition_Applicability>,
    ) -> &'a mut ChargeItemDefinition_PropertyGroupBuilder {
        self.value["applicability"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ChargeItemDefinition_PropertyGroupBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ChargeItemDefinition_PropertyGroupBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ChargeItemDefinition_PropertyGroupBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn price_component<'a>(
        &'a mut self,
        val: Vec<ChargeItemDefinition_PriceComponent>,
    ) -> &'a mut ChargeItemDefinition_PropertyGroupBuilder {
        self.value["priceComponent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
