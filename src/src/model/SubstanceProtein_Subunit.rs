#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence,
/// or a combination of subunits that are either covalently linked or have a defined
/// invariant stoichiometric relationship. This includes all synthetic, recombinant
/// and purified SubstanceProteins of defined sequence, whether the use is
/// therapeutic or prophylactic. This set of elements will be used to describe
/// albumins, coagulation factors, cytokines, growth factors,
/// peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant
/// vaccines, and immunomodulators.

#[derive(Debug)]
pub struct SubstanceProtein_Subunit<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceProtein_Subunit<'_> {
    pub fn new(value: &Value) -> SubstanceProtein_Subunit {
        SubstanceProtein_Subunit {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for cTerminalModification
    pub fn _c_terminal_modification(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cTerminalModification") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for length
    pub fn _length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_length") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for nTerminalModification
    pub fn _n_terminal_modification(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_nTerminalModification") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subunit
    pub fn _subunit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subunit") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The modification at the C-terminal shall be specified.
    pub fn c_terminal_modification(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("cTerminalModification") {
            return Some(string);
        }
        return None;
    }

    /// Unique identifier for molecular fragment modification based on the ISO 11238
    /// Substance ID.
    pub fn c_terminal_modification_id(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("cTerminalModificationId") {
            return Some(Identifier {
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

    /// Length of linear sequences of amino acids contained in the subunit.
    pub fn length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("length") {
            return Some(val.as_i64().unwrap());
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

    /// The name of the fragment modified at the N-terminal of the SubstanceProtein
    /// shall be specified.
    pub fn n_terminal_modification(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("nTerminalModification") {
            return Some(string);
        }
        return None;
    }

    /// Unique identifier for molecular fragment modification based on the ISO 11238
    /// Substance ID.
    pub fn n_terminal_modification_id(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("nTerminalModificationId") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The sequence information shall be provided enumerating the amino acids from N-
    /// to C-terminal end using standard single-letter amino acid codes. Uppercase shall
    /// be used for L-amino acids and lowercase for D-amino acids. Transcribed
    /// SubstanceProteins will always be described using the translated sequence; for
    /// synthetic peptide containing amino acids that are not represented with a single
    /// letter code an X should be used within the sequence. The modified amino acids
    /// will be distinguished by their position in the sequence.
    pub fn sequence(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sequence") {
            return Some(string);
        }
        return None;
    }

    /// The sequence information shall be provided enumerating the amino acids from N-
    /// to C-terminal end using standard single-letter amino acid codes. Uppercase shall
    /// be used for L-amino acids and lowercase for D-amino acids. Transcribed
    /// SubstanceProteins will always be described using the translated sequence; for
    /// synthetic peptide containing amino acids that are not represented with a single
    /// letter code an X should be used within the sequence. The modified amino acids
    /// will be distinguished by their position in the sequence.
    pub fn sequence_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("sequenceAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Index of primary sequences of amino acids linked through peptide bonds in order
    /// of decreasing length. Sequences of the same length will be ordered by molecular
    /// weight. Subunits that have identical sequences will be repeated and have
    /// sequential subscripts.
    pub fn subunit(&self) -> Option<i64> {
        if let Some(val) = self.value.get("subunit") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._c_terminal_modification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._length() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._n_terminal_modification() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subunit() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.c_terminal_modification() {}
        if let Some(_val) = self.c_terminal_modification_id() {
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
        if let Some(_val) = self.length() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.n_terminal_modification() {}
        if let Some(_val) = self.n_terminal_modification_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.sequence_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subunit() {}
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceProtein_SubunitBuilder {
    pub(crate) value: Value,
}

impl SubstanceProtein_SubunitBuilder {
    pub fn build(&self) -> SubstanceProtein_Subunit {
        SubstanceProtein_Subunit {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceProtein_Subunit) -> SubstanceProtein_SubunitBuilder {
        SubstanceProtein_SubunitBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceProtein_SubunitBuilder {
        let mut __value: Value = json!({});
        return SubstanceProtein_SubunitBuilder { value: __value };
    }

    pub fn _c_terminal_modification<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["_cTerminalModification"] = json!(val.value);
        return self;
    }

    pub fn _length<'a>(&'a mut self, val: Element) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["_length"] = json!(val.value);
        return self;
    }

    pub fn _n_terminal_modification<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["_nTerminalModification"] = json!(val.value);
        return self;
    }

    pub fn _sequence<'a>(&'a mut self, val: Element) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn _subunit<'a>(&'a mut self, val: Element) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["_subunit"] = json!(val.value);
        return self;
    }

    pub fn c_terminal_modification<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["cTerminalModification"] = json!(val);
        return self;
    }

    pub fn c_terminal_modification_id<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["cTerminalModificationId"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn length<'a>(&'a mut self, val: i64) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["length"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn n_terminal_modification<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["nTerminalModification"] = json!(val);
        return self;
    }

    pub fn n_terminal_modification_id<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["nTerminalModificationId"] = json!(val.value);
        return self;
    }

    pub fn sequence<'a>(&'a mut self, val: &str) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }

    pub fn sequence_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["sequenceAttachment"] = json!(val.value);
        return self;
    }

    pub fn subunit<'a>(&'a mut self, val: i64) -> &'a mut SubstanceProtein_SubunitBuilder {
        self.value["subunit"] = json!(val);
        return self;
    }
}
