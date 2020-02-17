#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The technical details of an endpoint that can be used for electronic services,
/// such as for web services providing XDS.b or a REST endpoint for another FHIR
/// server. This may include any security context information.

#[derive(Debug)]
pub struct Endpoint<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Endpoint<'_> {
    pub fn new(value: &Value) -> Endpoint {
        Endpoint {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for address
    pub fn _address(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_address") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for header
    pub fn _header(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_header") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for payloadMimeType
    pub fn _payload_mime_type(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_payloadMimeType") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The uri that describes the actual end-point to connect to.
    pub fn address(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("address") {
            return Some(string);
        }
        return None;
    }

    /// A coded value that represents the technical details of the usage of this
    /// endpoint, such as what WSDLs should be used in what way. (e.g. XDS.b/DICOM/cds-
    /// hook).
    pub fn connection_type(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["connectionType"]),
        }
    }

    /// Contact details for a human to contact about the subscription. The primary use
    /// of this for system administrator troubleshooting.
    pub fn contact(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactPoint {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Additional headers / information to send as part of the notification.
    pub fn header(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("header") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
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

    /// Identifier for the organization that is used to identify the endpoint across
    /// multiple disparate systems.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The organization that manages this endpoint (even if technically another
    /// organization is hosting this in the cloud, it is the organization associated
    /// with the data).
    pub fn managing_organization(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("managingOrganization") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A friendly name that this endpoint can be referred to with.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The mime type to send the payload in - e.g. application/fhir+xml,
    /// application/fhir+json. If the mime type is not specified, then the sender could
    /// send any content (including no content depending on the connectionType).
    pub fn payload_mime_type(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("payloadMimeType") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The payload type describes the acceptable content that can be communicated on
    /// the endpoint.
    pub fn payload_type(&self) -> Vec<CodeableConcept> {
        self.value
            .get("payloadType")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| CodeableConcept {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
    }

    /// The interval during which the endpoint is expected to be operational.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// active | suspended | error | off | test.
    pub fn status(&self) -> Option<EndpointStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(EndpointStatus::from_string(&val).unwrap());
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._header() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._payload_mime_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.address() {}
        if !self.connection_type().validate() {
            return false;
        }
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.header() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.managing_organization() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.payload_mime_type() {
            _val.into_iter().for_each(|_e| {});
        }
        if !self
            .payload_type()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EndpointBuilder {
    pub(crate) value: Value,
}

impl EndpointBuilder {
    pub fn build(&self) -> Endpoint {
        Endpoint {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Endpoint) -> EndpointBuilder {
        EndpointBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(connection_type: Coding, payload_type: Vec<CodeableConcept>) -> EndpointBuilder {
        let mut __value: Value = json!({});
        __value["connectionType"] = json!(connection_type.value);
        __value["payloadType"] = json!(payload_type
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return EndpointBuilder { value: __value };
    }

    pub fn _address<'a>(&'a mut self, val: Element) -> &'a mut EndpointBuilder {
        self.value["_address"] = json!(val.value);
        return self;
    }

    pub fn _header<'a>(&'a mut self, val: Vec<Element>) -> &'a mut EndpointBuilder {
        self.value["_header"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut EndpointBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut EndpointBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut EndpointBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _payload_mime_type<'a>(&'a mut self, val: Vec<Element>) -> &'a mut EndpointBuilder {
        self.value["_payloadMimeType"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut EndpointBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: &str) -> &'a mut EndpointBuilder {
        self.value["address"] = json!(val);
        return self;
    }

    pub fn contact<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut EndpointBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut EndpointBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EndpointBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn header<'a>(&'a mut self, val: Vec<&str>) -> &'a mut EndpointBuilder {
        self.value["header"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut EndpointBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut EndpointBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut EndpointBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut EndpointBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn managing_organization<'a>(&'a mut self, val: Reference) -> &'a mut EndpointBuilder {
        self.value["managingOrganization"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut EndpointBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut EndpointBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut EndpointBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn payload_mime_type<'a>(&'a mut self, val: Vec<&str>) -> &'a mut EndpointBuilder {
        self.value["payloadMimeType"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut EndpointBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: EndpointStatus) -> &'a mut EndpointBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut EndpointBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum EndpointStatus {
    Active,
    Suspended,
    Error,
    Off,
    EnteredInError,
    Test,
}

impl EndpointStatus {
    pub fn from_string(string: &str) -> Option<EndpointStatus> {
        match string {
            "active" => Some(EndpointStatus::Active),
            "suspended" => Some(EndpointStatus::Suspended),
            "error" => Some(EndpointStatus::Error),
            "off" => Some(EndpointStatus::Off),
            "entered-in-error" => Some(EndpointStatus::EnteredInError),
            "test" => Some(EndpointStatus::Test),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EndpointStatus::Active => "active".to_string(),
            EndpointStatus::Suspended => "suspended".to_string(),
            EndpointStatus::Error => "error".to_string(),
            EndpointStatus::Off => "off".to_string(),
            EndpointStatus::EnteredInError => "entered-in-error".to_string(),
            EndpointStatus::Test => "test".to_string(),
        }
    }
}
