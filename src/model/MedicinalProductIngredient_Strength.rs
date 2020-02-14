#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MedicinalProductIngredient_ReferenceStrength::MedicinalProductIngredient_ReferenceStrength;
use crate::model::Ratio::Ratio;
use serde_json::value::Value;

/// An ingredient of a manufactured item or pharmaceutical product.

#[derive(Debug)]
pub struct MedicinalProductIngredient_Strength<'a> {
    pub value: &'a Value,
}

impl MedicinalProductIngredient_Strength<'_> {
    /// A lower limit for the strength per unitary volume (or mass), for when there is a
    /// range. The concentration attribute then becomes the upper limit.
    pub fn concentration_low_limit(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("concentrationLowLimit") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// A lower limit for the quantity of substance in the unit of presentation. For use
    /// when there is a range of strengths, this is the lower limit, with the
    /// presentation attribute becoming the upper limit.
    pub fn presentation_low_limit(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("presentationLowLimit") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// The strength per unitary volume (or mass).
    pub fn concentration(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("concentration") {
            return Some(Ratio { value: val });
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

    /// Extensions for measurementPoint
    pub fn _measurement_point(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_measurementPoint") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Strength expressed in terms of a reference substance.
    pub fn reference_strength(&self) -> Option<Vec<MedicinalProductIngredient_ReferenceStrength>> {
        if let Some(Value::Array(val)) = self.value.get("referenceStrength") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductIngredient_ReferenceStrength { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The quantity of substance in the unit of presentation, or in the volume (or
    /// mass) of the single pharmaceutical product or manufactured item.
    pub fn presentation(&self) -> Ratio {
        Ratio {
            value: &self.value["presentation"],
        }
    }

    /// For when strength is measured at a particular point or distance.
    pub fn measurement_point(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("measurementPoint") {
            return Some(string);
        }
        return None;
    }

    /// The country or countries for which the strength range applies.
    pub fn country(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("country") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.concentration_low_limit() {
            _val.validate();
        }
        if let Some(_val) = self.presentation_low_limit() {
            _val.validate();
        }
        if let Some(_val) = self.concentration() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._measurement_point() {
            _val.validate();
        }
        if let Some(_val) = self.reference_strength() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.presentation().validate();
        if let Some(_val) = self.measurement_point() {}
        if let Some(_val) = self.country() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
