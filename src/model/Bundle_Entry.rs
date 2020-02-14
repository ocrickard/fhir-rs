#![allow(unused_imports, non_camel_case_types)]

use crate::model::Bundle_Link::Bundle_Link;
use crate::model::Bundle_Request::Bundle_Request;
use crate::model::Bundle_Response::Bundle_Response;
use crate::model::Bundle_Search::Bundle_Search;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Entry<'a> {
    pub value: &'a Value,
}

impl Bundle_Entry<'_> {
    /// A series of links that provide context to this entry.
    pub fn link(&self) -> Option<Vec<Bundle_Link>> {
        if let Some(Value::Array(val)) = self.value.get("link") {
            return Some(
                val.into_iter()
                    .map(|e| Bundle_Link { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Absolute URL for the resource.  The fullUrl SHALL NOT disagree with the id
    /// in the resource - i.e. if the fullUrl is not a urn:uuid, the URL shall be
    /// version-independent URL consistent with the Resource.id. The fullUrl is a
    /// version independent reference to the resource. The fullUrl element SHALL have a
    /// value except that:   * fullUrl can be empty on a POST (although it does not need
    /// to when specifying a temporary id for reference in the bundle)  * Results from
    /// operations might involve resources that are not identified.
    pub fn full_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fullUrl") {
            return Some(string);
        }
        return None;
    }

    /// Additional information about how this entry should be processed as part of a
    /// transaction or batch.  For history, it shows how the entry was processed to
    /// create the version contained in the entry.
    pub fn request(&self) -> Option<Bundle_Request> {
        if let Some(val) = self.value.get("request") {
            return Some(Bundle_Request { value: val });
        }
        return None;
    }

    /// Indicates the results of processing the corresponding 'request' entry in the
    /// batch or transaction being responded to or what the results of an operation
    /// where when returning history.
    pub fn response(&self) -> Option<Bundle_Response> {
        if let Some(val) = self.value.get("response") {
            return Some(Bundle_Response { value: val });
        }
        return None;
    }

    /// The Resource for the entry. The purpose/meaning of the resource is determined by
    /// the Bundle.type.
    pub fn resource(&self) -> Option<ResourceList> {
        if let Some(val) = self.value.get("resource") {
            return Some(ResourceList { value: val });
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

    /// Extensions for fullUrl
    pub fn _full_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fullUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Information about the search process that lead to the creation of this entry.
    pub fn search(&self) -> Option<Bundle_Search> {
        if let Some(val) = self.value.get("search") {
            return Some(Bundle_Search { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.link() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.full_url() {}
        if let Some(_val) = self.request() {
            _val.validate();
        }
        if let Some(_val) = self.response() {
            _val.validate();
        }
        if let Some(_val) = self.resource() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._full_url() {
            _val.validate();
        }
        if let Some(_val) = self.search() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
