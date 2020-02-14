#![allow(unused_imports, non_camel_case_types)]

use crate::model::CatalogEntry_RelatedEntry::CatalogEntry_RelatedEntry;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Catalog entries are wrappers that contextualize items included in a catalog.

#[derive(Debug)]
pub struct CatalogEntry<'a> {
    pub value: &'a Value,
}

impl CatalogEntry<'_> {
    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The item in a catalog or definition.
    pub fn referenced_item(&self) -> Reference {
        Reference {
            value: &self.value["referencedItem"],
        }
    }

    /// Extensions for validTo
    pub fn _valid_to(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_validTo") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Typically date of issue is different from the beginning of the validity. This
    /// can be used to see when an item was last updated.
    pub fn last_updated(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastUpdated") {
            return Some(string);
        }
        return None;
    }

    /// Used for examplefor Out of Formulary, or any specifics.
    pub fn additional_characteristic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("additionalCharacteristic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Used for example, to point to a substance, or to a device used to administer a
    /// medication.
    pub fn related_entry(&self) -> Option<Vec<CatalogEntry_RelatedEntry>> {
        if let Some(Value::Array(val)) = self.value.get("relatedEntry") {
            return Some(
                val.into_iter()
                    .map(|e| CatalogEntry_RelatedEntry { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// User for example for ATC classification, or.
    pub fn additional_classification(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("additionalClassification") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Classes of devices, or ATC for medication.
    pub fn classification(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("classification") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for lastUpdated
    pub fn _last_updated(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastUpdated") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// Whether the entry represents an orderable item.
    pub fn orderable(&self) -> Option<bool> {
        if let Some(val) = self.value.get("orderable") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The date until which this catalog entry is expected to be active.
    pub fn valid_to(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("validTo") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// Used in supporting related concepts, e.g. NDC to RxNorm.
    pub fn additional_identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("additionalIdentifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Used to support catalog exchange even for unsupported products, e.g. getting
    /// list of medications even if not prescribable.
    pub fn status(&self) -> Option<CatalogEntryStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(CatalogEntryStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The time period in which this catalog entry is expected to be active.
    pub fn validity_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("validityPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for orderable
    pub fn _orderable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_orderable") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The type of item - medication, device, service, protocol or other.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Used in supporting different identifiers for the same product, e.g. manufacturer
    /// code and retailer code.
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        let _ = self.referenced_item().validate();
        if let Some(_val) = self._valid_to() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.last_updated() {}
        if let Some(_val) = self.additional_characteristic() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.related_entry() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.additional_classification() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.classification() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._last_updated() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.orderable() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.valid_to() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.additional_identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.validity_period() {
            _val.validate();
        }
        if let Some(_val) = self._orderable() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum CatalogEntryStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl CatalogEntryStatus {
    pub fn from_string(string: &str) -> Option<CatalogEntryStatus> {
        match string {
            "draft" => Some(CatalogEntryStatus::Draft),
            "active" => Some(CatalogEntryStatus::Active),
            "retired" => Some(CatalogEntryStatus::Retired),
            "unknown" => Some(CatalogEntryStatus::Unknown),
            _ => None,
        }
    }
}
