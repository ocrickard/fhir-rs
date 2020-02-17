#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MolecularSequence_Inner::MolecularSequence_Inner;
use crate::model::MolecularSequence_Outer::MolecularSequence_Outer;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_StructureVariant<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MolecularSequence_StructureVariant<'_> {
    pub fn new(value: &Value) -> MolecularSequence_StructureVariant {
        MolecularSequence_StructureVariant {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for exact
    pub fn _exact(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exact") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for length
    pub fn _length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_length") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Used to indicate if the outer and inner start-end values have the same meaning.
    pub fn exact(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exact") {
            return Some(val.as_bool().unwrap());
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

    /// Structural variant inner.
    pub fn inner(&self) -> Option<MolecularSequence_Inner> {
        if let Some(val) = self.value.get("inner") {
            return Some(MolecularSequence_Inner {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Length of the variant chromosome.
    pub fn length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("length") {
            return Some(val.as_i64().unwrap());
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

    /// Structural variant outer.
    pub fn outer(&self) -> Option<MolecularSequence_Outer> {
        if let Some(val) = self.value.get("outer") {
            return Some(MolecularSequence_Outer {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about chromosome structure variation DNA change type.
    pub fn variant_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("variantType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._exact() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._length() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exact() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.inner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.length() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.outer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.variant_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MolecularSequence_StructureVariantBuilder {
    pub(crate) value: Value,
}

impl MolecularSequence_StructureVariantBuilder {
    pub fn build(&self) -> MolecularSequence_StructureVariant {
        MolecularSequence_StructureVariant {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MolecularSequence_StructureVariant,
    ) -> MolecularSequence_StructureVariantBuilder {
        MolecularSequence_StructureVariantBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MolecularSequence_StructureVariantBuilder {
        let mut __value: Value = json!({});
        return MolecularSequence_StructureVariantBuilder { value: __value };
    }

    pub fn _exact<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["_exact"] = json!(val.value);
        return self;
    }

    pub fn _length<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["_length"] = json!(val.value);
        return self;
    }

    pub fn exact<'a>(&'a mut self, val: bool) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["exact"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn inner<'a>(
        &'a mut self,
        val: MolecularSequence_Inner,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["inner"] = json!(val.value);
        return self;
    }

    pub fn length<'a>(&'a mut self, val: i64) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["length"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outer<'a>(
        &'a mut self,
        val: MolecularSequence_Outer,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["outer"] = json!(val.value);
        return self;
    }

    pub fn variant_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MolecularSequence_StructureVariantBuilder {
        self.value["variantType"] = json!(val.value);
        return self;
    }
}
