#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A material substance originating from a biological entity intended to be
/// transplanted or infused
/// into another (possibly the same) biological entity.

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_Collection<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl BiologicallyDerivedProduct_Collection<'_> {
    pub fn new(value: &Value) -> BiologicallyDerivedProduct_Collection {
        BiologicallyDerivedProduct_Collection {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for collectedDateTime
    pub fn _collected_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_collectedDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Time of product collection.
    pub fn collected_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("collectedDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Time of product collection.
    pub fn collected_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("collectedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Healthcare professional who is performing the collection.
    pub fn collector(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("collector") {
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

    /// The patient or entity, such as a hospital or vendor in the case of a
    /// processed/manipulated/manufactured product, providing the product.
    pub fn source(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("source") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._collected_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collected_date_time() {}
        if let Some(_val) = self.collected_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collector() {
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
        if let Some(_val) = self.source() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct BiologicallyDerivedProduct_CollectionBuilder {
    pub(crate) value: Value,
}

impl BiologicallyDerivedProduct_CollectionBuilder {
    pub fn build(&self) -> BiologicallyDerivedProduct_Collection {
        BiologicallyDerivedProduct_Collection {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: BiologicallyDerivedProduct_Collection,
    ) -> BiologicallyDerivedProduct_CollectionBuilder {
        BiologicallyDerivedProduct_CollectionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> BiologicallyDerivedProduct_CollectionBuilder {
        let mut __value: Value = json!({});
        return BiologicallyDerivedProduct_CollectionBuilder { value: __value };
    }

    pub fn _collected_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["_collectedDateTime"] = json!(val.value);
        return self;
    }

    pub fn collected_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["collectedDateTime"] = json!(val);
        return self;
    }

    pub fn collected_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["collectedPeriod"] = json!(val.value);
        return self;
    }

    pub fn collector<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["collector"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut BiologicallyDerivedProduct_CollectionBuilder {
        self.value["source"] = json!(val.value);
        return self;
    }
}
