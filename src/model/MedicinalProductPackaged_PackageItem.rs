#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::ProdCharacteristic::ProdCharacteristic;
use crate::model::ProductShelfLife::ProductShelfLife;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A medicinal product in a container or package.

#[derive(Debug)]
pub struct MedicinalProductPackaged_PackageItem<'a> {
    pub value: &'a Value,
}

impl MedicinalProductPackaged_PackageItem<'_> {
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

    /// A possible alternate material for the packaging.
    pub fn alternate_material(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("alternateMaterial") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A device accompanying a medicinal product.
    pub fn device(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("device") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// The manufactured item as contained in the packaged medicinal product.
    pub fn manufactured_item(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturedItem") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Allows containers within containers.
    pub fn package_item(&self) -> Option<Vec<MedicinalProductPackaged_PackageItem>> {
        if let Some(Value::Array(val)) = self.value.get("packageItem") {
            return Some(
                val.into_iter()
                    .map(|e| MedicinalProductPackaged_PackageItem { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Dimensions, color etc.
    pub fn physical_characteristics(&self) -> Option<ProdCharacteristic> {
        if let Some(val) = self.value.get("physicalCharacteristics") {
            return Some(ProdCharacteristic { value: val });
        }
        return None;
    }

    /// The quantity of this package in the medicinal product, at the current level of
    /// packaging. The outermost is always 1.
    pub fn quantity(&self) -> Quantity {
        Quantity {
            value: &self.value["quantity"],
        }
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

    /// Other codeable characteristics.
    pub fn other_characteristics(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("otherCharacteristics") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Material type of the package item.
    pub fn material(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("material") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The physical type of the container of the medicine.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    /// Manufacturer of this Package Item.
    pub fn manufacturer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("manufacturer") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Shelf Life and storage information.
    pub fn shelf_life_storage(&self) -> Option<Vec<ProductShelfLife>> {
        if let Some(Value::Array(val)) = self.value.get("shelfLifeStorage") {
            return Some(
                val.into_iter()
                    .map(|e| ProductShelfLife { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Including possibly Data Carrier Identifier.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.alternate_material() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.device() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.manufactured_item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.package_item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.physical_characteristics() {
            _val.validate();
        }
        let _ = self.quantity().validate();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.other_characteristics() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.material() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self.manufacturer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.shelf_life_storage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
