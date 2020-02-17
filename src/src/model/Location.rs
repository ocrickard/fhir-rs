#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Location_HoursOfOperation::Location_HoursOfOperation;
use crate::model::Location_Position::Location_Position;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Details and position information for a physical place where services are
/// provided and resources and participants may be stored, found, contained, or
/// accommodated.

#[derive(Debug)]
pub struct Location<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Location<'_> {
    pub fn new(value: &Value) -> Location {
        Location {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for alias
    pub fn _alias(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_alias") {
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

    /// Extensions for availabilityExceptions
    pub fn _availability_exceptions(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_availabilityExceptions") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for mode
    pub fn _mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mode") {
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Physical location.
    pub fn address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("address") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A list of alternate names that the location is known as, or was known as, in the
    /// past.
    pub fn alias(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("alias") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of when the locations opening ours are different to normal, e.g.
    /// public holiday availability. Succinctly describing all possible exceptions to
    /// normal site availability as detailed in the opening hours Times.
    pub fn availability_exceptions(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("availabilityExceptions") {
            return Some(string);
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

    /// Description of the Location, which helps in finding or referencing the place.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Technical endpoints providing access to services operated for the location.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
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

    /// What days/times during a week is this location usually open.
    pub fn hours_of_operation(&self) -> Option<Vec<Location_HoursOfOperation>> {
        if let Some(Value::Array(val)) = self.value.get("hoursOfOperation") {
            return Some(
                val.into_iter()
                    .map(|e| Location_HoursOfOperation {
                        value: Cow::Borrowed(e),
                    })
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

    /// Unique code or number identifying the location to its users.
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

    /// The organization responsible for the provisioning and upkeep of the location.
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

    /// Indicates whether a resource instance represents a specific location or a class
    /// of locations.
    pub fn mode(&self) -> Option<LocationMode> {
        if let Some(Value::String(val)) = self.value.get("mode") {
            return Some(LocationMode::from_string(&val).unwrap());
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

    /// Name of the location as used by humans. Does not need to be unique.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The operational status covers operation values most relevant to beds (but can
    /// also apply to rooms/units/chairs/etc. such as an isolation unit/dialysis chair).
    /// This typically covers concepts such as contamination, housekeeping, and other
    /// activities like maintenance.
    pub fn operational_status(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("operationalStatus") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Another Location of which this Location is physically a part of.
    pub fn part_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("partOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Physical form of the location, e.g. building, room, vehicle, road.
    pub fn physical_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("physicalType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The absolute geographic location of the Location, expressed using the WGS84
    /// datum (This is the same co-ordinate system used in KML).
    pub fn position(&self) -> Option<Location_Position> {
        if let Some(val) = self.value.get("position") {
            return Some(Location_Position {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status property covers the general availability of the resource, not the
    /// current value which may be covered by the operationStatus, or by a
    /// schedule/slots if they are configured for the location.
    pub fn status(&self) -> Option<LocationStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(LocationStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The contact details of communication devices available at the location. This can
    /// include phone numbers, fax numbers, mobile numbers, email addresses and web
    /// sites.
    pub fn telecom(&self) -> Option<Vec<ContactPoint>> {
        if let Some(Value::Array(val)) = self.value.get("telecom") {
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

    /// Indicates the type of function performed at the location.
    pub fn fhir_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._alias() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._availability_exceptions() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
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
        if let Some(_val) = self._mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.alias() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.availability_exceptions() {}
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.hours_of_operation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
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
        if let Some(_val) = self.mode() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.operational_status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.physical_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.position() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.telecom() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct LocationBuilder {
    pub(crate) value: Value,
}

impl LocationBuilder {
    pub fn build(&self) -> Location {
        Location {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Location) -> LocationBuilder {
        LocationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> LocationBuilder {
        let mut __value: Value = json!({});
        return LocationBuilder { value: __value };
    }

    pub fn _alias<'a>(&'a mut self, val: Vec<Element>) -> &'a mut LocationBuilder {
        self.value["_alias"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _availability_exceptions<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_availabilityExceptions"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _mode<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_mode"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut LocationBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn address<'a>(&'a mut self, val: Address) -> &'a mut LocationBuilder {
        self.value["address"] = json!(val.value);
        return self;
    }

    pub fn alias<'a>(&'a mut self, val: Vec<&str>) -> &'a mut LocationBuilder {
        self.value["alias"] = json!(val);
        return self;
    }

    pub fn availability_exceptions<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["availabilityExceptions"] = json!(val);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut LocationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut LocationBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut LocationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn hours_of_operation<'a>(
        &'a mut self,
        val: Vec<Location_HoursOfOperation>,
    ) -> &'a mut LocationBuilder {
        self.value["hoursOfOperation"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut LocationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn managing_organization<'a>(&'a mut self, val: Reference) -> &'a mut LocationBuilder {
        self.value["managingOrganization"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut LocationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn mode<'a>(&'a mut self, val: LocationMode) -> &'a mut LocationBuilder {
        self.value["mode"] = json!(val.to_string());
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut LocationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut LocationBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn operational_status<'a>(&'a mut self, val: Coding) -> &'a mut LocationBuilder {
        self.value["operationalStatus"] = json!(val.value);
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Reference) -> &'a mut LocationBuilder {
        self.value["partOf"] = json!(val.value);
        return self;
    }

    pub fn physical_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut LocationBuilder {
        self.value["physicalType"] = json!(val.value);
        return self;
    }

    pub fn position<'a>(&'a mut self, val: Location_Position) -> &'a mut LocationBuilder {
        self.value["position"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: LocationStatus) -> &'a mut LocationBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn telecom<'a>(&'a mut self, val: Vec<ContactPoint>) -> &'a mut LocationBuilder {
        self.value["telecom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut LocationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut LocationBuilder {
        self.value["type"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum LocationMode {
    Instance,
    Kind,
}

impl LocationMode {
    pub fn from_string(string: &str) -> Option<LocationMode> {
        match string {
            "instance" => Some(LocationMode::Instance),
            "kind" => Some(LocationMode::Kind),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LocationMode::Instance => "instance".to_string(),
            LocationMode::Kind => "kind".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum LocationStatus {
    Active,
    Suspended,
    Inactive,
}

impl LocationStatus {
    pub fn from_string(string: &str) -> Option<LocationStatus> {
        match string {
            "active" => Some(LocationStatus::Active),
            "suspended" => Some(LocationStatus::Suspended),
            "inactive" => Some(LocationStatus::Inactive),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            LocationStatus::Active => "active".to_string(),
            LocationStatus::Suspended => "suspended".to_string(),
            LocationStatus::Inactive => "inactive".to_string(),
        }
    }
}
