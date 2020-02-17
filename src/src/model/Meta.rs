#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The metadata about a resource. This is content in the resource that is
/// maintained by the infrastructure. Changes to the content might not always be
/// associated with version changes to the resource.

#[derive(Debug)]
pub struct Meta<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Meta<'_> {
    pub fn new(value: &Value) -> Meta {
        Meta {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for lastUpdated
    pub fn _last_updated(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastUpdated") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for source
    pub fn _source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_source") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for versionId
    pub fn _version_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_versionId") {
            return Some(Element {
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

    /// When the resource last changed - e.g. when the version changed.
    pub fn last_updated(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastUpdated") {
            return Some(string);
        }
        return None;
    }

    /// A list of profiles (references to [[[StructureDefinition]]] resources) that this
    /// resource claims to conform to. The URL is a reference to
    /// [[[StructureDefinition.url]]].
    pub fn profile(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("profile") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Security labels applied to this resource. These tags connect specific resources
    /// to the overall security policy and infrastructure.
    pub fn security(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("security") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A uri that identifies the source system of the resource. This provides a minimal
    /// amount of [[[Provenance]]] information that can be used to track or
    /// differentiate the source of information in the resource. The source may identify
    /// another FHIR server, document, message, database, etc.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
        }
        return None;
    }

    /// Tags applied to this resource. Tags are intended to be used to identify and
    /// relate resources to process and workflow, and applications are not required to
    /// consider the tags when interpreting the meaning of a resource.
    pub fn tag(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("tag") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The version specific identifier, as it appears in the version portion of the
    /// URL. This value changes when the resource is created, updated, or deleted.
    pub fn version_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("versionId") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._last_updated() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._source() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version_id() {
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
        if let Some(_val) = self.last_updated() {}
        if let Some(_val) = self.profile() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.security() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.tag() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version_id() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MetaBuilder {
    pub(crate) value: Value,
}

impl MetaBuilder {
    pub fn build(&self) -> Meta {
        Meta {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Meta) -> MetaBuilder {
        MetaBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MetaBuilder {
        let mut __value: Value = json!({});
        return MetaBuilder { value: __value };
    }

    pub fn _last_updated<'a>(&'a mut self, val: Element) -> &'a mut MetaBuilder {
        self.value["_lastUpdated"] = json!(val.value);
        return self;
    }

    pub fn _source<'a>(&'a mut self, val: Element) -> &'a mut MetaBuilder {
        self.value["_source"] = json!(val.value);
        return self;
    }

    pub fn _version_id<'a>(&'a mut self, val: Element) -> &'a mut MetaBuilder {
        self.value["_versionId"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MetaBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MetaBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn last_updated<'a>(&'a mut self, val: &str) -> &'a mut MetaBuilder {
        self.value["lastUpdated"] = json!(val);
        return self;
    }

    pub fn profile<'a>(&'a mut self, val: Vec<&str>) -> &'a mut MetaBuilder {
        self.value["profile"] = json!(val);
        return self;
    }

    pub fn security<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut MetaBuilder {
        self.value["security"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn source<'a>(&'a mut self, val: &str) -> &'a mut MetaBuilder {
        self.value["source"] = json!(val);
        return self;
    }

    pub fn tag<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut MetaBuilder {
        self.value["tag"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version_id<'a>(&'a mut self, val: &str) -> &'a mut MetaBuilder {
        self.value["versionId"] = json!(val);
        return self;
    }
}
