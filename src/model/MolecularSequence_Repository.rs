#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Repository<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_Repository<'_> {
    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for datasetId
    pub fn _dataset_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_datasetId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// URI of an external repository which contains further details about the genetics
    /// data.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Id of the variantset in this external repository. The server will understand how
    /// to use this id to call for more info about variantsets in external repository.
    pub fn variantset_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("variantsetId") {
            return Some(string);
        }
        return None;
    }

    /// URI of an external repository which contains further details about the genetics
    /// data.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Id of the read in this external repository.
    pub fn readset_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("readsetId") {
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

    /// Click and see / RESTful API / Need login to see / RESTful API with
    /// authentication / Other ways to see resource.
    pub fn fhir_type(&self) -> Option<MolecularSequence_RepositoryType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(MolecularSequence_RepositoryType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for readsetId
    pub fn _readset_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_readsetId") {
            return Some(Element { value: val });
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

    /// Extensions for variantsetId
    pub fn _variantset_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variantsetId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Id of the variant in this external repository. The server will understand how to
    /// use this id to call for more info about datasets in external repository.
    pub fn dataset_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("datasetId") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._dataset_id() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.variantset_id() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.readset_id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._readset_id() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._variantset_id() {
            _val.validate();
        }
        if let Some(_val) = self.dataset_id() {}
        return true;
    }
}

#[derive(Debug)]
pub enum MolecularSequence_RepositoryType {
    Directlink,
    Openapi,
    Login,
    Oauth,
    Other,
}

impl MolecularSequence_RepositoryType {
    pub fn from_string(string: &str) -> Option<MolecularSequence_RepositoryType> {
        match string {
            "directlink" => Some(MolecularSequence_RepositoryType::Directlink),
            "openapi" => Some(MolecularSequence_RepositoryType::Openapi),
            "login" => Some(MolecularSequence_RepositoryType::Login),
            "oauth" => Some(MolecularSequence_RepositoryType::Oauth),
            "other" => Some(MolecularSequence_RepositoryType::Other),
            _ => None,
        }
    }
}
