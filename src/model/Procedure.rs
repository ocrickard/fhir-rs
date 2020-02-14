#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Procedure_FocalDevice::Procedure_FocalDevice;
use crate::model::Procedure_Performer::Procedure_Performer;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// An action that is or was performed on or for a patient. This can be a physical
/// intervention like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.

#[derive(Debug)]
pub struct Procedure<'a> {
    pub value: &'a Value,
}

impl Procedure<'_> {
    /// The location where the procedure actually happened.  E.g. a newborn at home, a
    /// tracheostomy at a restaurant.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A code specifying the state of the procedure. Generally, this will be the in-
    /// progress or completed state.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// This could be a histology result, pathology report, surgical report, etc.
    pub fn report(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("report") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Any complications that occurred during the procedure, or in the immediate post-
    /// performance period.
    pub fn complication_detail(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("complicationDetail") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Any other notes and comments about the procedure.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The URL pointing to an externally maintained protocol, guideline, order set or
    /// other definition that is adhered to in whole or in part by this Procedure.
    pub fn instantiates_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Detailed and structured anatomical location information. Multiple locations are
    /// allowed - e.g. multiple punch biopsies of a lesion.
    pub fn body_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodySite") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed.  Allows a period to support complex procedures that span more than
    /// one date, and also allows for the length of the procedure to be captured.
    pub fn performed_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("performedPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Any complications that occurred during the procedure, or in the immediate post-
    /// performance period. These are generally tracked separately from the notes,
    /// which will typically describe the procedure itself rather than any 'post
    /// procedure' issues.
    pub fn complication(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("complication") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The justification of why the procedure was performed.
    pub fn reason_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("reasonReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// If the procedure required specific follow up - e.g. removal of sutures. The
    /// follow up may be represented as a simple note or could potentially be more
    /// complex, in which case the CarePlan resource can be used.
    pub fn follow_up(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("followUp") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Captures the reason for the current state of the procedure.
    pub fn status_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("statusReason") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A reference to a resource that contains details of the request for this
    /// procedure.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies medications, devices and any other substance used as part of the
    /// procedure.
    pub fn used_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("usedReference") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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
                    .map(|e| ResourceList { value: e })
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
            return Some(Narrative { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specific procedure that is performed. Use text if the exact nature of the
    /// procedure cannot be coded (e.g. "Laparoscopic Appendectomy").
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Identifies coded items that were used as part of the procedure.
    pub fn used_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("usedCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Business identifiers assigned to this procedure by the performer or other
    /// systems which remain constant as the resource is updated and is propagated from
    /// server to server.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The person, animal or group on which the procedure was performed.
    pub fn subject(&self) -> Reference {
        Reference {
            value: &self.value["subject"],
        }
    }

    /// A larger event of which this particular procedure is a component or step.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Individual who is making the procedure statement.
    pub fn asserter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("asserter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The outcome of the procedure - did it resolve the reasons for the procedure
    /// being performed?
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The Encounter during which this Procedure was created or performed or to which
    /// the creation of this record is tightly associated.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed.  Allows a period to support complex procedures that span more than
    /// one date, and also allows for the length of the procedure to be captured.
    pub fn performed_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("performedDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for performedDateTime
    pub fn _performed_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_performedDateTime") {
            return Some(Element { value: val });
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

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_instantiatesUri") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed.  Allows a period to support complex procedures that span more than
    /// one date, and also allows for the length of the procedure to be captured.
    pub fn performed_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("performedRange") {
            return Some(Range { value: val });
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

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed.  Allows a period to support complex procedures that span more than
    /// one date, and also allows for the length of the procedure to be captured.
    pub fn performed_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("performedString") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined protocol, guideline, order set or other
    /// definition that is adhered to in whole or in part by this Procedure.
    pub fn instantiates_canonical(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("instantiatesCanonical") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Estimated or actual date, date-time, period, or age when the procedure was
    /// performed.  Allows a period to support complex procedures that span more than
    /// one date, and also allows for the length of the procedure to be captured.
    pub fn performed_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("performedAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Limited to "real" people rather than equipment.
    pub fn performer(&self) -> Option<Vec<Procedure_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| Procedure_Performer { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The coded reason why the procedure was performed. This may be a coded entity of
    /// some type, or may simply be present as text.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A device that is implanted, removed or otherwise manipulated (calibration,
    /// battery replacement, fitting a prosthesis, attaching a wound-vac, etc.) as a
    /// focal portion of the Procedure.
    pub fn focal_device(&self) -> Option<Vec<Procedure_FocalDevice>> {
        if let Some(Value::Array(val)) = self.value.get("focalDevice") {
            return Some(
                val.into_iter()
                    .map(|e| Procedure_FocalDevice { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code that classifies the procedure for searching, sorting and display purposes
    /// (e.g. "Surgical Procedure").
    pub fn category(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("category") {
            return Some(CodeableConcept { value: val });
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

    /// Individual who recorded the record and takes responsibility for its content.
    pub fn recorder(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("recorder") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for performedString
    pub fn _performed_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_performedString") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.report() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.complication_detail() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.body_site() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performed_period() {
            _val.validate();
        }
        if let Some(_val) = self.complication() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.follow_up() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status_reason() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.used_reference() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.used_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.subject().validate();
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.asserter() {
            _val.validate();
        }
        if let Some(_val) = self.outcome() {
            _val.validate();
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.performed_date_time() {}
        if let Some(_val) = self._performed_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._instantiates_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.performed_range() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.performed_string() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.performed_age() {
            _val.validate();
        }
        if let Some(_val) = self.performer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.focal_device() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.category() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.recorder() {
            _val.validate();
        }
        if let Some(_val) = self._performed_string() {
            _val.validate();
        }
        return true;
    }
}
