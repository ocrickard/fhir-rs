#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// The marketing status describes the date when a medicinal product is actually put
/// on the market or the date as of which it is no longer available.

#[derive(Debug)]
pub struct ProdCharacteristic<'a> {
    pub value: &'a Value,
}

impl ProdCharacteristic<'_> {
    /// Where applicable, the weight can be specified using a numerical value and its
    /// unit of measurement The unit of measurement shall be specified in accordance
    /// with ISO 11240 and the resulting terminology The symbol and the symbol
    /// identifier shall be used.
    pub fn weight(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("weight") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Where applicable, the external diameter can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified in
    /// accordance with ISO 11240 and the resulting terminology The symbol and the
    /// symbol identifier shall be used.
    pub fn external_diameter(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("externalDiameter") {
            return Some(Quantity { value: val });
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

    /// Extensions for imprint
    pub fn _imprint(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_imprint") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for color
    pub fn _color(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_color") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where applicable, the nominal volume can be specified using a numerical value
    /// and its unit of measurement The unit of measurement shall be specified in
    /// accordance with ISO 11240 and the resulting terminology The symbol and the
    /// symbol identifier shall be used.
    pub fn nominal_volume(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("nominalVolume") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Where applicable, the width can be specified using a numerical value and its
    /// unit of measurement The unit of measurement shall be specified in accordance
    /// with ISO 11240 and the resulting terminology The symbol and the symbol
    /// identifier shall be used.
    pub fn width(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("width") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for shape
    pub fn _shape(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_shape") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Where applicable, the imprint can be specified as text.
    pub fn imprint(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("imprint") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where applicable, the depth can be specified using a numerical value and its
    /// unit of measurement The unit of measurement shall be specified in accordance
    /// with ISO 11240 and the resulting terminology The symbol and the symbol
    /// identifier shall be used.
    pub fn depth(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("depth") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Where applicable, the image can be provided The format of the image attachment
    /// shall be specified by regional implementations.
    pub fn image(&self) -> Option<Vec<Attachment>> {
        if let Some(Value::Array(val)) = self.value.get("image") {
            return Some(
                val.into_iter()
                    .map(|e| Attachment { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where applicable, the scoring can be specified An appropriate controlled
    /// vocabulary shall be used The term and the term identifier shall be used.
    pub fn scoring(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scoring") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Where applicable, the shape can be specified An appropriate controlled
    /// vocabulary shall be used The term and the term identifier shall be used.
    pub fn shape(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("shape") {
            return Some(string);
        }
        return None;
    }

    /// Where applicable, the color can be specified An appropriate controlled
    /// vocabulary shall be used The term and the term identifier shall be used.
    pub fn color(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("color") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where applicable, the height can be specified using a numerical value and its
    /// unit of measurement The unit of measurement shall be specified in accordance
    /// with ISO 11240 and the resulting terminology The symbol and the symbol
    /// identifier shall be used.
    pub fn height(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("height") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.weight() {
            _val.validate();
        }
        if let Some(_val) = self.external_diameter() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._imprint() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._color() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.nominal_volume() {
            _val.validate();
        }
        if let Some(_val) = self.width() {
            _val.validate();
        }
        if let Some(_val) = self._shape() {
            _val.validate();
        }
        if let Some(_val) = self.imprint() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.depth() {
            _val.validate();
        }
        if let Some(_val) = self.image() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.scoring() {
            _val.validate();
        }
        if let Some(_val) = self.shape() {}
        if let Some(_val) = self.color() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.height() {
            _val.validate();
        }
        return true;
    }
}
