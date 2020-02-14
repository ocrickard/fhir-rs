#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Response<'a> {
    pub value: &'a Value,
}

impl Bundle_Response<'_> {
    /// An OperationOutcome containing hints and warnings produced as part of processing
    /// this entry in a batch or transaction.
    pub fn outcome(&self) -> Option<ResourceList> {
        if let Some(val) = self.value.get("outcome") {
            return Some(ResourceList { value: val });
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

    /// The date/time that the resource was modified on the server.
    pub fn last_modified(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastModified") {
            return Some(string);
        }
        return None;
    }

    /// The status code returned by processing this entry. The status SHALL start with a
    /// 3 digit HTTP code (e.g. 404) and may contain the standard HTTP description
    /// associated with the status code.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
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

    /// The location header created by processing this operation, populated if the
    /// operation returns a location.
    pub fn location(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("location") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for location
    pub fn _location(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_location") {
            return Some(Element { value: val });
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

    /// Extensions for etag
    pub fn _etag(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_etag") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The Etag for the resource, if the operation for the entry produced a versioned
    /// resource (see [Resource Metadata and Versioning](http.html#versioning) and
    /// [Managing Resource Contention](http.html#concurrency)).
    pub fn etag(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("etag") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for lastModified
    pub fn _last_modified(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastModified") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.outcome() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.last_modified() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.location() {}
        if let Some(_val) = self._location() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._etag() {
            _val.validate();
        }
        if let Some(_val) = self.etag() {}
        if let Some(_val) = self._last_modified() {
            _val.validate();
        }
        return true;
    }
}
