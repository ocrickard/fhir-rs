#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::SpecimenDefinition_Additive::SpecimenDefinition_Additive;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A kind of specimen with associated set of requirements.

#[derive(Debug)]
pub struct SpecimenDefinition_Container<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SpecimenDefinition_Container<'_> {
    pub fn new(value: &Value) -> SpecimenDefinition_Container {
        SpecimenDefinition_Container {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for minimumVolumeString
    pub fn _minimum_volume_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minimumVolumeString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preparation
    pub fn _preparation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preparation") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Substance introduced in the kind of container to preserve, maintain or enhance
    /// the specimen. Examples: Formalin, Citrate, EDTA.
    pub fn additive(&self) -> Option<Vec<SpecimenDefinition_Additive>> {
        if let Some(Value::Array(val)) = self.value.get("additive") {
            return Some(
                val.into_iter()
                    .map(|e| SpecimenDefinition_Additive {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Color of container cap.
    pub fn cap(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("cap") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The capacity (volume or other measure) of this kind of container.
    pub fn capacity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("capacity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The textual description of the kind of container.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// The type of material of the container.
    pub fn material(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("material") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The minimum volume to be conditioned in the container.
    pub fn minimum_volume_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("minimumVolumeQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The minimum volume to be conditioned in the container.
    pub fn minimum_volume_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minimumVolumeString") {
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

    /// Special processing that should be applied to the container for this kind of
    /// specimen.
    pub fn preparation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preparation") {
            return Some(string);
        }
        return None;
    }

    /// The type of container used to contain this kind of specimen.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._minimum_volume_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._preparation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additive() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.cap() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.capacity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.material() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.minimum_volume_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.minimum_volume_string() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.preparation() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SpecimenDefinition_ContainerBuilder {
    pub(crate) value: Value,
}

impl SpecimenDefinition_ContainerBuilder {
    pub fn build(&self) -> SpecimenDefinition_Container {
        SpecimenDefinition_Container {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SpecimenDefinition_Container) -> SpecimenDefinition_ContainerBuilder {
        SpecimenDefinition_ContainerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SpecimenDefinition_ContainerBuilder {
        let mut __value: Value = json!({});
        return SpecimenDefinition_ContainerBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _minimum_volume_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["_minimumVolumeString"] = json!(val.value);
        return self;
    }

    pub fn _preparation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["_preparation"] = json!(val.value);
        return self;
    }

    pub fn additive<'a>(
        &'a mut self,
        val: Vec<SpecimenDefinition_Additive>,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["additive"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn cap<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["cap"] = json!(val.value);
        return self;
    }

    pub fn capacity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["capacity"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn material<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["material"] = json!(val.value);
        return self;
    }

    pub fn minimum_volume_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["minimumVolumeQuantity"] = json!(val.value);
        return self;
    }

    pub fn minimum_volume_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["minimumVolumeString"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn preparation<'a>(&'a mut self, val: &str) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["preparation"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SpecimenDefinition_ContainerBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
