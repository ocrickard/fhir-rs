#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide_Resource1<'a> {
    pub value: &'a Value,
}

impl ImplementationGuide_Resource1<'_> {
    /// The relative path for primary page for this resource within the IG.
    pub fn relative_path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("relativePath") {
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

    /// Extensions for relativePath
    pub fn _relative_path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_relativePath") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If true or a reference, indicates the resource is an example instance.  If a
    /// reference is present, indicates that the example is an example of the specified
    /// profile.
    pub fn example_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exampleBoolean") {
            return Some(val.as_bool().unwrap());
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

    /// Where this resource is found.
    pub fn reference(&self) -> Reference {
        Reference {
            value: &self.value["reference"],
        }
    }

    /// Extensions for exampleBoolean
    pub fn _example_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exampleBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If true or a reference, indicates the resource is an example instance.  If a
    /// reference is present, indicates that the example is an example of the specified
    /// profile.
    pub fn example_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("exampleCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for exampleCanonical
    pub fn _example_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exampleCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.relative_path() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._relative_path() {
            _val.validate();
        }
        if let Some(_val) = self.example_boolean() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.reference().validate();
        if let Some(_val) = self._example_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.example_canonical() {}
        if let Some(_val) = self._example_canonical() {
            _val.validate();
        }
        return true;
    }
}
