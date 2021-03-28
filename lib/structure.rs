// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// SDMX-JSON Schema for structure messages
#[derive(Serialize, Deserialize)]
pub struct Structure {
    /// Data contains the message's “primary data”.
    pub data: Option<Data>,
    /// Errors field is an array of error objects. When appropriate provides a list of error
    /// messages in addition to RESTful web services HTTP error status codes.
    pub errors: Option<Vec<Error>>,
    /// A meta object that contains non-standard meta-information and basic technical information
    /// about the message, such as when it was prepared and who has sent it.
    pub meta: Option<Meta>,
}

/// Data contains the message's “primary data”.
#[derive(Serialize, Deserialize)]
pub struct Data {
    /// agencySchemes contains a collection of agency scheme descriptions.
    #[serde(rename = "agencySchemes")]
    pub agency_schemes: Option<Vec<AgencySchemeTypeElement>>,
    /// attachmentConstraints contains one or more attachment constraint, which can be explicitly
    /// detailed or referenced from an external structure document or registry service.
    #[serde(rename = "attachmentConstraints")]
    pub attachment_constraints: Option<Vec<AttachmentConstraintTypeElement>>,
    /// categorisations contains a collection of structural object categorisations. This
    /// container may contain categorisations for many types of objects. The categorisations may
    /// be detailed in full, or referenced from an external structure document or registry
    /// service.
    pub categorisations: Option<Vec<CategorisationTypeElement>>,
    /// categorySchemes contains a collection of category scheme descriptions. The category
    /// schemes may be detailed in full, or referenced from an external structure document or
    /// registry service.
    #[serde(rename = "categorySchemes")]
    pub category_schemes: Option<Vec<CategorySchemeType>>,
    /// codelists contains a collection of code list descriptions. The code lists may be detailed
    /// in full, or referenced from an external structure document or registry service.
    pub codelists: Option<Vec<CodelistType>>,
    /// conceptSchemes contains one or more concept schemes, which can be explicitly detailed or
    /// referenced from an external structure document or registry service.
    #[serde(rename = "conceptSchemes")]
    pub concept_schemes: Option<Vec<ConceptSchemeType>>,
    /// contentConstraints contains one or more content constraint, which can be explicitly
    /// detailed or referenced from an external structure document or registry service.
    #[serde(rename = "contentConstraints")]
    pub content_constraints: Option<Vec<ContentConstraintTypeElement>>,
    /// dataConsumerSchemes contains a collection of data consumer schemes descriptions.
    #[serde(rename = "dataConsumerSchemes")]
    pub data_consumer_schemes: Option<Vec<DataConsumerSchemeTypeElement>>,
    /// dataflows contains a collection of data flow descriptions. The data flows may be detailed
    /// in full, or referenced from an external structure document or registry service.
    pub dataflows: Option<Vec<DataflowTypeElement>>,
    /// dataProviderSchemes contains a collection of data provider schemes descriptions.
    #[serde(rename = "dataProviderSchemes")]
    pub data_provider_schemes: Option<Vec<DataProviderSchemeTypeElement>>,
    /// dataStructures contains a collection of data structure definitions. The data structure
    /// definitions may be detailed in full, or referenced from an external structure document or
    /// registry service.
    #[serde(rename = "dataStructures")]
    pub data_structures: Option<Vec<DataStructureTypeElement>>,
    /// hierarchicalCodelists contains a collection of hierarchical code list descriptions. The
    /// hierarchical code lists may be detailed in full, or referenced from an external structure
    /// document or registry service.
    #[serde(rename = "hierarchicalCodelists")]
    pub hierarchical_codelists: Option<Vec<HierarchicalCodelistTypeElement>>,
    /// metadataflows contains a collection of metadata flow descriptions. The metadata flows may
    /// be detailed in full, or referenced from an external structure document or registry
    /// service.
    pub metadataflows: Option<Vec<MetadataflowTypeElement>>,
    /// metadataStructures contains a collection of metadata structure definition descriptions.
    /// The metadata structure definitions may be detailed in full, or referenced from an
    /// external structure document or registry service.
    #[serde(rename = "metadataStructures")]
    pub metadata_structures: Option<Vec<MetadataStructureTypeElement>>,
    /// organisationUnitSchemes contains a collection of organisation unit schemes descriptions.
    #[serde(rename = "organisationUnitSchemes")]
    pub organisation_unit_schemes:
        Option<Vec<OrganisationUnitSchemeTypeElement>>,
    /// processes contains a collection of process descriptions. The processes may be detailed in
    /// full, or referenced from an external structure document or registry service.
    pub processes: Option<Vec<ProcessTypeElement>>,
    /// provisionAgreements contains a collection of provision agreements. The provision
    /// agreements may be detailed in full, or referenced from an external structure document or
    /// registry service.
    #[serde(rename = "provisionAgreements")]
    pub provision_agreements: Option<Vec<ProvisionAgreementTypeElement>>,
    /// reportingTaxonomies contains a collection of reporting taxonomy descriptions. The
    /// reporting taxonomies may be detailed in full, or referenced from an external structure
    /// document or registry service.
    #[serde(rename = "reportingTaxonomies")]
    pub reporting_taxonomies: Option<Vec<ReportingTaxonomyTypeElement>>,
    /// structureSets contains a collection of structure set descriptions. The structure sets may
    /// be detailed in full, or referenced from an external structure document or registry
    /// service.
    #[serde(rename = "structureSets")]
    pub structure_sets: Option<Vec<StructureSetTypeElement>>,
}

/// agencyScheme provides the details of an agency scheme, in which agencies are described.
///
/// AgencySchemeType defines a specific type of organisation scheme which contains only
/// maintenance agencies. The agency scheme maintained by a particular maintenance agency is
/// always provided a fixed identifier and version, and is never final. Therefore, agencies
/// can be added or removed without have to version the scheme. Agencies schemes have no
/// hierarchy, meaning that no agency may define a relationship with another agency in the
/// scheme. In fact, the actual parent agency for an agency in a scheme is the agency which
/// defines the scheme.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct AgencySchemeTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub agencies: Option<Vec<AgencyType>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// Agency is an organisation which maintains structural metadata such as statistical
/// classifications, glossaries, data structure definitions, and metadata structure
/// definitions..
///
/// AgencyType defines the structure of an agency description. The contacts defined for the
/// organisation are specific to the agency role the organisation is serving.
///
/// NameableType is an abstract base type for all nameable objects with NCNameID.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct AgencyType {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    pub contacts: Option<Vec<ContactType>>,
}

/// Annotations is a reusable element the provides for a collection of annotations. It has
/// been made global so that restrictions of types that extend AnnotatableType my reference
/// it.
///
/// AnnotationType provides for non-documentation notes and annotations to be embedded in
/// data and structure messages. It provides optional fields for providing a title, a type
/// description, a URI, and the text of the annotation.
#[derive(Serialize, Deserialize)]
pub struct AnnotationType {
    /// Non-standard identification of an annotation.
    pub id: Option<String>,
    /// Links field is an array of link objects. Also used to specify the Annotation URL which
    /// points to an external resource which may contain or supplement the annotation (using
    /// 'self' as relationship). If a specific behavior is desired, an annotation type should be
    /// defined which specifies the use of this field more exactly. If appropriate, a collection
    /// of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// AnnotationText holds a language-specific string containing the text of the annotation.
    pub text: Option<String>,
    /// AnnotationText holds a language-specific string containing the text of the annotation.
    pub texts: Option<HashMap<String, Option<serde_json::Value>>>,
    /// AnnotationTitle provides a title for the annotation.
    pub title: Option<String>,
    /// AnnotationType is used to distinguish between annotations designed to support various
    /// uses. The types are not enumerated, as these can be specified by the user or creator of
    /// the annotations. The definitions and use of annotation types should be documented by
    /// their creator.
    #[serde(rename = "type")]
    pub annotation_type_type: Option<String>,
}

/// Links field is an array of link objects. Also used to specify the Annotation URL which
/// points to an external resource which may contain or supplement the annotation (using
/// 'self' as relationship). If a specific behavior is desired, an annotation type should be
/// defined which specifies the use of this field more exactly. If appropriate, a collection
/// of links to additional external resources.
///
/// Links field is an array of link objects. If appropriate, a collection of links to
/// additional external resources for the header.
#[derive(Serialize, Deserialize)]
pub struct Link {
    pub href: Option<String>,
    /// The natural language of the external link, the same as used in the HTTP Accept-Language
    /// request header.
    pub hreflang: Option<String>,
    /// Relationship of the object to the external resource. See semantics below. Use 'self' to
    /// indicate the urn to the parent object.
    pub rel: String,
    pub title: Option<String>,
    pub titles: Option<HashMap<String, Option<serde_json::Value>>>,
    /// A hint about the type of representation returned by the link.
    #[serde(rename = "type")]
    pub link_type: Option<String>,
    /// The uri attribute holds a URI that contains a link to additional information about the
    /// resource, such as a web page. This uri is not an SDMX resource.
    pub uri: Option<String>,
    /// The urn holds a valid SDMX Registry URN (see SDMX Registry Specification for details).
    pub urn: Option<String>,
}

/// ContactType describes the structure of a contact's details.
#[derive(Serialize, Deserialize)]
pub struct ContactType {
    /// Department is a humain-readable designation of the organisational structure by a
    /// linguistic expression, within which the contact person works.
    pub department: Option<String>,
    /// Department is a humain-readable designation of the organisational structure by a
    /// linguistic expression, within which the contact person works.
    pub departments: Option<HashMap<String, Option<serde_json::Value>>>,
    pub emails: Option<Vec<String>>,
    pub faxes: Option<Vec<String>>,
    /// Name contains a humain-readable name for the contact.
    pub name: Option<String>,
    /// Name contains a humain-readable name for the contact.
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Role is the humain-readable responsibility of the contact person with respect to the
    /// object for which this person is the contact.
    pub role: Option<String>,
    /// Role is the humain-readable responsibility of the contact person with respect to the
    /// object for which this person is the contact.
    pub roles: Option<HashMap<String, Option<serde_json::Value>>>,
    pub telephones: Option<Vec<String>>,
    pub uris: Option<Vec<String>>,
    #[serde(rename = "x400s")]
    pub x400_s: Option<Vec<String>>,
}

/// attachmentConstraint describes sub sets of the content of a data or metadata set in terms
/// of the content regions or in terms of the set of key combinations to which attributes or
/// reference metadata (as defined by structure definitions) may be attached.
///
/// AttachmentConstraintType describes the details of an attachment constraint by defining
/// the data or metadata key sets or component regions that attributes or reference metadata
/// may be attached in the constraint attachment objects.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct AttachmentConstraintTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "constraintAttachment")]
    pub constraint_attachment: Option<AttachmentConstraintAttachmentType>,
    #[serde(rename = "dataKeySets")]
    pub data_key_sets: Option<Vec<DataKeySetType>>,
    #[serde(rename = "metadataKeySets")]
    pub metadata_key_sets: Option<Vec<MetadataKeySetType>>,
}

/// AttachmentConstraintAttachmentType defines the structure for specifying the object to
/// which an attachment constraints applies.
#[derive(Serialize, Deserialize)]
pub struct AttachmentConstraintAttachmentType {
    pub dataflows: Option<Vec<String>>,
    #[serde(rename = "dataSets")]
    pub data_sets: Option<Vec<SetReferenceType>>,
    #[serde(rename = "dataStructures")]
    pub data_structures: Option<Vec<String>>,
    pub metadataflows: Option<Vec<String>>,
    #[serde(rename = "metadataSets")]
    pub metadata_sets: Option<Vec<SetReferenceType>>,
    #[serde(rename = "metadataStructures")]
    pub metadata_structures: Option<Vec<String>>,
    #[serde(rename = "provisionAgreements")]
    pub provision_agreements: Option<Vec<String>>,
    #[serde(rename = "simpleDataSources")]
    pub simple_data_sources: Option<Vec<String>>,
}

/// DataSet is a urn reference to a data set to which the constraint is attached. Multiple
/// instance can only be used if they have the same underlying structure.
///
/// SetReferenceType defines the structure of a reference to a data/metadata set. A full
/// reference to a data provider and the identifier for the data set must be provided. Note
/// that this is not derived from the base reference structure since data/metadata sets are
/// not technically identifiable.
///
/// MetadataSet is a urn reference to a metadata set to which the constraint is attached.
/// Multiple instance can only be used if they have the same underlying structure.
///
/// dataSet is a urn reference to a data set to which the constraint is attached.
///
/// metadataSet is a urn reference to a metadata set to which the constraint is attached.
#[derive(Serialize, Deserialize)]
pub struct SetReferenceType {
    /// DataProvider is a urn reference to a the provider of the data/metadata set.
    #[serde(rename = "dataProvider")]
    pub data_provider: String,
    /// ID contains the identifier of the data/metadata set being referenced.
    pub id: String,
}

/// DataKeySetType defines a collection of full or partial data keys (dimension values).
///
/// DataKeySet defines a collection of full or partial data keys.
#[derive(Serialize, Deserialize)]
pub struct DataKeySetType {
    #[serde(rename = "isIncluded")]
    pub is_included: bool,
    pub keys: Vec<DataKeyType>,
}

/// Key contains a set of dimension values which identify a full set of data.
///
/// DataKeyType is a region which defines a distinct full or partial data key. The key
/// consists of a set of values, each referencing a dimension and providing a single value
/// for that dimension. The purpose of the key is to define a subset of a data set (i.e. the
/// observed value and data attribute) which have the dimension values provided in this
/// definition. Any dimension not stated explicitly in this key is assumed to be wild carded,
/// thus allowing for the definition of partial data keys.
#[derive(Serialize, Deserialize)]
pub struct DataKeyType {
    #[serde(rename = "keyValues")]
    pub key_values: Vec<DataKeyValueType>,
}

/// DataKeyValueType is a type for providing a dimension value for the purpose of defining a
/// distinct data key. Only a single value can be provided for the dimension.
#[derive(Serialize, Deserialize)]
pub struct DataKeyValueType {
    pub id: String,
    pub value: String,
}

/// MetadataKeySetType defines a collection of metadata keys (identifier component values).
///
/// MetadataKeySet defines a collection of metadata keys.
#[derive(Serialize, Deserialize)]
pub struct MetadataKeySetType {
    #[serde(rename = "isIncluded")]
    pub is_included: bool,
    pub keys: Vec<MetadataKeyType>,
}

/// Key contains a set of target object values for a specified report structure which serve
/// to identify which object reference metadata conforming to the specified report structure
/// is available for.
///
/// MetadataKeyType is a region which defines a distinct full or partial metadata key. The
/// key consists of a set of values, each referencing a target object for the metadata target
/// referenced in the metadataTarget attribute, which must be defined in the report structure
/// referenced in the report attribute. Each target object can be assigned a single value. If
/// an target object from the reference metadata target is not included in this key, the
/// value of that is assumed to be all known objects for a reference target object, all
/// possible keys for a key descriptor values target object, or all dates for report period
/// target object. The purpose of this key reference a metadata conforming to a particular
/// report structure for given object or set of objects.
#[derive(Serialize, Deserialize)]
pub struct MetadataKeyType {
    #[serde(rename = "keyValues")]
    pub key_values: Vec<MetadataKeyValueType>,
    #[serde(rename = "metadataTarget")]
    pub metadata_target: String,
    pub report: String,
}

/// MetadataKeyValueType is a type for providing a target object value for the purpose of
/// defining a distinct metadata key. Only a single value can be provided for the target
/// object.
#[derive(Serialize, Deserialize)]
pub struct MetadataKeyValueType {
    #[serde(rename = "dataKey")]
    pub data_key: Option<DataKeyType>,
    #[serde(rename = "dataSet")]
    pub data_set: Option<SetReferenceType>,
    pub id: String,
    /// Urn reference to any object. The type of object actually referenced can be determined
    /// from the URN.
    pub object: Option<String>,
    pub value: Option<String>,
}

/// categorisation allows for the association of an identifiable object to a category,
/// providing for the classifications of the reference identifiable object. This must either
/// contain the full details of the categorisation, or provide a name and identification
/// information and reference the full details from an external structure document or
/// registry service.
///
/// CategorisationType defines the structure for a categorisation. A source object is
/// referenced via an object reference and the target category is referenced via the target
/// category.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CategorisationTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// Source is a urn reference to an object to be categorized.
    pub source: Option<String>,
    /// Target is a urn reference to the category that the referenced object is to be mapped to.
    pub target: Option<String>,
}

/// categoryScheme provides the details of a category scheme, which is the descriptive
/// information for an arrangement or division of categories into groups based on
/// characteristics, which the objects have in common. This provides for a simple, leveled
/// hierarchy or categories.
///
/// CategorySchemeType describes the structure of a category scheme. A category scheme is the
/// descriptive information for an arrangement or division of categories into groups based on
/// characteristics, which the objects have in common. This provides for a simple, leveled
/// hierarchy or categories.
///
/// MaintainableType is an abstract base type for all maintainable objects with NCNameID.
///
/// NameableType is an abstract base type for all nameable objects with NCNameID.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CategorySchemeType {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub categories: Option<Vec<CategoryTypeElement>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// Category represents a set of nested categories which describe a simple classification
/// hierarchy.
///
/// CategoryType describes the details of a category. A category is defined as an item at any
/// level in a classification. The Category element represents a set of nested categories
/// which are child categories.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CategoryTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    pub categories: Option<Vec<CategoryTypeElement>>,
}

/// codelist provides the details of a code list, which is defined as a list from which some
/// statistical concepts (coded concepts) take their values.
///
/// CodelistType defines the structure of a codelist. A codelist is defined as a list from
/// which some statistical concepts (coded concepts) take their values.
///
/// MaintainableType is an abstract base type for all maintainable objects with NCNameID.
///
/// NameableType is an abstract base type for all nameable objects with NCNameID.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CodelistType {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub codes: Option<Vec<CodeTypeElement>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// Code describes a code in a codelist. In addition to the identification and description of
/// the code, basic presentational information is also available. Presentational information
/// not present may be added through the use of annotations.
///
/// CodeType describes the structure of a code. A code is defined as a language independent
/// set of letters, numbers or symbols that represent a concept whose meaning is described in
/// a natural language. Presentational information not present may be added through the use
/// of annotations.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CodeTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Parent provides the ability to describe simple hierarchies within a single codelist, by
    /// referencing the id value of another code in the same codelist.
    pub parent: Option<String>,
}

/// conceptScheme provides the details of a concept scheme, which is the descriptive
/// information for an arrangement or division of concepts into groups based on
/// characteristics, which the objects have in common. It contains a collection of concept
/// definitions, that may be arranged in simple hierarchies.
///
/// ConceptSchemeType describes the structure of a concept scheme. A concept scheme is the
/// descriptive information for an arrangement or division of concepts into groups based on
/// characteristics, which the objects have in common. It contains a collection of concept
/// definitions, that may be arranged in simple hierarchies.
///
/// MaintainableType is an abstract base type for all maintainable objects with NCNameID.
///
/// NameableType is an abstract base type for all nameable objects with NCNameID.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ConceptSchemeType {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub concepts: Option<Vec<ConceptType>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// Concept describes the details of a concept within a concept scheme.
///
/// ConceptType describes the details of a concept. A concept is defined as a unit of
/// knowledge created by a unique combination of characteristics. If a concept does not
/// specify a TextFormat or a core representation, then the representation of the concept is
/// assumed to be represented by any set of valid characters (corresponding to the string
/// datatype).
///
/// NameableType is an abstract base type for all nameable objects with NCNameID.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ConceptType {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "coreRepresentation")]
    pub core_representation: Option<ConceptRepresentation>,
    /// Provides a urn reference (containing conceptSchemeID, conceptAgency, conceptID) to an ISO
    /// 11179 concept.
    #[serde(rename = "isoConceptReference")]
    pub iso_concept_reference: Option<String>,
    /// Urn reference to a local concept. Parent captures the semantic relationships between
    /// concepts which occur within a single concept scheme. This identifies the concept of which
    /// the current concept is a qualification (in the ISO 11179 sense) or subclass.
    pub parent: Option<String>,
}

/// ConceptRepresentation defines the core representation that are allowed for a concept. The
/// text format allowed for a concept is that which is allowed for any non-target object
/// component.
#[derive(Serialize, Deserialize)]
pub struct ConceptRepresentation {
    /// Urn reference to a codelist which enumerates the possible values that can be used as the
    /// representation of this concept.
    pub enumeration: Option<String>,
    #[serde(rename = "enumerationFormat")]
    pub enumeration_format: Option<CodededTextFormatType>,
    #[serde(rename = "textFormat")]
    pub text_format: Option<BasicComponentTextFormatType>,
}

/// CodededTextFormatType is a restricted version of the SimpleComponentTextFormatType that
/// only allows factets and text types applicable to codes. Although the time facets permit
/// any value, an actual code identifier does not support the necessary characters for time.
/// Therefore these facets should not contain time in their values.
#[derive(Serialize, Deserialize)]
pub struct CodededTextFormatType {
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "endValue")]
    pub end_value: Option<i64>,
    pub interval: Option<i64>,
    #[serde(rename = "isSequence")]
    pub is_sequence: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<i64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<i64>,
    pub pattern: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "startValue")]
    pub start_value: Option<i64>,
    #[serde(rename = "textType")]
    pub text_type: Option<CodeDataType>,
    #[serde(rename = "timeInterval")]
    pub time_interval: Option<String>,
}

/// BasicComponentTextFormatType is a restricted version of the TextFormatType that restricts
/// the text type to the representations allowed for all components except for target objects.
#[derive(Serialize, Deserialize)]
pub struct BasicComponentTextFormatType {
    pub decimals: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "endValue")]
    pub end_value: Option<f64>,
    pub interval: Option<f64>,
    #[serde(rename = "isMultiLingual")]
    pub is_multi_lingual: Option<bool>,
    #[serde(rename = "isSequence")]
    pub is_sequence: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<f64>,
    pub pattern: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "startValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "textType")]
    pub text_type: Option<BasicComponentDataType>,
    #[serde(rename = "timeInterval")]
    pub time_interval: Option<String>,
}

/// contentConstraint specifies a sub set of the definition of the allowable or available
/// content of a data or metadata set in terms of the content or in terms of the set of key
/// combinations.
///
/// ContentConstraintType describes the details of a content constraint by defining the
/// content regions, key sets, or release information for the constraint attachment objects.
/// Note that if the constraint is for a data provider, then only release calendar
/// information is relevant, as there is no reliable way of determining which data structure
/// is being used to frame constraints in terms of cube regions or key sets.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ContentConstraintTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// ConstraintAttachment describes the collection of constrainable artefacts that the
    /// constraint is attached to.
    #[serde(rename = "constraintAttachment")]
    pub constraint_attachment: Option<ContentConstraintAttachmentType>,
    #[serde(rename = "cubeRegions")]
    pub cube_regions: Option<Vec<CubeRegionType>>,
    #[serde(rename = "dataKeySets")]
    pub data_key_sets: Option<Vec<DataKeySetType>>,
    #[serde(rename = "metadataKeySets")]
    pub metadata_key_sets: Option<Vec<MetadataKeySetType>>,
    #[serde(rename = "metadataTargetRegions")]
    pub metadata_target_regions: Option<Vec<MetadataTargetRegionType>>,
    /// ReferencePeriod is used to report start date and end date constraints.
    #[serde(rename = "referencePeriod")]
    pub reference_period: Option<ReferencePeriodType>,
    /// ReleaseCalendar defines dates on which the constrained data is to be made available.
    #[serde(rename = "releaseCalendar")]
    pub release_calendar: Option<ReleaseCalendarType>,
    #[serde(rename = "type")]
    pub type_type: Option<ContentConstraintTypeCodeType>,
}

/// ConstraintAttachment describes the collection of constrainable artefacts that the
/// constraint is attached to.
///
/// ContentConstraintAttachmentType defines the structure for specifying the target object(s)
/// of a content constraint.
#[derive(Serialize, Deserialize)]
pub struct ContentConstraintAttachmentType {
    pub dataflows: Option<Vec<String>>,
    /// dataProvider is a urn reference to a the provider of the data/metadata set to which the
    /// constraint is attached. If this is used, then only the release calendar is relevant..
    #[serde(rename = "dataProvider")]
    pub data_provider: Option<String>,
    /// dataSet is a urn reference to a data set to which the constraint is attached.
    #[serde(rename = "dataSet")]
    pub data_set: Option<SetReferenceType>,
    #[serde(rename = "dataStructures")]
    pub data_structures: Option<Vec<String>>,
    pub metadataflows: Option<Vec<String>>,
    /// metadataSet is a urn reference to a metadata set to which the constraint is attached.
    #[serde(rename = "metadataSet")]
    pub metadata_set: Option<SetReferenceType>,
    #[serde(rename = "metadataStructures")]
    pub metadata_structures: Option<Vec<String>>,
    #[serde(rename = "provisionAgreements")]
    pub provision_agreements: Option<Vec<String>>,
    #[serde(rename = "queryableDataSources")]
    pub queryable_data_sources: Option<Vec<QueryableDataSourceType>>,
    /// simpleDataSource describes a simple data source, which is a URL of a SDMX-ML data or
    /// metadata message.
    #[serde(rename = "simpleDataSource")]
    pub simple_data_source: Option<String>,
}

/// queryableDataSource describes a queryable data source to which the constraint is
/// attached.
///
/// QueryableDataSourceType describes a data source which accepts an standard SDMX Query
/// message and responds appropriately.
#[derive(Serialize, Deserialize)]
pub struct QueryableDataSourceType {
    /// DataURL contains the URL of the data source.
    #[serde(rename = "dataURL")]
    pub data_url: String,
    #[serde(rename = "isRESTDatasource")]
    pub is_rest_datasource: bool,
    #[serde(rename = "isWebServiceDatasource")]
    pub is_web_service_datasource: bool,
    /// WADLURL provides the location of a WADL instance on the internet which describes the REST
    /// protocol of the queryable data source.
    #[serde(rename = "wadlURL")]
    pub wadl_url: Option<String>,
    /// WSDLURL provides the location of a WSDL instance on the internet which describes the
    /// queryable data source.
    #[serde(rename = "wsdlURL")]
    pub wsdl_url: Option<String>,
}

/// CubeRegion describes a set of dimension values which define a region and attributes which
/// relate to the region for the purpose of describing a constraint.
///
/// CubeRegionType defines the structure of a data cube region. This is based on the abstract
/// RegionType and simply refines the key and attribute values to conform with what is
/// applicable for dimensions and attributes, respectively. See the documentation of the base
/// type for more details on how a region is defined.
#[derive(Serialize, Deserialize)]
pub struct CubeRegionType {
    pub attributes: Option<Vec<AttributeValueSetType>>,
    #[serde(rename = "isIncluded")]
    pub is_included: Option<bool>,
    #[serde(rename = "keyValues")]
    pub key_values: Option<Vec<CubeRegionKeyType>>,
}

/// AttributeValueSetType defines the structure for providing values for a data attribute. If
/// no values are provided, the attribute is implied to include/excluded from the region in
/// which it is defined, with no regard to the value of the data attribute. Note that for
/// metadata attributes which occur within other metadata attributes, a nested identifier can
/// be provided. For example, a value of CONTACT.ADDRESS.STREET refers to the metadata
/// attribute with the identifier STREET which exists in the ADDRESS metadata attribute in
/// the CONTACT metadata attribute, which is defined at the root of the report structure.
#[derive(Serialize, Deserialize)]
pub struct AttributeValueSetType {
    #[serde(rename = "cascadeValues")]
    pub cascade_values: Option<Vec<String>>,
    pub id: String,
    #[serde(rename = "timeRange")]
    pub time_range: Option<TimeRangeValueType>,
    pub values: Option<Vec<String>>,
}

/// TimeRangeValueType allows a time period value to be expressed as a range. It can be
/// expressed as the period before a period, after a period, or between two periods. Each of
/// these properties can specify their inclusion in regards to the range.
#[derive(Serialize, Deserialize)]
pub struct TimeRangeValueType {
    /// AfterPeriod is the period after which the period is meant to cover. This date may be
    /// inclusive or exclusive in the range.
    #[serde(rename = "afterPeriod")]
    pub after_period: Option<TimePeriodRangeType>,
    /// BeforePeriod is the period before which the period is meant to cover. This date may be
    /// inclusive or exclusive in the range.
    #[serde(rename = "beforePeriod")]
    pub before_period: Option<TimePeriodRangeType>,
    /// EndPeriod is the end period of the range. This date may be inclusive or exclusive in the
    /// range.
    #[serde(rename = "endPeriod")]
    pub end_period: Option<TimePeriodRangeType>,
    /// StartPeriod is the start date or the range that the queried date must occur within. This
    /// date may be inclusive or exclusive in the range.
    #[serde(rename = "startPeriod")]
    pub start_period: Option<TimePeriodRangeType>,
}

/// AfterPeriod is the period after which the period is meant to cover. This date may be
/// inclusive or exclusive in the range.
///
/// TimePeriodRangeType defines a time period, and indicates whether it is inclusive in a
/// range.
///
/// BeforePeriod is the period before which the period is meant to cover. This date may be
/// inclusive or exclusive in the range.
///
/// EndPeriod is the end period of the range. This date may be inclusive or exclusive in the
/// range.
///
/// StartPeriod is the start date or the range that the queried date must occur within. This
/// date may be inclusive or exclusive in the range.
#[derive(Serialize, Deserialize)]
pub struct TimePeriodRangeType {
    #[serde(rename = "isInclusive")]
    pub is_inclusive: Option<bool>,
    pub period: Option<String>,
}

/// CubeRegionKeyType is a type for providing a set of values for a dimension for the purpose
/// of defining a data cube region. A set of distinct value can be provided, or if this
/// dimension is represented as time, and time range can be specified.
#[derive(Serialize, Deserialize)]
pub struct CubeRegionKeyType {
    #[serde(rename = "cascadeValues")]
    pub cascade_values: Option<Vec<String>>,
    pub id: String,
    #[serde(rename = "timeRange")]
    pub time_range: Option<TimeRangeValueType>,
    pub values: Option<Vec<String>>,
}

/// MetadataTargetRegion describes a set of target object values for a given report structure
/// which define a region, and the metadata attribute which relate to the target for the
/// purpose of describing a constraint.
///
/// MetadataTargetRegionType defines the structure of a metadata target region. A metadata
/// target region must define the report structure and the metadata target from that
/// structure on which the region is based. This type is based on the abstract RegionType and
/// simply refines the key and attribute values to conform with what is applicable for target
/// objects and metadata attributes, respectively. See the documentation of the base type for
/// more details on how a region is defined.
#[derive(Serialize, Deserialize)]
pub struct MetadataTargetRegionType {
    pub attributes: Option<Vec<MetadataAttributeValueSetType>>,
    pub include: Option<bool>,
    #[serde(rename = "keyValues")]
    pub key_values: Option<Vec<MetadataTargetRegionKeyType>>,
    #[serde(rename = "metadataTarget")]
    pub metadata_target: String,
    pub report: String,
}

/// MetadataAttributeValueSetType defines the structure for providing values for a metadata
/// attribute. If no values are provided, the attribute is implied to include/excluded from
/// the region in which it is defined, with no regard to the value of the metadata attribute.
#[derive(Serialize, Deserialize)]
pub struct MetadataAttributeValueSetType {
    #[serde(rename = "cascadeValues")]
    pub cascade_values: Option<Vec<String>>,
    pub id: String,
    #[serde(rename = "timeRange")]
    pub time_range: Option<TimeRangeValueType>,
    pub values: Option<Vec<String>>,
}

/// MetadataTargetRegionKeyType is a type for providing a set of values for a target object
/// in a metadata target of a refence metadata report. A set of values or a time range can be
/// provided for a report period target object. A collection of the respective types of
/// references can be provided for data set reference and identifiable object reference
/// target objects. For a key descriptor values target object, a collection of data keys can
/// be provided.
#[derive(Serialize, Deserialize)]
pub struct MetadataTargetRegionKeyType {
    #[serde(rename = "dataKeys")]
    pub data_keys: Option<Vec<DataKeyType>>,
    #[serde(rename = "dataSets")]
    pub data_sets: Option<Vec<SetReferenceType>>,
    pub id: String,
    pub objects: Option<Vec<String>>,
    #[serde(rename = "timeRange")]
    pub time_range: Option<TimeRangeValueType>,
    pub values: Option<Vec<String>>,
}

/// ReferencePeriod is used to report start date and end date constraints.
///
/// Specifies the inclusive start and end times.
#[derive(Serialize, Deserialize)]
pub struct ReferencePeriodType {
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
}

/// ReleaseCalendar defines dates on which the constrained data is to be made available.
///
/// ReleaseCalendarType describes information about the timing of releases of the constrained
/// data. All of these values use the standard "P7D" - style format.
#[derive(Serialize, Deserialize)]
pub struct ReleaseCalendarType {
    /// Offset is the interval between January first and the first release of data within the
    /// year.
    pub offset: String,
    /// Periodicity is the period between releases of the data set.
    pub periodicity: String,
    /// Tolerance is the period after which the release of data may be deemed late.
    pub tolerance: String,
}

/// dataConsumerScheme provides the details of an data consumer scheme, in which data
/// consumers are described.
///
/// DataConsumerSchemeType defines a type of organisation scheme which contains only data
/// consumers. The data consumer scheme maintained by a particular maintenance agency is
/// always provided a fixed identifier and version, and is never final. Therefore, consumers
/// can be added or removed without have to version the scheme. This scheme has no hierarchy,
/// meaning that no organisation may define a relationship with another organisation in the
/// scheme.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataConsumerSchemeTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "dataConsumers")]
    pub data_consumers: Option<Vec<DataConsumerTypeElement>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// DataConsumer describes an organisation using data as input for further processing.
///
/// DataConsumerType defines the structure of a data consumer description. The contacts
/// defined for the organisation are specific to the data consumer role the organisation is
/// serving.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataConsumerTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    pub contacts: Option<Vec<ContactType>>,
}

/// dataProviderScheme provides the details of an data provider scheme, in which data
/// providers are described.
///
/// DataProviderSchemeType defines a type of organisation scheme which contains only data
/// providers. The data provider scheme maintained by a particular maintenance agency is
/// always provided a fixed identifier and version, and is never final. Therefore, providers
/// can be added or removed without have to version the scheme. This scheme has no hierarchy,
/// meaning that no organisation may define a relationship with another organisation in the
/// scheme.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataProviderSchemeTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "dataProviders")]
    pub data_providers: Option<Vec<DataProviderTypeElement>>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
}

/// DataProvider describes an organisation that produces data or reference metadata.
///
/// DataProviderType defines the structure of a data provider description. The contacts
/// defined for the organisation are specific to the data provider role the organisation is
/// serving.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataProviderTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    pub contacts: Option<Vec<ContactType>>,
}

/// DataStructure provides the details of a data structure definition, which is defined as a
/// collection of metadata concepts, their structure and usage when used to collect or
/// disseminate data.
///
/// DataStructureType describes the structure of a data structure definition. A data
/// structure definition is defined as a collection of metadata concepts, their structure and
/// usage when used to collect or disseminate data.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataStructureTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "dataStructureComponents")]
    pub data_structure_components: Option<DataStructureComponentsType>,
}

/// DataStructureComponents defines the grouping of the sets of metadata concepts that have a
/// defined structural role in the data structure definition. Note that for any component or
/// group defined in a data structure definition, its id must be unique. This applies to the
/// identifiers explicitly defined by the components as well as those inherited from the
/// concept identity of a component. For example, if two dimensions take their identity from
/// concepts with same identity (regardless of whether the concepts exist in different
/// schemes) one of the dimensions must be provided a different explicit identifier. Although
/// there are XML schema constraints to help enforce this, these only apply to explicitly
/// assigned identifiers. Identifiers inherited from a concept from which a component takes
/// its identity cannot be validated against this constraint. Therefore, systems processing
/// data structure definitions will have to perform this check outside of the XML validation.
/// There are also three reserved identifiers in a data structure definition; OBS_VALUE,
/// TIME_PERIOD, and REPORTING_PERIOD_START_DAY. These identifiers may not be used outside of
/// their respective defintions (PrimaryMeasure, TimeDimension, and ReportingYearStartDay).
/// This applies to both the explicit identifiers that can be assigned to the components or
/// groups as well as an identifier inherited by a component from its concept identity. For
/// example, if an ordinary dimension (i.e. not the time dimension) takes its concept
/// identity from a concept with the identifier TIME_PERIOD, that dimension must provide a
/// different explicit identifier.
///
/// DataStructureComponentsType describes the structure of the grouping to the sets of
/// metadata concepts that have a defined structural role in the data structure definition.
/// At a minimum at least one dimension and a primary measure must be defined.
#[derive(Serialize, Deserialize)]
pub struct DataStructureComponentsType {
    #[serde(rename = "attributeList")]
    pub attribute_list: Option<AttributeListTypeClass>,
    #[serde(rename = "dimensionList")]
    pub dimension_list: DimensionListTypeClass,
    pub groups: Option<Vec<GroupTypeElement>>,
    #[serde(rename = "measureList")]
    pub measure_list: MeasureListTypeClass,
}

/// AttributeList describes the attribute descriptor for the data structure definition. It is
/// a collection of metadata concepts that define the attributes of the data structure
/// definition.
///
/// AttributeListType describes the attribute descriptor for the data structure definition.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct AttributeListTypeClass {
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub attributes: Option<Vec<AttributeType>>,
    #[serde(rename = "reportingYearStartDays")]
    pub reporting_year_start_days: Option<Vec<ReportingYearStartDayType>>,
}

/// Attribute describes the definition of a data attribute, which is defined as a
/// characteristic of an object or entity.
///
/// AttributeType describes the structure of a data attribute, which is defined as a
/// characteristic of an object or entity. The attribute takes its semantic, and in some
/// cases it representation, from its concept identity. An attribute can be coded by
/// referencing a code list from its coded local representation. It can also specify its text
/// format, which is used as the representation of the attribute if a coded representation is
/// not defined. Neither the coded or uncoded representation are necessary, since the
/// attribute may take these from the referenced concept. An attribute specifies its
/// relationship with other data structure components and is given an assignment status.
/// These two properties dictate where in a data message the attribute will be attached, and
/// whether or not the attribute will be required to be given a value. A set of roles defined
/// in concept scheme can be assigned to the attribute.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct AttributeType {
    #[serde(rename = "assignmentStatus")]
    pub assignment_status: UsageStatusType,
    /// AttributeRelationship describes how the value of this attribute varies with the values of
    /// other components. These relationships will be used to determine the attachment level of
    /// the attribute in the various data formats.
    #[serde(rename = "attributeRelationship")]
    pub attribute_relationship: AttributeRelationshipType,
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    #[serde(rename = "conceptRoles")]
    pub concept_roles: Option<Vec<String>>,
    #[serde(rename = "localRepresentation")]
    pub local_representation: Option<SimpleDataStructureRepresentationType>,
}

/// AttributeRelationship describes how the value of this attribute varies with the values of
/// other components. These relationships will be used to determine the attachment level of
/// the attribute in the various data formats.
///
/// AttributeRelationshipType defines the structure for stating the relationship between an
/// attribute and other data structure definition components.
#[derive(Serialize, Deserialize)]
pub struct AttributeRelationshipType {
    #[serde(rename = "attachmentGroups")]
    pub attachment_groups: Option<Vec<String>>,
    pub dimensions: Option<Vec<String>>,
    /// Urn reference to a local GroupKey Descriptor. This is used as a convenience to
    /// referencing all of the dimension defined by the referenced group. The attribute will also
    /// be attached to this group.
    pub group: Option<String>,
    /// This means that value of the attribute will not vary with any of the other data structure
    /// components. This will always be treated as a data set level attribute.
    pub none: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Urn reference to a primary measure locally, where the reference to the data structure
    /// definition which defines the primary measure is provided in another context (for example
    /// the data structure definition in which the reference occurs). This is used to specify
    /// that the value of the attribute is dependent upon the observed value. An attribute with
    /// this relationship will always be treated as an observation level attribute.
    #[serde(rename = "primaryMeasure")]
    pub primary_measure: Option<String>,
}

/// SimpleDataStructureRepresentationType defines the representation for any non-measure and
/// non-time dimension data structure definition component.
#[derive(Serialize, Deserialize)]
pub struct SimpleDataStructureRepresentationType {
    /// Urn reference to a codelist
    pub enumeration: Option<String>,
    #[serde(rename = "enumerationFormat")]
    pub enumeration_format: Option<CodededTextFormatType>,
    #[serde(rename = "textFormat")]
    pub text_format: Option<SimpleComponentTextFormatType>,
}

/// SimpleComponentTextFormatType is a restricted version of the BasicComponentTextFormatType
/// that does not allow for multi-lingual values.
#[derive(Serialize, Deserialize)]
pub struct SimpleComponentTextFormatType {
    pub decimals: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "endValue")]
    pub end_value: Option<f64>,
    pub interval: Option<f64>,
    #[serde(rename = "isSequence")]
    pub is_sequence: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<f64>,
    pub pattern: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "startValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "textType")]
    pub text_type: Option<SimpleDataType>,
    #[serde(rename = "timeInterval")]
    pub time_interval: Option<String>,
}

/// ReportingYearStartDay is a specialized data attribute which provides important context to
/// the time dimension. If the value of the time dimension is one of the standard reporting
/// periods (see common:ReportingTimePeriodType) then this attribute is used to state the
/// month and day that the reporting year begins. This provides a reference point from which
/// the actual calendar dates covered by these periods can be determined. If this attribute
/// does not occur in a data set, then the reporting year start day will be assumed to be
/// January 1.
///
/// ReportingYearStartDayType defines the structure of the reporting year start day
/// attribute. The reporting year start day attribute takes its semantic from its concept
/// identity (usually the REPORTING_YEAR_START_DAY concept), yet is always has a fixed
/// identifier (REPORTING_YEAR_START_DAY). The reporting year start day attribute always has
/// a fixed text format, which specifies that the format of its value is always a day and
/// month in the ISO 8601 format of '--MM-DD'. As with any other attribute, an attribute
/// relationship must be specified. this relationship should be carefully selected as it will
/// determin what type of data the data structure definition will allow. For example, if an
/// attribute relationship of none is specified, this will mean the data sets conforming to
/// this data structure definition can only contain data with standard reporting periods
/// where the all reporting periods have the same start day. In this case, data reported as
/// standard reporting periods from two entities with different fiscal year start days could
/// not be contained in the same data set.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportingYearStartDayType {
    #[serde(rename = "assignmentStatus")]
    pub assignment_status: UsageStatusType,
    #[serde(rename = "attributeRelationship")]
    pub attribute_relationship: AttributeRelationshipType,
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    #[serde(rename = "localRepresentation")]
    pub local_representation: ReportingYearStartDayRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// ReportingYearStartDayRepresentationType defines the representation for the reporting year
/// start day attribute. Enumerated values are not allowed and the text format is fixed to be
/// a day and month in the ISO 8601 format of '--MM-DD'.
#[derive(Serialize, Deserialize)]
pub struct ReportingYearStartDayRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: ReportingYearStartDayTextFormatType,
}

/// ReportingYearStartDayTextFormatType is a restricted version of the
/// NonFacetedTextFormatType that fixes the value of the text type to be DayMonth. This type
/// exists solely for the purpose of fixing the representation of the reporting year start
/// day attribute.
#[derive(Serialize, Deserialize)]
pub struct ReportingYearStartDayTextFormatType {
    #[serde(rename = "textType")]
    pub text_type: Option<SimpleDataType>,
}

/// DimensionList describes the key descriptor for the data structure definition. It is an
/// ordered set of metadata concepts that, combined, classify a statistical series, such as a
/// time series, and whose values, when combined (the key) in an instance such as a data set,
/// uniquely identify a specific series.
///
/// DimensionListType describes the key descriptor for a data structure definition. The order
/// of the declaration of child dimensions is significant: it is used to describe the order
/// in which they will appear in data formats for which key values are supplied in an ordered
/// fashion (exclusive of the time dimension, which is not represented as a member of the
/// ordered key). Any data structure definition which uses the time dimension should also
/// declare a frequency dimension, conventionally the first dimension in the key (the set of
/// ordered non-time dimensions). If is not necessary to assign a time dimension, as data can
/// be organised in any fashion required.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DimensionListTypeClass {
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub dimensions: Option<Vec<DimensionType>>,
    #[serde(rename = "measureDimensions")]
    pub measure_dimensions: Option<Vec<MeasureDimensionType>>,
    #[serde(rename = "timeDimensions")]
    pub time_dimensions: Option<Vec<TimeDimensionType>>,
}

/// Dimension describes the structure of a dimension, which is defined as a statistical
/// concept used (most probably together with other statistical concepts) to identify a
/// statistical series, such as a time series, e.g. a statistical concept indicating certain
/// economic activity or a geographical reference area.
///
/// DimensionType describes the structure of an ordinary dimension, which is defined as a
/// statistical concept used (most probably together with other statistical concepts) to
/// identify a statistical series, such as a time series, e.g. a statistical concept
/// indicating certain economic activity or a geographical reference area. The dimension
/// takes its semantic, and in some cases it representation, from its concept identity. A
/// dimension can be coded by referencing a code list from its coded local representation. It
/// can also specify its text format, which is used as the representation of the dimension if
/// a coded representation is not defined. Neither the coded or uncoded representation are
/// necessary, since the dimension may take these from the referenced concept.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DimensionType {
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// ConceptRoles references concepts which define roles which this dimension serves. If the
    /// concept from which the attribute takes its identity also defines a role the concept
    /// serves, then the isConceptRole indicator can be set to true on the concept identity
    /// rather than repeating the reference here.
    #[serde(rename = "conceptRoles")]
    pub concept_roles: Option<Vec<String>>,
    #[serde(rename = "localRepresentation")]
    pub local_representation: Option<SimpleDataStructureRepresentationType>,
    /// The position attribute specifies the position of the dimension in the data structure
    /// definition, starting at 0. It is optional as the position of the dimension in the key
    /// descriptor (DimensionList element) always takes precedence over the value supplied here.
    /// This is strictly for informational purposes only.
    pub position: Option<i64>,
    /// The type attribute identifies whether then dimension is a measure dimension, the time
    /// dimension, or a regular dimension. Although these are all apparent by the element names,
    /// this attribute allows for each dimension to be processed independent of its element as
    /// well as maintaining the restriction of only one measure and time dimension while still
    /// allowing dimension to occur in any order.
    #[serde(rename = "type")]
    pub dimension_type_type: Option<DimensionTypeType>,
}

/// MeasureDimension is a special type of dimension which defines multiple measures in a data
/// structure. This is represented as any other dimension in a unless it is the observation
/// dimension. It takes it representation from a concept scheme, and this scheme defines the
/// measures and their representations. When data is formatted with this as the observation
/// dimension, these measures can be made explicit or the value of the dimension can be
/// treated as any other dimension. If the measures are explicit, the representation of the
/// observation will be specific to the core representation for each concept in the
/// representation concept scheme. Note that it is necessary that these representations are
/// compliant (the same or derived from) with that of the primary measure.
///
/// MeasureDimensionType defines the structure of the measure dimension. It is derived from
/// the base dimension structure, but requires that a coded representation taken from a
/// concept scheme is given.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MeasureDimensionType {
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    #[serde(rename = "localRepresentation")]
    pub local_representation: MeasureDimensionRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// ConceptRoles references concepts which define roles which this dimension serves. If the
    /// concept from which the attribute takes its identity also defines a role the concept
    /// serves, then the isConceptRole indicator can be set to true on the concept identity
    /// rather than repeating the reference here.
    #[serde(rename = "conceptRoles")]
    pub concept_roles: Option<Vec<String>>,
    /// The position attribute specifies the position of the dimension in the data structure
    /// definition, starting at 0. It is optional as the position of the dimension in the key
    /// descriptor (DimensionList element) always takes precedence over the value supplied here.
    /// This is strictly for informational purposes only.
    pub position: Option<i64>,
    /// The type attribute identifies whether then dimension is a measure dimension, the time
    /// dimension, or a regular dimension. Although these are all apparent by the element names,
    /// this attribute allows for each dimension to be processed independent of its element as
    /// well as maintaining the restriction of only one measure and time dimension while still
    /// allowing dimension to occur in any order.
    #[serde(rename = "type")]
    pub measure_dimension_type_type: Option<DimensionTypeType>,
}

/// BaseDimensionRepresentationType is an abstract base which defines the representation for
/// a measure dimension.
#[derive(Serialize, Deserialize)]
pub struct MeasureDimensionRepresentationType {
    /// Urn reference to a concept scheme object.
    pub enumeration: String,
}

/// TimeDimension is a special dimension which designates the period in time in which the
/// data identified by the full series key applies.
///
/// TimeDimensionType describes the structure of a time dimension. The time dimension takes
/// its semantic from its concept identity (usually the TIME_PERIOD concept), yet is always
/// has a fixed identifier (TIME_PERIOD). The time dimension always has a fixed text format,
/// which specifies that its format is always the in the value set of the observational time
/// period (see common:ObservationalTimePeriodType). It is possible that the format may be a
/// sub-set of the observational time period value set. For example, it is possible to state
/// that the representation might always be a calendar year. See the enumerations of the
/// textType attribute in the LocalRepresentation/TextFormat for more details of the possible
/// sub-sets. It is also possible to facet this representation with start and end dates. The
/// purpose of such facts is to restrict the value of the time dimension to occur within the
/// specified range. If the time dimension is expected to allow for the standard reporting
/// periods (see common:ReportingTimePeriodType) to be used, then it is strongly recommended
/// that the reporting year start day attribute also be included in the data structure
/// definition. When the reporting year start day attribute is used, any standard reporting
/// period values will be assumed to be based on the start day contained in this attribute.
/// If the reporting year start day attribute is not included and standard reporting periods
/// are used, these values will be assumed to be based on a reporting year which begins
/// January 1.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct TimeDimensionType {
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    #[serde(rename = "localRepresentation")]
    pub local_representation: TimeDimensionRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// The position attribute specifies the position of the dimension in the data structure
    /// definition, starting at 0. It is optional as the position of the dimension in the key
    /// descriptor (DimensionList element) always takes precedence over the value supplied here.
    /// This is strictly for informational purposes only.
    pub position: Option<i64>,
    /// The type attribute identifies whether the dimension is a measure dimension, the time
    /// dimension, or a regular dimension. Although these are all apparent by the element names,
    /// this attribute allows for each dimension to be processed independent of its element as
    /// well as maintaining the restriction of only one measure and time dimension while still
    /// allowing dimension to occur in any order.
    #[serde(rename = "type")]
    pub time_dimension_type_type: Option<DimensionTypeType>,
}

/// TimeDimensionRepresentationType defines the representation for the time dimension.
/// Enumerated values are not allowed.
#[derive(Serialize, Deserialize)]
pub struct TimeDimensionRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: TimeTextFormatType,
}

/// TimeTextFormat is a restricted version of the SimpleComponentTextFormatType that only
/// allows time based format and specifies a default ObservationalTimePeriod representation
/// and facets of a start and end time.
#[derive(Serialize, Deserialize)]
pub struct TimeTextFormatType {
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "textType")]
    pub text_type: Option<TimeDataType>,
}

/// Group describes a group descriptor in a data structure definition. It is a set metadata
/// concepts (and possibly their values) that define a partial key derived from the key
/// descriptor in a data structure definition.
///
/// GroupType describes the structure of a group descriptor in a data structure definition. A
/// group may consist of a of partial key, or collection of distinct cube regions or key sets
/// to which attributes may be attached. The purpose of a group is to specify attributes
/// values which have the same value based on some common dimensionality. All groups declared
/// in the data structure must be unique - that is, you may not have duplicate partial keys.
/// All groups must be given unique identifiers.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct GroupTypeElement {
    pub id: String,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// Urn reference to an attachment constraint that defines the key sets and/or cube regions
    /// that attributes may be attached to. This is an alternative to referencing the dimensions,
    /// and allows attributes to be attached to data for given values of dimensions.
    #[serde(rename = "attachmentConstraint")]
    pub attachment_constraint: Option<String>,
    #[serde(rename = "groupDimensions")]
    pub group_dimensions: Option<Vec<String>>,
}

/// MeasureList describes the measure descriptor for a data structure. It contains a single
/// metadata concepts that define the primary measures of a data structure.
///
/// MeasureListType describes the structure of the measure descriptor for a data structure
/// definition. Only a primary may be defined.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MeasureListTypeClass {
    #[serde(rename = "primaryMeasure")]
    pub primary_measure: PrimaryMeasureType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// PrimaryMeasure defines the structure of the primary measure, which is the concept that is
/// the value of the phenomenon to be measured in a data set. Although this may take its
/// semantic from any concept, this is provided a fixed identifier (OBS_VALUE) so that it may
/// be easily distinguished in data messages.
///
/// PrimaryMeasureType describes the structure of the primary measure. It describes the
/// observation values for all presentations of the data. The primary measure takes its
/// semantic, and in some cases it representation, from its concept identity (conventionally
/// the OBS_VALUE concept). The primary measure can be coded by referencing a code list from
/// its coded local representation. It can also specify its text format, which is used as the
/// representation of the primary measure if a coded representation is not defined. Neither
/// the coded or uncoded representation are necessary, since the primary measure may take
/// these from the referenced concept. Note that if the data structure declares a measure
/// dimension, the representation of this must be a superset of all possible measure concept
/// representations.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct PrimaryMeasureType {
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    #[serde(rename = "localRepresentation")]
    pub local_representation: Option<SimpleDataStructureRepresentationType>,
}

/// dataflow provides the details of a data flow, which is defined as the structure of data
/// that will be provided for different reference periods.
///
/// DataflowType describes the structure of a data flow. A data flow is defined as the
/// structure of data that will provided for different reference periods. If this type is not
/// referenced externally, then a reference to a data structure definition must be provided.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataflowTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// Urn reference to the data structure definition which defines the structure of all data
    /// for this flow.
    pub structure: Option<String>,
}

/// hierarchicalCodelist provides the details of a hierarchical code list, which is defined
/// as an organised collection of codes that may participate in many parent/child
/// relationships with other codes in the list, as defined by one or more hierarchy of the
/// list.
///
/// HierarchicalCodelistType describes the structure of a hierarchical codelist. A
/// hierarchical code list is defined as an organised collection of codes that may
/// participate in many parent/child relationships with other codes in the list, as defined
/// by one or more hierarchy of the list.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct HierarchicalCodelistTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub hierarchies: Option<Vec<HierarchyTypeElement>>,
    /// includedCodelists provides a collection of references to the code lists whose codes are
    /// arranged in this hierarchical code list.
    #[serde(rename = "includedCodelists")]
    pub included_codelists: Option<HashMap<String, Option<serde_json::Value>>>,
}

/// Hierarchy describes a classification structure arranged in levels of detail from the
/// broadest to the most detailed level. These levels can be formal or informal, and are not
/// necessary to describe. If the hierarchy does contain levels, then each hierarchical code
/// is assumed to exist in the level where the depths of nesting match.
///
/// The Hierarchy is an abstract type that provides for a classification structure of
/// referenced codes arranged in levels of detail from the broadest to the most detailed
/// level. The levels in which the code exist can be formal or informal.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct HierarchyTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "hierarchicalCodes")]
    pub hierarchical_codes: Vec<HierarchicalCodeTypeElement>,
    /// In a formally leveled hierarchy, Level describes a group of codes which are characterised
    /// by homogeneous coding, and where the parent of each code in the group is at the same
    /// higher level of the hierarchy. In a value based hierarchy Level describes information
    /// about the codes at the specified nesting level. This structure is recursive to indicate
    /// the hierarchy of the levels.
    pub level: Box<Option<LevelTypeClass>>,
    pub leveled: Option<bool>,
}

/// HierarchicalCode is used to assemble the codes from the codelist(s) referenced into a
/// hierarchy.
///
/// HierarchicalCodeType describes the structure of a hierarchical code. A hierarchical code
/// provides for a reference to a code that is referenced within the hierarchical code list
/// via either a complete reference to a code through a URN, or a local reference which
/// utilizes the included codelist reference alias and the identification of a code from the
/// list. Codes are arranged in a hierarchy by this reference. Note that it is possible to
/// reference a single code such that it has multiple parents within the hierarchy. Further,
/// the hierarchy may or may not be a leveled one.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// HierarchicalCode is used to nest referenced codes into a value based hierarchy.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct HierarchicalCodeTypeElement {
    pub id: String,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// Code provides a complete, explicit urn reference to the codelist and code.
    pub code: Option<String>,
    /// CodeID references the id of a code from the codelist that is referenced through the
    /// CodelistAliaRef element.
    #[serde(rename = "codeID")]
    pub code_id: Option<String>,
    /// CodelistAliasRef references an alias assigned in a IncludedCodelist element in the
    /// containing hierarchical codelist. This is used in conjunction with the CodeID element to
    /// reference a code from one of the included codelists.
    #[serde(rename = "codelistAliasRef")]
    pub codelist_alias_ref: Option<String>,
    #[serde(rename = "hierarchicalCodes")]
    pub hierarchical_codes: Option<Vec<HierarchicalCodeTypeElement>>,
    /// Level references (through a urn) a formal level defined within the hierarchy which
    /// defines this hierarchical code. This is only necessary if the nesting depth of the
    /// hierarchical code does not correspond to the nesting depth of the level to which it
    /// belongs (i.e. the hieararchical code is to skip down a level). Otherwise, the code is
    /// assumed to exist at the level in which the nesting depth of the level matches the nesting
    /// depth of the code.
    pub level: Option<String>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    pub version: Option<String>,
}

/// In a formally leveled hierarchy, Level describes a group of codes which are characterised
/// by homogeneous coding, and where the parent of each code in the group is at the same
/// higher level of the hierarchy. In a value based hierarchy Level describes information
/// about the codes at the specified nesting level. This structure is recursive to indicate
/// the hierarchy of the levels.
///
/// LevelType describes a level in a hierarchical codelist. Where level is defined as a group
/// where codes can be characterised by homogeneous coding, and where the parent of each code
/// in the group is at the same higher level of the hierarchy.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// Level describes the next level down in the hierarchy.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct LevelTypeClass {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    /// CodingFormat specifies the text formatting of the codes in this level. This includes
    /// facets such as the expected characters and the length of the codes.
    #[serde(rename = "codingFormat")]
    pub coding_format: Option<CodingTextFormatType>,
    /// Level describes the next level down in the hierarchy.
    pub level: Box<Option<LevelTypeClass>>,
}

/// CodingFormat specifies the text formatting of the codes in this level. This includes
/// facets such as the expected characters and the length of the codes.
#[derive(Serialize, Deserialize)]
pub struct CodingTextFormatType {
    #[serde(rename = "endValue")]
    pub end_value: Option<i64>,
    pub interval: Option<i64>,
    #[serde(rename = "isSequence")]
    pub is_sequence: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<i64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<i64>,
    pub pattern: Option<String>,
    #[serde(rename = "startValue")]
    pub start_value: Option<i64>,
    #[serde(rename = "textType")]
    pub text_type: Option<SimpleCodeDataType>,
}

/// metadataStructure provides the details of a metadata structure definition, which is
/// defined as a collection of metadata concepts, their structure and usage when used to
/// collect or disseminate reference metadata. A metadata structure definition performs
/// several functions: it groups sets of objects into "targets" against which reference
/// metadata may be reported. Targets define the structure of the reference metadata "keys"
/// which identify specific types of reported metadata, and describe the valid values for
/// populating the keys. Also, metadata structure definitions provide a presentational
/// organization of concepts for reporting purposes. The structure of a reference metadata
/// report is derived from this presentational structure.
///
/// MetadataStructureType is used to describe a metadata structure definition, which is
/// defined as a collection of metadata concepts, their structure and usage when used to
/// collect or disseminate reference metadata.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MetadataStructureTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "metadataStructureComponents")]
    pub metadata_structure_components: Option<MetadataStructureComponentsType>,
}

/// MetadataStructureComponents defines the grouping of the sets of the components that make
/// up the metadata structure definition. All components and component list (target
/// identifiers, identifier components, report structures, and metadata attributes) in the
/// structure definition must have a unique identification.
///
/// MetadataStructureComponentsType describes the structure of the grouping of the sets of
/// the components that make up the metadata structure definition. At a minimum, a full
/// target identifier and at least one report structure must be defined.
#[derive(Serialize, Deserialize)]
pub struct MetadataStructureComponentsType {
    #[serde(rename = "metadataTargets")]
    pub metadata_targets: Vec<MetadataTargetTypeElement>,
    #[serde(rename = "reportStructures")]
    pub report_structures: Vec<ReportStructureTypeElement>,
}

/// MetadataTarget is a collection of target objects which when taken together describe a
/// structure which defines the key of an object type to which metadata may be attached and
/// serve to disambiguate reference metadata set reports.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MetadataTargetTypeElement {
    pub id: String,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    #[serde(rename = "constraintContentTargets")]
    pub constraint_content_targets: Option<Vec<ConstraintContentTargetType>>,
    #[serde(rename = "dataSetTargets")]
    pub data_set_targets: Option<Vec<DataSetTargetType>>,
    #[serde(rename = "identifiableObjectTargets")]
    pub identifiable_object_targets: Option<Vec<Identifiable>>,
    #[serde(rename = "keyDescriptorValuesTargets")]
    pub key_descriptor_values_targets:
        Option<Vec<KeyDescriptorValuesTargetType>>,
    #[serde(rename = "reportPeriodTargets")]
    pub report_period_targets: Option<Vec<ReportPeriodTargetType>>,
}

/// ConstraintContentTarget is target object which references an attachment constraint for
/// the purpose of attaching reference metadata data to data key sets or cube regions defined
/// by the constraint.
///
/// ConstraintTargetType defines the structure of a constraint target object. The constraint
/// target object has a fixed representation and identifier.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ConstraintContentTargetType {
    #[serde(rename = "localRepresentation")]
    pub local_representation: ConstraintRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// ConstraintRepresentationType defines the possible local representations of a constraint
/// reference target object. The representation is fixed to always be an attachment
/// constraint reference.
#[derive(Serialize, Deserialize)]
pub struct ConstraintRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: ConstraintTextFormatType,
}

/// ConstraintTextFormatType is a restricted version of the NonFacetedTextFormatType that
/// specifies a fixed AttachmentConstraintReference representation.
#[derive(Serialize, Deserialize)]
pub struct ConstraintTextFormatType {
    #[serde(rename = "textType")]
    pub text_type: Option<TargetObjectDataType>,
}

/// DataSetTarget is target object which references a data set for the purpose of attaching
/// reference metadata data. A data set reference is a full reference to a data provider and
/// an identifier for the data set.
///
/// DataSetTargetType defines the structure of a data set target object. The data set target
/// object has a fixed representation and identifier.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct DataSetTargetType {
    #[serde(rename = "localRepresentation")]
    pub local_representation: DataSetRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// DataSetRepresentationType defines the possible local representations of a data set
/// reference target object. The representation is fixed to always be a data set reference.
#[derive(Serialize, Deserialize)]
pub struct DataSetRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: DataSetTextFormatType,
}

/// DataSetTextFormatType is a restricted version of the NonFacetedTextFormatType that
/// specifies a fixed DataSetReference representation.
#[derive(Serialize, Deserialize)]
pub struct DataSetTextFormatType {
    #[serde(rename = "textType")]
    pub text_type: Option<TargetObjectDataType>,
}

/// IdentifiableObjectTarget is target object which references an Identifiable object as
/// defined in the SDMX Information Model. The reference must be complete (i.e. a URN or a
/// complete set of reference fields). For an item object, it is possible to define a local
/// representation of an item scheme from which the item must be referenced.
///
/// IdentifiableObjectTargetType defines the structure of an identifiable target object. The
/// identifiable target object has a fixed representation of a reference and can specify a
/// local representation of any item scheme for the purpose of restricting which items may be
/// referenced. The identifiable object target must specify the object type which the target
/// object is meant to reference.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct Identifiable {
    pub id: String,
    #[serde(rename = "localRepresentation")]
    pub local_representation: IdentifiableObjectRepresentationType,
    #[serde(rename = "objectType")]
    pub object_type: ObjectTypeCodelistType,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// IdentifiableObjectRepresentationType defines the possible local representations of an
/// identifiable object target object.
#[derive(Serialize, Deserialize)]
pub struct IdentifiableObjectRepresentationType {
    /// Urn reference to an item scheme. Enumeration is only permissible if the object type of
    /// the identifiable object target is an item in an item scheme. This enumeration is meant to
    /// limit the referencable objects to the items defined in the referenced item scheme.
    pub enumeration: Option<String>,
    #[serde(rename = "textFormat")]
    pub text_format: Option<IdentifiableObjectTextFormatType>,
}

/// IdentifiableObjectTextFormatType is a restricted version of the NonFacetedTextFormatType
/// that specifies a fixed IdentifiableReference representation.
#[derive(Serialize, Deserialize)]
pub struct IdentifiableObjectTextFormatType {
    #[serde(rename = "textType")]
    pub text_type: Option<TargetObjectDataType>,
}

/// KeyDescriptorValuesTarget is target object which references a data key for the purpose of
/// attach reference metadata to portions of data. A data key is a set of dimension
/// references and values for those dimension. This component on its own is not of much use,
/// as the data key only has local references to the dimensions. Therefore it is typical that
/// this is used in combination with some sort of reference to the data (either a data set
/// reference or a reference to the underlying structure, structure usage, or provision
/// agreement of the data.
///
/// KeyDescriptorValuesTargetType defines the structure of a key descriptor values target
/// object. The key descriptor values target object has a fixed representation and
/// identifier.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct KeyDescriptorValuesTargetType {
    #[serde(rename = "localRepresentation")]
    pub local_representation: KeyDescriptorValuesRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// KeyDescriptorValuesRepresentationType defines the possible local representations of a key
/// descriptor values target object. The representation is fixed to always be a data key
/// (KeyValues).
#[derive(Serialize, Deserialize)]
pub struct KeyDescriptorValuesRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: KeyDescriptorValuesTextFormatType,
}

/// KeyDescriptorValuesTextFormatType is a restricted version of the NonFacetedTextFormatType
/// that specifies a fixed KeyValues representation.
#[derive(Serialize, Deserialize)]
pub struct KeyDescriptorValuesTextFormatType {
    #[serde(rename = "textType")]
    pub text_type: Option<TargetObjectDataType>,
}

/// ReportPeriodTarget is target object which specifies a reporting period to which a
/// metadata report applies.
///
/// ReportPeriodTargetType defines the structure of a report period target object. The report
/// period target object has a fixed representation and identifier.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportPeriodTargetType {
    #[serde(rename = "localRepresentation")]
    pub local_representation: ReportPeriodRepresentationType,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// ReportPeriodRepresentationType defines the possible local representations of a report
/// period target object. The reprentation must be a time period or a subset of this
/// representation.
#[derive(Serialize, Deserialize)]
pub struct ReportPeriodRepresentationType {
    #[serde(rename = "textFormat")]
    pub text_format: TimeTextFormatType,
}

/// ReportStructure defines a report structure, which comprises a set of metadata attributes
/// that can be defined as a hierarchy, for reporting reference metadata about a target
/// object. The identification of metadata attributes must be unique at any given level of
/// the report structure. Although there are XML schema constraints to help enforce this,
/// these only apply to explicitly assigned identifiers. Identifiers inherited from a concept
/// from which a metadata attribute takes its identity cannot be validated against this
/// constraint. Therefore, systems processing metadata structure definitions will have to
/// perform this check outside of the XML validation.
///
/// ReportStructureType describes the structure of a report structure. It comprises a set of
/// metadata attributes that can be defined as a hierarchy, and identifies the potential
/// attachment of these attributes to an object by referencing a target identifier.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportStructureTypeElement {
    pub id: String,
    #[serde(rename = "metadataAttributes")]
    pub metadata_attributes: Vec<MetadataAttributeType>,
    #[serde(rename = "metadataTargets")]
    pub metadata_targets: Vec<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
}

/// MetadataAttribute defines the a metadata attribute, which is the value of an attribute,
/// such as the instance of a coded or uncoded attribute in a metadata structure definition.
///
/// MetadataAttributeType describes the structure of a metadata attribute. The metadata
/// attribute takes its semantic, and in some cases it representation, from its concept
/// identity. A metadata attribute may be coded (via the local representation), uncoded (via
/// the text format), or take no value. In addition to this value, the metadata attribute may
/// also specify subordinate metadata attributes. If a metadata attribute only serves the
/// purpose of containing subordinate metadata attributes, then the isPresentational
/// attribute should be used. Otherwise, it is assumed to also take a value. If the metadata
/// attribute does take a value, and a representation is not defined, it will be inherited
/// from the concept it takes its semantic from. The optional id on the metadata attribute
/// uniquely identifies it within the metadata structured definition. If this id is not
/// supplied, its value is assumed to be that of the concept referenced from the concept
/// identity. Note that a metadata attribute (as identified by the id attribute) definition
/// must be unique across the entire metadata structure definition (including target
/// identifier, identifier component, and report structure ids). A metadata attribute may be
/// used in multiple report structures and at different levels, but the content (value and/or
/// child metadata attributes and their cardinality) of the metadata attribute cannot
/// change.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MetadataAttributeType {
    /// Urn reference to a concept where the identification of the concept scheme which defines
    /// it is contained in another context.
    #[serde(rename = "conceptIdentity")]
    pub concept_identity: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    #[serde(rename = "isPresentational")]
    pub is_presentational: Option<bool>,
    #[serde(rename = "localRepresentation")]
    pub local_representation: Option<MetadataAttributeRepresentationType>,
    #[serde(rename = "maxOccurs")]
    pub max_occurs: Option<OccurenceType>,
    #[serde(rename = "metadataAttributes")]
    pub metadata_attributes: Option<Vec<MetadataAttributeType>>,
    #[serde(rename = "minOccurs")]
    pub min_occurs: Option<i64>,
}

/// MetadataAttributeRepresentationType defines the possible local representations of a
/// metadata attribute.
#[derive(Serialize, Deserialize)]
pub struct MetadataAttributeRepresentationType {
    /// Urn reference to a codelist object.
    pub enumeration: Option<String>,
    #[serde(rename = "enumerationFormat")]
    pub enumeration_format: Option<CodededTextFormatType>,
    #[serde(rename = "textFormat")]
    pub text_format: Option<BasicComponentTextFormatType>,
}

/// metadataflow provides the details of a metadata flow, which is defined as the structure
/// of reference metadata that will be provided for different reference periods
///
/// MetadataflowType describes the structure of a metadata flow. A dataflow is defined as the
/// structure of reference metadata that will be provided for different reference periods. If
/// this type is not referenced externally, then a reference to a metadata structure
/// definition must be provided.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct MetadataflowTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// Structure provides a urn reference to the metadata structure definition describing the
    /// structure of all reference metadata for this flow.
    pub structure: Option<String>,
}

/// organisationUnitScheme provides the details of an organisation unit scheme, in which
/// organisation units are described.
///
/// OrganisationUnitSchemeType defines a type of organisation scheme which simply defines
/// organisations and there parent child relationships. Organisations in this scheme are
/// assigned no particular role, and may in fact exist within the other type of organisation
/// schemes as well.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct OrganisationUnitSchemeTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
    #[serde(rename = "organisationUnits")]
    pub organisation_units: Option<Vec<OrganisationUnitTypeElement>>,
}

/// OrganisationUnit describes a generic organisation, which serves not predefined role in
/// SDMX.
///
/// OrganisationUnitType defines the structure of an organisation unit description. In
/// addition to general identification and contact information, an organisation unit can
/// specify a relationship with another organisation unit from the same scheme which is its
/// parent organisation.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct OrganisationUnitTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    pub contacts: Option<Vec<ContactType>>,
    /// Urn reference to an organisation unit, where the reference to the organisation unit
    /// scheme which defines it is provided in another context.
    pub parent: Option<String>,
}

/// process provides the details of a process, which is a scheme which defines or documents
/// the operations performed on data in order to validate data or to derive new information
/// according to a given set of rules. It is not meant to support process automation, but
/// serves as a description of how processes occur. The primary use for this structural
/// mechanism is the attachment of reference metadata regarding statistical processing. This
/// must either contain the full details of the category scheme, or provide a name and
/// identification information and reference the full details from an external structure
/// document or registry service.
///
/// ProcessType describes the structure of a process, which is a scheme which defines or
/// documents the operations performed on data in order to validate data or to derive new
/// information according to a given set of rules. Processes occur in order, and will
/// continue in order unless a transition dictates another step should occur.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ProcessTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "processSteps")]
    pub process_steps: Option<Vec<ProcessStepTypeElement>>,
}

/// ProcessStep defines a process step, which is a specific operation, performed on data in
/// order to validate or to derive new information according to a given set of rules.
///
/// ProcessStepType describes the structure of a process step. A nested process step is
/// automatically sub-ordinate, and followed as the next step. If the following step is
/// conditional, it should be referenced in a transition.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ProcessStepTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Computation describes the computations involved in the process, in any form desired by
    /// the user (these are informational rather than machine-actionable), and so may be supplied
    /// in multiple, parallel-language versions.
    pub computation: Option<ComputationType>,
    pub inputs: Option<Vec<InputOutputType>>,
    pub outputs: Option<Vec<InputOutputType>>,
    #[serde(rename = "processSteps")]
    pub process_steps: Option<Vec<ProcessStepTypeElement>>,
    pub transitions: Option<Vec<TransitionTypeElement>>,
}

/// Computation describes the computations involved in the process, in any form desired by
/// the user (these are informational rather than machine-actionable), and so may be supplied
/// in multiple, parallel-language versions.
///
/// ComputationType describes a computation in a process.
#[derive(Serialize, Deserialize)]
pub struct ComputationType {
    pub annotations: Option<Vec<AnnotationType>>,
    pub description: String,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "localID")]
    pub local_id: Option<String>,
    #[serde(rename = "softwareLanguage")]
    pub software_language: Option<String>,
    #[serde(rename = "softwarePackage")]
    pub software_package: Option<String>,
    #[serde(rename = "softwareVersion")]
    pub software_version: Option<String>,
}

/// Input references an object which is an input to the process step.
///
/// InputOutputType describes the structure of an input or output to a process step. It
/// provides a reference to the object that is the input or output.
///
/// Output references an object which is an output form the process step.
#[derive(Serialize, Deserialize)]
pub struct InputOutputType {
    pub annotations: Option<Vec<AnnotationType>>,
    #[serde(rename = "localID")]
    pub local_id: Option<String>,
    /// ObjectReference is an abstract substitution head that references (through a urn) the
    /// object that is an input or output. It is substituted with a concrete reference to an
    /// explicit object type.
    #[serde(rename = "objectReference")]
    pub object_reference: String,
}

/// Transition describes the next process steps. Each transition in a process step should be
/// evaluated, allowing for multiple process step branches from a single process step.
///
/// TransitionType describes the details of a transition, which is an expression in a textual
/// or formalised way of the transformation of data between two specific operations performed
/// on the data.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct TransitionTypeElement {
    /// Condition is a textual description of the conditions to be met in order for the target
    /// step to be proceeded to. It is informational only (not machine-actionable), provided in
    /// the best-match language.
    pub condition: String,
    /// TargetStep references (through a urn) a process step within the process that should be
    /// transitioned to, should the conditions described be met.
    #[serde(rename = "targetStep")]
    pub target_step: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    /// Condition is a textual description of the conditions to be met in order for the target
    /// step to be proceeded to. It is informational only (not machine-actionable), and may be
    /// supplied in multiple, parallel-language form.
    pub conditions: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "localID")]
    pub local_id: Option<String>,
}

/// provisionAgreement provides the details of a provision agreement, which is an agreement
/// for a data provider to report data or reference metadata against a flow.
///
/// ProvisionAgreementType describes the structure of a provision agreement. A provision
/// agreement defines an agreement for a data provider to report data or reference metadata
/// against a flow. Attributes which describe how the registry must behave when data or
/// metadata is registered against this provision agreement are supplied.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ProvisionAgreementTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    /// DataProvider is a urn reference to a pre-existing data (or metadata) provider in the
    /// registry.
    #[serde(rename = "dataProvider")]
    pub data_provider: String,
    /// DataflowReference provides a urn reference to a pre-existing structure usage (i.e. a
    /// dataflow or metadataflow) in the registry.
    #[serde(rename = "structureUsage")]
    pub structure_usage: String,
}

/// reportingTaxonomy provides the details of a reporting taxonomy, which is a scheme which
/// defines the composition structure of a data report where each component can be described
/// by an independent data or metadata flow definition.
///
/// ReportingTaxonomyType describes the structure of a reporting taxonomy, which is a scheme
/// which defines the composition structure of a data report where each component can be
/// described by an independent structure or structure usage description.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportingTaxonomyTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "isPartial")]
    pub is_partial: Option<bool>,
    #[serde(rename = "reportingCategories")]
    pub reporting_categories: Option<Vec<ReportingCategoryTypeElement>>,
}

/// ReportingCateogry defines a reporting category, which is used to group structure usages
/// into useful sub-packages.
///
/// ReportingCategoryType describes the structure of a reporting category, which groups
/// structure usages into useful sub-packages. Sub ordinate reporting categories can be
/// nested within the category definition.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportingCategoryTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "provisioningMetadata")]
    pub provisioning_metadata: Option<Vec<String>>,
    #[serde(rename = "reportingCategories")]
    pub reporting_categories: Option<Vec<ReportingCategoryTypeElement>>,
    #[serde(rename = "structuralMetadata")]
    pub structural_metadata: Option<Vec<String>>,
}

/// structureSet provides the details or a structure set, which allows components in one
/// structure, structure usage, or item scheme to be mapped to components in another
/// structural component of the same type.
///
/// StructureSetType describes the structure of a structure set. It allows components in one
/// structure, structure usage, or item scheme to be mapped to components in another
/// structural component of the same type.
///
/// MaintainableType is an abstract base type for all maintainable objects.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct StructureSetTypeElement {
    pub id: String,
    #[serde(rename = "agencyID")]
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "isExternalReference")]
    pub is_external_reference: Option<bool>,
    #[serde(rename = "isFinal")]
    pub is_final: Option<bool>,
    #[serde(rename = "validFrom")]
    pub valid_from: Option<String>,
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,
    #[serde(rename = "categorySchemeMaps")]
    pub category_scheme_maps: Option<Vec<CategorySchemeMapTypeElement>>,
    #[serde(rename = "codelistMaps")]
    pub codelist_maps: Option<Vec<CodelistMapTypeElement>>,
    #[serde(rename = "conceptSchemeMaps")]
    pub concept_scheme_maps: Option<Vec<ConceptSchemeMapTypeElement>>,
    #[serde(rename = "hybridCodelistMaps")]
    pub hybrid_codelist_maps: Option<Vec<HybridCodelistMapTypeElement>>,
    #[serde(rename = "organisationSchemeMaps")]
    pub organisation_scheme_maps: Option<Vec<OrganisationSchemeMapTypeElement>>,
    #[serde(rename = "relatedStructures")]
    pub related_structures: Option<Vec<String>>,
    #[serde(rename = "reportingTaxonomyMaps")]
    pub reporting_taxonomy_maps: Option<Vec<ReportingTaxonomyMapTypeElement>>,
    #[serde(rename = "structureMaps")]
    pub structure_maps: Option<Vec<StructureMapTypeElement>>,
}

/// CategorySchemeMap links a source and target categories from different schemes where there
/// is a semantic equivalence between them.
///
/// CategorySchemeMapType defines the structure of a map which identifies relationships
/// between categories in different category schemes.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CategorySchemeMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "categoryMaps")]
    pub category_maps: Vec<CategoryMapType>,
    /// Urn reference to a category scheme object.
    pub source: String,
    /// Urn reference to a category scheme object.
    pub target: String,
}

/// CategoryMap defines the structure of a map which identifies relationships between
/// categories in different category schemes.
///
/// CategoryMapType defines the structure for mapping two categories. A local reference is
/// provided both the source and target category.
#[derive(Serialize, Deserialize)]
pub struct CategoryMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Urn reference to a category where the identification of the category scheme which defines
    /// it is contained in another context.
    pub source: String,
    /// Urn reference to a category where the identification of the category scheme which defines
    /// it is contained in another context.
    pub target: String,
}

/// CodelistMap links a source and target codes from different lists where there is a
/// semantic equivalence between them.
///
/// CodelistMapType defines the structure of a map which identifies relationships between
/// codes in different codelists.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct CodelistMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "codeMaps")]
    pub code_maps: Vec<CodeMapType>,
    /// Source provides a urn reference to a codelist.
    pub source: String,
    /// Target provides a urn reference to a codelist.
    pub target: String,
}

/// CodeMap defines the structure of a map which identifies relationships between codes in
/// different codelists.
///
/// CodeMapType defines the structure for mapping two codes. A local reference is provided
/// both the source and target code.
#[derive(Serialize, Deserialize)]
pub struct CodeMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Urn reference to a code where the identification of the codelist which defines it is
    /// contained in another context.
    pub source: String,
    /// Urn reference to a code where the identification of the codelist which defines it is
    /// contained in another context.
    pub target: String,
}

/// ConceptSchemeMap links a source and target concepts from different schemes where there is
/// a semantic equivalence between them.
///
/// ConceptSchemeMapType defines the structure of a map which identifies relationships
/// between concepts in different concept schemes.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ConceptSchemeMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "conceptMaps")]
    pub concept_maps: Vec<ConceptMapType>,
    /// Urn reference to a concept scheme object.
    pub source: String,
    /// Urn reference to a concept scheme object.
    pub target: String,
}

/// ConceptMap defines the structure of a map which identifies relationships between concepts
/// in different concept schemes.
///
/// ConceptMapType defines the structure for mapping two concepts. A local reference is
/// provided both the source and target concept.
#[derive(Serialize, Deserialize)]
pub struct ConceptMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Urn reference to a local concept.
    pub source: String,
    /// Urn reference to a local concept.
    pub target: String,
}

/// HybridCodelistMap links a source and target codes from different codelists, which may be
/// hierarchical or flat, where there is a semantic equivalence between them.
///
/// HybridCodelistMapType defines the structure of a map which relates codes (possibly
/// hierarchical) from different code lists.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct HybridCodelistMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "hybridCodeMaps")]
    pub hybrid_code_maps: Vec<HybridCodeMapType>,
    /// Source provides a urn reference to either a codelist or a hierarchical codelist, from
    /// which the codes are to be mapped.
    pub source: String,
    /// Target provides a urn reference to either a codelist or a hierarchical codelist, to which
    /// the source codes are to be mapped.
    pub target: String,
}

/// HybridCodeMap defines the relationship of a code in the source list to code in the target
/// list.
///
/// CodeMapType defines the structure for associating a code from a source codelist to a code
/// in a target codelist. Note that either of these may come from a hierarchical codelist.
#[derive(Serialize, Deserialize)]
pub struct HybridCodeMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Source provides a local reference to the code which is to be mapped. If this code is from
    /// a hierarchical codelist, a reference to the hierarchy in which it is defined must also be
    /// provided.
    pub source: String,
    /// Target provides a local reference to the code to which the source code is mapped. If this
    /// code is from a hierarchical codelist, a reference to the hierarchy in which it is defined
    /// must also be provided.
    pub target: String,
}

/// OrganisationSchemeMap links a source and target organisations from different schemes
/// where there is a semantic equivalence between them. Organisations are mapped without
/// regard to role.
///
/// OrganisationSchemeMapType defines the structure of a map which identifies relationships
/// between organisations in different organisation schemes.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct OrganisationSchemeMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "organisationMaps")]
    pub organisation_maps: Vec<OrganisationMapType>,
    /// Urn reference to an organisation scheme regardless of the specific type.
    pub source: String,
    /// Urn reference to an organisation scheme regardless of the specific type.
    pub target: String,
}

/// OrganisationMap relates a source organisation to a target organisation.
///
/// OrganisationMapType defines the structure for mapping two organisations. A local
/// reference is provided both the source and target organisation.
#[derive(Serialize, Deserialize)]
pub struct OrganisationMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Urn reference to an organisation, regardless of type, where the identification of the
    /// organisation scheme which defines it is contained in another context.
    pub source: String,
    /// Urn reference to an organisation, regardless of type, where the identification of the
    /// organisation scheme which defines it is contained in another context.
    pub target: String,
}

/// ReportingTaxonomyMap links a source and target reporting categories from different
/// taxonomies where there is a semantic equivalence between them.
///
/// ReportingTaxonomyMapType defines the structure of a map which identifies relationships
/// between reporting categories in different reporting taxonomies.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct ReportingTaxonomyMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "reportingCategoryMaps")]
    pub reporting_category_maps: Vec<ReportingCategoryMapType>,
    /// Urn reference to a reporting taxonomy object.
    pub source: String,
    /// Urn reference to a reporting taxonomy object.
    pub target: String,
}

/// ReportingCategoryMap defines the structure of a map which identifies relationships
/// between reporting categories in different reporting taxonomies.
///
/// ReportingCategoryMapType defines the structure for mapping two reporting categories. A
/// local reference is provided both the source and target category.
#[derive(Serialize, Deserialize)]
pub struct ReportingCategoryMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// Urn reference to a reporting category.
    pub source: String,
    /// Urn reference to a reporting category.
    pub target: String,
}

/// StructureMap maps components from one structure to components to another structure, and
/// can describe how the value of the components are related.
///
/// StructureMapType defines the structure for mapping components of one structure to
/// components of another structure. A structure may be referenced directly meaning the map
/// applies wherever the structure is used, or it may be a reference via a structure usage
/// meaning the map only applies within the context of that usage. Using the related
/// structures, one can make extrapolations between maps. For example, if data structures, A,
/// B, and C, are all grouped in a related structures container, then a map from data
/// structure A to C and a map from data structure B to C could be used to infer a relation
/// between data structure A to C.
///
/// NameableType is an abstract base type for all nameable objects.
///
/// IdentifiableType is an abstract base type for all identifiable objects.
///
/// AnnotableType is an abstract base type used for all annotable artefacts. Any type that
/// provides for annotations should extend this type.
#[derive(Serialize, Deserialize)]
pub struct StructureMapTypeElement {
    pub name: String,
    pub id: Option<String>,
    pub annotations: Option<Vec<AnnotationType>>,
    /// Links field is an array of link objects. Also used to specify the URI or the URN to
    /// itself. If appropriate, a collection of links to additional external resources.
    pub links: Option<Vec<Link>>,
    pub description: Option<String>,
    pub descriptions: Option<HashMap<String, Option<serde_json::Value>>>,
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "componentMaps")]
    pub component_maps: Vec<ComponentMapType>,
    #[serde(rename = "isExtension")]
    pub is_extension: Option<bool>,
    /// Source provides a reference to a structure (data or metadata) or a structure usage
    /// (dataflow or metadataflow) from which components defined by the actual structure are to
    /// mapped.
    pub source: String,
    /// Target provides a reference to a structure (data or metadata) or a structure usage
    /// (dataflow or metadataflow) to which components from the source are to mapped.
    pub target: String,
}

/// ComponentMap defines the relationship between the components of the source and target
/// structures, including information on how the value from the source component relates to
/// values in the target component.
///
/// ComponentMapType defines the structure for relating a component in a source structure to
/// a component in a target structure.
#[derive(Serialize, Deserialize)]
pub struct ComponentMapType {
    pub annotations: Option<Vec<AnnotationType>>,
    /// RepresentationMapping describes the mapping rules to map the value of the source
    /// component to the target component. Note that is a representation mapping is not supplied,
    /// then the value of the source component is mapped directly to the value of the target
    /// component without any manipulation.
    #[serde(rename = "representationMapping")]
    pub representation_mapping: Option<RepresentationMapType>,
    /// Urn reference to any type of component in a specific component list where the reference
    /// to the structure which defines it are provided in another context.
    pub source: String,
    /// Urn reference to any type of component in a specific component list where the reference
    /// to the structure which defines it are provided in another context.
    pub target: String,
}

/// RepresentationMapping describes the mapping rules to map the value of the source
/// component to the target component. Note that is a representation mapping is not supplied,
/// then the value of the source component is mapped directly to the value of the target
/// component without any manipulation.
///
/// RepresentationMapType describes the structure of the mapping of the value of a source to
/// component to a target component. Either a reference to another map defined within the
/// containing structure set or a description of the source and target text formats must be
/// provided. Note that for data structure components, only a reference to a codelist map is
/// relevant, since that is the only type of coded representation allowed in a data structure.
#[derive(Serialize, Deserialize)]
pub struct RepresentationMapType {
    /// CodelistMap references (through a urn) a codelist map defined in the same structure set
    /// which maps the enumeration of the representation of the source component to the
    /// enumeration of the representation of the target component.
    #[serde(rename = "codelistMap")]
    pub codelist_map: Option<String>,
    /// ToTextFormat describes the un-coded representation of the target to which the value of
    /// the referenced component should be transformed.
    #[serde(rename = "toTextFormat")]
    pub to_text_format: Option<TextFormatType>,
    /// ToValueType notes whether the value, name, or description of the source value should be
    /// used in the target value.
    #[serde(rename = "toValueType")]
    pub to_value_type: Option<ToValueTypeType>,
    /// ValueMap provides for a simple mapping of a source value to a target value without having
    /// to define a codelist map. This is available to allow mappings in situations such as the
    /// source or target is not being formally coded, or the source and/or target being a measure
    /// dimension in which case its representation is not mappable from a codelist map.
    #[serde(rename = "valueMap")]
    pub value_map: Option<ValueMapType>,
}

/// ToTextFormat describes the un-coded representation of the target to which the value of
/// the referenced component should be transformed.
///
/// TextFormatType defines the information for describing a full range of text formats and
/// may place restrictions on the values of the other attributes, referred to as "facets".
#[derive(Serialize, Deserialize)]
pub struct TextFormatType {
    pub decimals: Option<i64>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "endValue")]
    pub end_value: Option<f64>,
    pub interval: Option<f64>,
    #[serde(rename = "isMultiLingual")]
    pub is_multi_lingual: Option<bool>,
    #[serde(rename = "isSequence")]
    pub is_sequence: Option<bool>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    #[serde(rename = "maxValue")]
    pub max_value: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "minValue")]
    pub min_value: Option<f64>,
    pub pattern: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(rename = "startValue")]
    pub start_value: Option<f64>,
    #[serde(rename = "textType")]
    pub text_type: Option<DataType>,
    #[serde(rename = "timeInterval")]
    pub time_interval: Option<String>,
}

/// ValueMap provides for a simple mapping of a source value to a target value without having
/// to define a codelist map. This is available to allow mappings in situations such as the
/// source or target is not being formally coded, or the source and/or target being a measure
/// dimension in which case its representation is not mappable from a codelist map.
///
/// ValueMapType contains a collection of value mappings, which give a source and target
/// value.
#[derive(Serialize, Deserialize)]
pub struct ValueMapType {
    #[serde(rename = "valueMappings")]
    pub value_mappings: Vec<ValueMappingType>,
}

/// ValueMapping provides a source and target value for the purpose of mapping.
///
/// ValueMappingType specifies the relationship between two values as a source and target.
#[derive(Serialize, Deserialize)]
pub struct ValueMappingType {
    pub source: String,
    pub target: String,
}

/// Error describes the structure of an error or warning message.
#[derive(Serialize, Deserialize)]
pub struct Error {
    /// Provides a code number for the error message. Code numbers are defined in the SDMX 2.1
    /// Web Services Guidelines.
    pub code: f64,
    /// Detail contains the detailed text of the message, in best-match language value. A
    /// human-readable localised explanation specific to this occurrence of the problem. Like
    /// title, this field’s value can be localized. It is fully customizable by the service
    /// providers and should provide enough detail to ease understanding the reasons of the error.
    pub detail: Option<String>,
    /// Detail contains the detailed text of the message, in parallel language values. A list of
    /// human-readable localised explanations specific to this occurrence of the problem. Like
    /// title, this field’s value can be localized. It is fully customizable by the service
    /// providers and should provide enough detail to ease understanding the reasons of the error.
    pub details: Option<HashMap<String, Option<serde_json::Value>>>,
    /// Links field is an array of link objects. If appropriate, a collection of links to
    /// additional external resources for the error.
    pub links: Option<Vec<Link>>,
    /// Title contains the title of the message, in best-match language value. A short,
    /// human-readable localised summary of the problem that SHOULD NOT change from occurrence to
    /// occurrence of the problem, except for purposes of localization.
    pub title: Option<String>,
    /// Title contains the title of the message, in parallel language values. A list of short,
    /// human-readable localised summary of the problem that SHOULD NOT change from occurrence to
    /// occurrence of the problem, except for purposes of localization.
    pub titles: Option<HashMap<String, Option<serde_json::Value>>>,
}

/// A meta object that contains non-standard meta-information and basic technical information
/// about the message, such as when it was prepared and who has sent it.
#[derive(Serialize, Deserialize)]
pub struct Meta {
    /// Array of strings containing the identifyer of all languages used anywhere in the message
    /// for localized elements, and thus the languages of the intended audience, representaing in
    /// an array format the same information than the http Content-Language response header, e.g.
    /// "en, fr-fr". See IETF Language Tags: https://tools.ietf.org/html/rfc5646#section-2.1. The
    /// array's first element indicates the main language used in the message for localized
    /// elements. The usage of this property is recommended.
    #[serde(rename = "contentLanguages")]
    pub content_languages: Option<Vec<String>>,
    /// Unique string assigned by the sender that identifies the message for further references.
    pub id: String,
    pub links: Option<Vec<Link>>,
    /// Name provides a name for the transmission. Multiple instances allow for parallel language
    /// values.
    pub name: Option<String>,
    /// Name provides a name for the transmission. Multiple instances allow for parallel language
    /// values.
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
    /// A timestamp indicating when the message was prepared. Values must follow the ISO 8601
    /// syntax for combined dates and times, including time zone.
    pub prepared: String,
    /// Receiver is information about the part(y/ies) that is/are the intended recipient(s) of
    /// the message. This can be useful if the WS requires authentication.
    pub receivers: Option<Vec<Party>>,
    /// Contains the URL to the schema allowing to validate the message. This also allows
    /// identifying the version of SDMX-JSON format used in this message. Providing the link to
    /// the SDMX-JSON schema is recommended.
    pub schema: Option<String>,
    /// Sender is information about the party that is transmitting the message.
    pub sender: Party,
    /// Test indicates whether the message is for test purposes or not. False for normal messages.
    pub test: Option<bool>,
}

/// Sender contains information about the party that is transmitting the message.
///
/// Sender is information about the party that is transmitting the message.
#[derive(Serialize, Deserialize)]
pub struct Party {
    /// Contact provides contact information for the party in regard to the transmission of the
    /// message.
    pub contacts: Option<Vec<ContactType>>,
    /// The id holds the identification of the party.
    pub id: String,
    /// Name is a human-readable name of the party.
    pub name: Option<String>,
    /// Name is a human-readable name of the party.
    pub names: Option<HashMap<String, Option<serde_json::Value>>>,
}

/// OccurenceType is used to express the maximum occurrence of an object. It combines an
/// integer, greater than 1, and the literal text, "unbounded", for objects which have no
/// upper limit on its occurrence.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OccurenceType {
    Enum(OccurenceTypeEnum),
    Integer(i64),
}

/// CodeDataType is a restriction of the basic data types that are applicable to codes.
/// Although some of the higher level time period formats are perimitted, it should be noted
/// that any value which contains time (which includes a time zone offset) is not allowable
/// as a code identifier.
#[derive(Serialize, Deserialize)]
pub enum CodeDataType {
    Alpha,
    AlphaNumeric,
    BasicTimePeriod,
    BigInteger,
    Boolean,
    Count,
    Day,
    Duration,
    ExclusiveValueRange,
    GregorianDay,
    GregorianTimePeriod,
    GregorianYear,
    GregorianYearMonth,
    InclusiveValueRange,
    Incremental,
    Integer,
    Long,
    Month,
    MonthDay,
    Numeric,
    ObservationalTimePeriod,
    ReportingDay,
    ReportingMonth,
    ReportingQuarter,
    ReportingSemester,
    ReportingTimePeriod,
    ReportingTrimester,
    ReportingWeek,
    ReportingYear,
    Short,
    StandardTimePeriod,
    String,
    #[serde(rename = "URI")]
    Uri,
}

/// BasicComponentDataType provides an enumerated list of the types of characters allowed in
/// the textType attribute for all non-target object components.
#[derive(Serialize, Deserialize)]
pub enum BasicComponentDataType {
    Alpha,
    AlphaNumeric,
    BasicTimePeriod,
    BigInteger,
    Boolean,
    Count,
    DateTime,
    Day,
    Decimal,
    Double,
    Duration,
    ExclusiveValueRange,
    Float,
    GregorianDay,
    GregorianTimePeriod,
    GregorianYear,
    GregorianYearMonth,
    InclusiveValueRange,
    Incremental,
    Integer,
    Long,
    Month,
    MonthDay,
    Numeric,
    ObservationalTimePeriod,
    ReportingDay,
    ReportingMonth,
    ReportingQuarter,
    ReportingSemester,
    ReportingTimePeriod,
    ReportingTrimester,
    ReportingWeek,
    ReportingYear,
    Short,
    StandardTimePeriod,
    String,
    Time,
    TimeRange,
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "XHTML")]
    Xhtml,
}

/// ContentConstraintTypeCodeType defines a list of types for a content constraint. A content
/// constraint can state which data is present or which content is allowed for the constraint
/// attachment. If 'Allowed' then the constraint contains the allowed values for attachable
/// object. If 'Actual' then the constraints contains the actual data present for the
/// attachable object.
#[derive(Serialize, Deserialize)]
pub enum ContentConstraintTypeCodeType {
    Actual,
    Allowed,
}

/// UsageStatusType provides a list of enumerated types for indicating whether reporting a
/// given attribute is mandatory or conditional.
#[derive(Serialize, Deserialize)]
pub enum UsageStatusType {
    Conditional,
    Mandatory,
}

/// SimpleDataType restricts BasicComponentDataType to specify the allowable data types for a
/// data structure definition component. The XHTML representation is removed as a possible
/// type.
#[derive(Serialize, Deserialize)]
pub enum SimpleDataType {
    Alpha,
    AlphaNumeric,
    BasicTimePeriod,
    BigInteger,
    Boolean,
    Count,
    DateTime,
    Day,
    Decimal,
    Double,
    Duration,
    ExclusiveValueRange,
    Float,
    GregorianDay,
    GregorianTimePeriod,
    GregorianYear,
    GregorianYearMonth,
    InclusiveValueRange,
    Incremental,
    Integer,
    Long,
    Month,
    MonthDay,
    Numeric,
    ObservationalTimePeriod,
    ReportingDay,
    ReportingMonth,
    ReportingQuarter,
    ReportingSemester,
    ReportingTimePeriod,
    ReportingTrimester,
    ReportingWeek,
    ReportingYear,
    Short,
    StandardTimePeriod,
    String,
    Time,
    TimeRange,
    #[serde(rename = "URI")]
    Uri,
}

/// The type attribute identifies whether then dimension is a measure dimension, the time
/// dimension, or a regular dimension. Although these are all apparent by the element names,
/// this attribute allows for each dimension to be processed independent of its element as
/// well as maintaining the restriction of only one measure and time dimension while still
/// allowing dimension to occur in any order.
///
/// DimensionTypeType enumerates the sub-classes of a dimension.
///
/// The type attribute identifies whether the dimension is a measure dimension, the time
/// dimension, or a regular dimension. Although these are all apparent by the element names,
/// this attribute allows for each dimension to be processed independent of its element as
/// well as maintaining the restriction of only one measure and time dimension while still
/// allowing dimension to occur in any order.
#[derive(Serialize, Deserialize)]
pub enum DimensionTypeType {
    Dimension,
    MeasureDimension,
    TimeDimension,
}

/// TimeDataType restricts SimpleDataType to specify the allowable data types for
/// representing a time value.
#[derive(Serialize, Deserialize)]
pub enum TimeDataType {
    BasicTimePeriod,
    DateTime,
    GregorianDay,
    GregorianTimePeriod,
    GregorianYear,
    GregorianYearMonth,
    ObservationalTimePeriod,
    ReportingDay,
    ReportingMonth,
    ReportingQuarter,
    ReportingSemester,
    ReportingTimePeriod,
    ReportingTrimester,
    ReportingWeek,
    ReportingYear,
    StandardTimePeriod,
    TimeRange,
}

/// SimpleCodeDataType restricts SimpleDataType to specify the allowable data types for a
/// simple code. The possible values are simply Alpha, AlphaNumeric, or Numeric.
#[derive(Serialize, Deserialize)]
pub enum SimpleCodeDataType {
    Alpha,
    AlphaNumeric,
    Numeric,
}

/// TargetObjectDataType restricts DataType to specify the allowable data types for
/// representing a target object value.
#[derive(Serialize, Deserialize)]
pub enum TargetObjectDataType {
    AttachmentConstraintReference,
    DataSetReference,
    IdentifiableReference,
    KeyValues,
}

/// ObjectTypeCodelistType provides an enumeration of all objects outside of the base
/// infomration model class. This includes some abstract object types such as Organsiation
/// and Constraint.
#[derive(Serialize, Deserialize)]
pub enum ObjectTypeCodelistType {
    Agency,
    AgencyScheme,
    Any,
    AttachmentConstraint,
    Attribute,
    AttributeDescriptor,
    Categorisation,
    Category,
    CategoryScheme,
    CategorySchemeMap,
    Code,
    CodeMap,
    Codelist,
    CodelistMap,
    ComponentMap,
    Concept,
    ConceptMap,
    ConceptScheme,
    ConceptSchemeMap,
    Constraint,
    ConstraintTarget,
    ContentConstraint,
    DataConsumer,
    DataConsumerScheme,
    DataProvider,
    DataProviderScheme,
    DataSetTarget,
    DataStructure,
    Dataflow,
    Dimension,
    DimensionDescriptor,
    DimensionDescriptorValuesTarget,
    GroupDimensionDescriptor,
    HierarchicalCode,
    HierarchicalCodelist,
    Hierarchy,
    HybridCodeMap,
    HybridCodelistMap,
    IdentifiableObjectTarget,
    Level,
    MeasureDescriptor,
    MeasureDimension,
    MetadataAttribute,
    MetadataSet,
    MetadataStructure,
    MetadataTarget,
    Metadataflow,
    Organisation,
    OrganisationMap,
    OrganisationScheme,
    OrganisationSchemeMap,
    OrganisationUnit,
    OrganisationUnitScheme,
    PrimaryMeasure,
    Process,
    ProcessStep,
    ProvisionAgreement,
    ReportPeriodTarget,
    ReportStructure,
    ReportingCategory,
    ReportingCategoryMap,
    ReportingTaxonomy,
    ReportingTaxonomyMap,
    ReportingYearStartDay,
    StructureMap,
    StructureSet,
    TimeDimension,
    Transition,
}

/// UnboundedCodeType provides single textual value of "unbounded", for use in OccurentType.
#[derive(Serialize, Deserialize)]
pub enum OccurenceTypeEnum {
    #[serde(rename = "unbounded")]
    Unbounded,
}

/// DataTypeType provides an enumerated list of the types of data formats allowed as the for
/// the representation of an object.
#[derive(Serialize, Deserialize)]
pub enum DataType {
    Alpha,
    AlphaNumeric,
    AttachmentConstraintReference,
    BasicTimePeriod,
    BigInteger,
    Boolean,
    Count,
    DataSetReference,
    DateTime,
    Day,
    Decimal,
    Double,
    Duration,
    ExclusiveValueRange,
    Float,
    GregorianDay,
    GregorianTimePeriod,
    GregorianYear,
    GregorianYearMonth,
    IdentifiableReference,
    InclusiveValueRange,
    Incremental,
    Integer,
    KeyValues,
    Long,
    Month,
    MonthDay,
    Numeric,
    ObservationalTimePeriod,
    ReportingDay,
    ReportingMonth,
    ReportingQuarter,
    ReportingSemester,
    ReportingTimePeriod,
    ReportingTrimester,
    ReportingWeek,
    ReportingYear,
    Short,
    StandardTimePeriod,
    String,
    Time,
    TimeRange,
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "XHTML")]
    Xhtml,
}

/// ToValueType notes whether the value, name, or description of the source value should be
/// used in the target value.
///
/// ToValueTypeType provides an enumeration of available text-equivalents for translation of
/// coded values into textual formats.
#[derive(Serialize, Deserialize)]
pub enum ToValueTypeType {
    Description,
    Name,
    Value,
}
