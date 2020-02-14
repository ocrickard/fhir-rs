#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Request<'a> {
    pub value: &'a Value,
}

impl Bundle_Request<'_> {
    /// Instruct the server not to perform the create if a specified resource already
    /// exists. For further information, see the API documentation for ["Conditional
    /// Create"](http.html#ccreate). This is just the query portion of the URL - what
    /// follows the "?" (not including the "?").
    pub fn if_none_exist(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ifNoneExist") {
            return Some(string);
        }
        return None;
    }

    /// If the ETag values match, return a 304 Not Modified status. See the API
    /// documentation for ["Conditional Read"](http.html#cread).
    pub fn if_none_match(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ifNoneMatch") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for ifMatch
    pub fn _if_match(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifMatch") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The URL for this entry, relative to the root (the address to which the request
    /// is posted).
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
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

    /// Extensions for ifModifiedSince
    pub fn _if_modified_since(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifModifiedSince") {
            return Some(Element { value: val });
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

    /// In a transaction or batch, this is the HTTP action to be executed for this
    /// entry. In a history bundle, this indicates the HTTP action that occurred.
    pub fn method(&self) -> Option<Bundle_RequestMethod> {
        if let Some(Value::String(val)) = self.value.get("method") {
            return Some(Bundle_RequestMethod::from_string(&val).unwrap());
        }
        return None;
    }

    /// Only perform the operation if the last updated date matches. See the API
    /// documentation for ["Conditional Read"](http.html#cread).
    pub fn if_modified_since(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ifModifiedSince") {
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

    /// Extensions for method
    pub fn _method(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_method") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Only perform the operation if the Etag value matches. For more information, see
    /// the API section ["Managing Resource Contention"](http.html#concurrency).
    pub fn if_match(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ifMatch") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for ifNoneExist
    pub fn _if_none_exist(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifNoneExist") {
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

    /// Extensions for ifNoneMatch
    pub fn _if_none_match(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifNoneMatch") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.if_none_exist() {}
        if let Some(_val) = self.if_none_match() {}
        if let Some(_val) = self._if_match() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._if_modified_since() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.method() {}
        if let Some(_val) = self.if_modified_since() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._method() {
            _val.validate();
        }
        if let Some(_val) = self.if_match() {}
        if let Some(_val) = self._if_none_exist() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._if_none_match() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum Bundle_RequestMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Bundle_RequestMethod {
    pub fn from_string(string: &str) -> Option<Bundle_RequestMethod> {
        match string {
            "GET" => Some(Bundle_RequestMethod::GET),
            "HEAD" => Some(Bundle_RequestMethod::HEAD),
            "POST" => Some(Bundle_RequestMethod::POST),
            "PUT" => Some(Bundle_RequestMethod::PUT),
            "DELETE" => Some(Bundle_RequestMethod::DELETE),
            "PATCH" => Some(Bundle_RequestMethod::PATCH),
            _ => None,
        }
    }
}
