#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A container for a collection of resources.

#[derive(Debug)]
pub struct Bundle_Request<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Bundle_Request<'_> {
    pub fn new(value: &Value) -> Bundle_Request {
        Bundle_Request {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for ifMatch
    pub fn _if_match(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifMatch") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for ifModifiedSince
    pub fn _if_modified_since(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifModifiedSince") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for ifNoneExist
    pub fn _if_none_exist(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifNoneExist") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for ifNoneMatch
    pub fn _if_none_match(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_ifNoneMatch") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for method
    pub fn _method(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_method") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
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

    /// Only perform the operation if the Etag value matches. For more information, see
    /// the API section ["Managing Resource Contention"](http.html#concurrency).
    pub fn if_match(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("ifMatch") {
            return Some(string);
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

    /// In a transaction or batch, this is the HTTP action to be executed for this
    /// entry. In a history bundle, this indicates the HTTP action that occurred.
    pub fn method(&self) -> Option<Bundle_RequestMethod> {
        if let Some(Value::String(val)) = self.value.get("method") {
            return Some(Bundle_RequestMethod::from_string(&val).unwrap());
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._if_match() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._if_modified_since() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._if_none_exist() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._if_none_match() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
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
        if let Some(_val) = self.if_match() {}
        if let Some(_val) = self.if_modified_since() {}
        if let Some(_val) = self.if_none_exist() {}
        if let Some(_val) = self.if_none_match() {}
        if let Some(_val) = self.method() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Bundle_RequestBuilder {
    pub(crate) value: Value,
}

impl Bundle_RequestBuilder {
    pub fn build(&self) -> Bundle_Request {
        Bundle_Request {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Bundle_Request) -> Bundle_RequestBuilder {
        Bundle_RequestBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Bundle_RequestBuilder {
        let mut __value: Value = json!({});
        return Bundle_RequestBuilder { value: __value };
    }

    pub fn _if_match<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_ifMatch"] = json!(val.value);
        return self;
    }

    pub fn _if_modified_since<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_ifModifiedSince"] = json!(val.value);
        return self;
    }

    pub fn _if_none_exist<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_ifNoneExist"] = json!(val.value);
        return self;
    }

    pub fn _if_none_match<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_ifNoneMatch"] = json!(val.value);
        return self;
    }

    pub fn _method<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_method"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut Bundle_RequestBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Bundle_RequestBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn if_match<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["ifMatch"] = json!(val);
        return self;
    }

    pub fn if_modified_since<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["ifModifiedSince"] = json!(val);
        return self;
    }

    pub fn if_none_exist<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["ifNoneExist"] = json!(val);
        return self;
    }

    pub fn if_none_match<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["ifNoneMatch"] = json!(val);
        return self;
    }

    pub fn method<'a>(&'a mut self, val: Bundle_RequestMethod) -> &'a mut Bundle_RequestBuilder {
        self.value["method"] = json!(val.to_string());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Bundle_RequestBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut Bundle_RequestBuilder {
        self.value["url"] = json!(val);
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            Bundle_RequestMethod::GET => "GET".to_string(),
            Bundle_RequestMethod::HEAD => "HEAD".to_string(),
            Bundle_RequestMethod::POST => "POST".to_string(),
            Bundle_RequestMethod::PUT => "PUT".to_string(),
            Bundle_RequestMethod::DELETE => "DELETE".to_string(),
            Bundle_RequestMethod::PATCH => "PATCH".to_string(),
        }
    }
}
