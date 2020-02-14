#![allow(unused_imports, non_camel_case_types)]

use crate::model::CapabilityStatement_Interaction::CapabilityStatement_Interaction;
use crate::model::CapabilityStatement_Operation::CapabilityStatement_Operation;
use crate::model::CapabilityStatement_SearchParam::CapabilityStatement_SearchParam;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A Capability Statement documents a set of capabilities (behaviors) of a FHIR
/// Server for a particular version of FHIR that may be used as a statement of
/// actual server functionality or a statement of required or desired server
/// implementation.

#[derive(Debug)]
pub struct CapabilityStatement_Resource<'a> {
    pub value: &'a Value,
}

impl CapabilityStatement_Resource<'_> {
    /// Additional information about the resource type used by the system.
    pub fn documentation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("documentation") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for documentation
    pub fn _documentation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_documentation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// This field is set to no-version to specify that the system does not support
    /// (server) or use (client) versioning for this resource type. If this has some
    /// other value, the server must at least correctly track and populate the versionId
    /// meta-property on resources. If the value is 'versioned-update', then the server
    /// supports all the versioning features, including using e-tags for version
    /// integrity in the API.
    pub fn versioning(&self) -> Option<CapabilityStatement_ResourceVersioning> {
        if let Some(Value::String(val)) = self.value.get("versioning") {
            return Some(CapabilityStatement_ResourceVersioning::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for updateCreate
    pub fn _update_create(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_updateCreate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for referencePolicy
    pub fn _reference_policy(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_referencePolicy") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A type of resource exposed via the restful interface.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// A list of profiles that represent different use cases supported by the system.
    /// For a server, "supported by the system" means the system hosts/produces a set of
    /// resources that are conformant to a particular profile, and allows clients that
    /// use its services to search using this profile and to find appropriate data. For
    /// a client, it means the system will search by this profile and process data
    /// according to the guidance implicit in the profile. See further discussion in
    /// [Using Profiles](profiling.html#profile-uses).
    pub fn supported_profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("supportedProfile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for conditionalRead
    pub fn _conditional_read(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conditionalRead") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that indicates how the server supports conditional delete.
    pub fn conditional_delete(&self) -> Option<CapabilityStatement_ResourceConditionalDelete> {
        if let Some(Value::String(val)) = self.value.get("conditionalDelete") {
            return Some(CapabilityStatement_ResourceConditionalDelete::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for searchInclude
    pub fn _search_include(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_searchInclude") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for searchRevInclude
    pub fn _search_rev_include(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_searchRevInclude") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Definition of an operation or a named query together with its parameters and
    /// their meaning and type. Consult the definition of the operation for details
    /// about how to invoke the operation, and the parameters.
    pub fn operation(&self) -> Option<Vec<CapabilityStatement_Operation>> {
        if let Some(Value::Array(val)) = self.value.get("operation") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Operation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for conditionalDelete
    pub fn _conditional_delete(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conditionalDelete") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A flag for whether the server is able to return past versions as part of the
    /// vRead operation.
    pub fn read_history(&self) -> Option<bool> {
        if let Some(val) = self.value.get("readHistory") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A list of _revinclude (reverse include) values supported by the server.
    pub fn search_rev_include(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("searchRevInclude") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for readHistory
    pub fn _read_history(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_readHistory") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies a restful operation supported by the solution.
    pub fn interaction(&self) -> Option<Vec<CapabilityStatement_Interaction>> {
        if let Some(Value::Array(val)) = self.value.get("interaction") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_Interaction { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// A flag that indicates that the server supports conditional create.
    pub fn conditional_create(&self) -> Option<bool> {
        if let Some(val) = self.value.get("conditionalCreate") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A specification of the profile that describes the solution's overall support for
    /// the resource, including any constraints on cardinality, bindings, lengths or
    /// other limitations. See further discussion in [Using
    /// Profiles](profiling.html#profile-uses).
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    /// Search parameters for implementations to support and/or make use of - either
    /// references to ones defined in the specification, or additional ones defined
    /// for/by the implementation.
    pub fn search_param(&self) -> Option<Vec<CapabilityStatement_SearchParam>> {
        if let Some(Value::Array(val)) = self.value.get("searchParam") {
            return Some(
                val.into_iter()
                    .map(|e| CapabilityStatement_SearchParam { value: e })
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for conditionalUpdate
    pub fn _conditional_update(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conditionalUpdate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A list of _include values supported by the server.
    pub fn search_include(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("searchInclude") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for conditionalCreate
    pub fn _conditional_create(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_conditionalCreate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A flag that indicates that the server supports conditional update.
    pub fn conditional_update(&self) -> Option<bool> {
        if let Some(val) = self.value.get("conditionalUpdate") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A code that indicates how the server supports conditional read.
    pub fn conditional_read(&self) -> Option<CapabilityStatement_ResourceConditionalRead> {
        if let Some(Value::String(val)) = self.value.get("conditionalRead") {
            return Some(CapabilityStatement_ResourceConditionalRead::from_string(&val).unwrap());
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

    /// A flag to indicate that the server allows or needs to allow the client to create
    /// new identities on the server (that is, the client PUTs to a location where there
    /// is no existing resource). Allowing this operation means that the server allows
    /// the client to create new identities on the server.
    pub fn update_create(&self) -> Option<bool> {
        if let Some(val) = self.value.get("updateCreate") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for versioning
    pub fn _versioning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versioning") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.documentation() {}
        if let Some(_val) = self._documentation() {
            _val.validate();
        }
        if let Some(_val) = self.versioning() {}
        if let Some(_val) = self._update_create() {
            _val.validate();
        }
        if let Some(_val) = self._reference_policy() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.supported_profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._conditional_read() {
            _val.validate();
        }
        if let Some(_val) = self.conditional_delete() {}
        if let Some(_val) = self._search_include() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._search_rev_include() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.operation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._conditional_delete() {
            _val.validate();
        }
        if let Some(_val) = self.read_history() {}
        if let Some(_val) = self.search_rev_include() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._read_history() {
            _val.validate();
        }
        if let Some(_val) = self.interaction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.conditional_create() {}
        if let Some(_val) = self.profile() {}
        if let Some(_val) = self.search_param() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._conditional_update() {
            _val.validate();
        }
        if let Some(_val) = self.search_include() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._conditional_create() {
            _val.validate();
        }
        if let Some(_val) = self.conditional_update() {}
        if let Some(_val) = self.conditional_read() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.update_create() {}
        if let Some(_val) = self._versioning() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_ResourceVersioning {
    NoVersion,
    Versioned,
    VersionedUpdate,
}

impl CapabilityStatement_ResourceVersioning {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_ResourceVersioning> {
        match string {
            "no-version" => Some(CapabilityStatement_ResourceVersioning::NoVersion),
            "versioned" => Some(CapabilityStatement_ResourceVersioning::Versioned),
            "versioned-update" => Some(CapabilityStatement_ResourceVersioning::VersionedUpdate),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_ResourceConditionalDelete {
    NotSupported,
    Single,
    Multiple,
}

impl CapabilityStatement_ResourceConditionalDelete {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_ResourceConditionalDelete> {
        match string {
            "not-supported" => Some(CapabilityStatement_ResourceConditionalDelete::NotSupported),
            "single" => Some(CapabilityStatement_ResourceConditionalDelete::Single),
            "multiple" => Some(CapabilityStatement_ResourceConditionalDelete::Multiple),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum CapabilityStatement_ResourceConditionalRead {
    NotSupported,
    ModifiedSince,
    NotMatch,
    FullSupport,
}

impl CapabilityStatement_ResourceConditionalRead {
    pub fn from_string(string: &str) -> Option<CapabilityStatement_ResourceConditionalRead> {
        match string {
            "not-supported" => Some(CapabilityStatement_ResourceConditionalRead::NotSupported),
            "modified-since" => Some(CapabilityStatement_ResourceConditionalRead::ModifiedSince),
            "not-match" => Some(CapabilityStatement_ResourceConditionalRead::NotMatch),
            "full-support" => Some(CapabilityStatement_ResourceConditionalRead::FullSupport),
            _ => None,
        }
    }
}
