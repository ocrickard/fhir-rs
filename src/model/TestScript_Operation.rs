#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::TestScript_RequestHeader::TestScript_RequestHeader;
use serde_json::value::Value;

/// A structured set of tests against a FHIR server or client implementation to
/// determine compliance against the FHIR specification.

#[derive(Debug)]
pub struct TestScript_Operation<'a> {
    pub value: &'a Value,
}

impl TestScript_Operation<'_> {
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

    /// Extensions for resource
    pub fn _resource(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_resource") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The type of the resource.  See http://build.fhir.org/resourcelist.html.
    pub fn resource(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("resource") {
            return Some(string);
        }
        return None;
    }

    /// Server interaction or operation type.
    pub fn fhir_type(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("type") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Extensions for encodeRequestUrl
    pub fn _encode_request_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_encodeRequestUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The mime-type to use for RESTful operation in the 'Content-Type' header.
    pub fn content_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentType") {
            return Some(string);
        }
        return None;
    }

    /// The fixture id (maybe new) to map to the request.
    pub fn request_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requestId") {
            return Some(string);
        }
        return None;
    }

    /// The fixture id (maybe new) to map to the response.
    pub fn response_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("responseId") {
            return Some(string);
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

    /// Extensions for sourceId
    pub fn _source_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sourceId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The label would be used for tracking/logging purposes by test engines.
    pub fn label(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("label") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for origin
    pub fn _origin(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_origin") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for targetId
    pub fn _target_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_targetId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for params
    pub fn _params(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_params") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for label
    pub fn _label(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_label") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The mime-type to use for RESTful operation in the 'Accept' header.
    pub fn accept(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("accept") {
            return Some(string);
        }
        return None;
    }

    /// Path plus parameters after [type].  Used to set parts of the request URL
    /// explicitly.
    pub fn params(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("params") {
            return Some(string);
        }
        return None;
    }

    /// Complete request URL.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
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

    /// The server where the request message is destined for.  Must be one of the server
    /// numbers listed in TestScript.destination section.
    pub fn destination(&self) -> Option<i64> {
        if let Some(val) = self.value.get("destination") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for requestId
    pub fn _request_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requestId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for contentType
    pub fn _content_type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentType") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The server where the request message originates from.  Must be one of the server
    /// numbers listed in TestScript.origin section.
    pub fn origin(&self) -> Option<i64> {
        if let Some(val) = self.value.get("origin") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for accept
    pub fn _accept(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_accept") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The description would be used by test engines for tracking and reporting
    /// purposes.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for responseId
    pub fn _response_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_responseId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The HTTP method the test engine MUST use for this operation regardless of any
    /// other operation details.
    pub fn method(&self) -> Option<TestScript_OperationMethod> {
        if let Some(Value::String(val)) = self.value.get("method") {
            return Some(TestScript_OperationMethod::from_string(&val).unwrap());
        }
        return None;
    }

    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests.
    pub fn target_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("targetId") {
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

    /// Whether or not to implicitly send the request url in encoded format. The default
    /// is true to match the standard RESTful client behavior. Set to false when
    /// communicating with a server that does not support encoded url paths.
    pub fn encode_request_url(&self) -> Option<bool> {
        if let Some(val) = self.value.get("encodeRequestUrl") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for destination
    pub fn _destination(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_destination") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The id of the fixture used as the body of a PUT or POST request.
    pub fn source_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sourceId") {
            return Some(string);
        }
        return None;
    }

    /// Header elements would be used to set HTTP headers.
    pub fn request_header(&self) -> Option<Vec<TestScript_RequestHeader>> {
        if let Some(Value::Array(val)) = self.value.get("requestHeader") {
            return Some(
                val.into_iter()
                    .map(|e| TestScript_RequestHeader { value: e })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._resource() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.resource() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._encode_request_url() {
            _val.validate();
        }
        if let Some(_val) = self.content_type() {}
        if let Some(_val) = self.request_id() {}
        if let Some(_val) = self.response_id() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._source_id() {
            _val.validate();
        }
        if let Some(_val) = self.label() {}
        if let Some(_val) = self._origin() {
            _val.validate();
        }
        if let Some(_val) = self._target_id() {
            _val.validate();
        }
        if let Some(_val) = self._params() {
            _val.validate();
        }
        if let Some(_val) = self._label() {
            _val.validate();
        }
        if let Some(_val) = self.accept() {}
        if let Some(_val) = self.params() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self._method() {
            _val.validate();
        }
        if let Some(_val) = self.destination() {}
        if let Some(_val) = self._request_id() {
            _val.validate();
        }
        if let Some(_val) = self._content_type() {
            _val.validate();
        }
        if let Some(_val) = self.origin() {}
        if let Some(_val) = self._accept() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._response_id() {
            _val.validate();
        }
        if let Some(_val) = self.method() {}
        if let Some(_val) = self.target_id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.encode_request_url() {}
        if let Some(_val) = self._destination() {
            _val.validate();
        }
        if let Some(_val) = self.source_id() {}
        if let Some(_val) = self.request_header() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum TestScript_OperationMethod {
    Delete,
    Get,
    Options,
    Patch,
    Post,
    Put,
    Head,
}

impl TestScript_OperationMethod {
    pub fn from_string(string: &str) -> Option<TestScript_OperationMethod> {
        match string {
            "delete" => Some(TestScript_OperationMethod::Delete),
            "get" => Some(TestScript_OperationMethod::Get),
            "options" => Some(TestScript_OperationMethod::Options),
            "patch" => Some(TestScript_OperationMethod::Patch),
            "post" => Some(TestScript_OperationMethod::Post),
            "put" => Some(TestScript_OperationMethod::Put),
            "head" => Some(TestScript_OperationMethod::Head),
            _ => None,
        }
    }
}
