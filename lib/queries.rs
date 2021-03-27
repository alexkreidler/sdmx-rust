use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::string::ToString;

/// MetadataQuery represents a SDMX API metadata query. Agency, resource_id, and item refer to IDs
#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataQuery {
    /// One of: datastructure, metadatastructure, categoryscheme, conceptscheme, codelist,
    /// hierarchicalcodelist, organisationscheme, agencyscheme, dataproviderscheme, dataconsumerscheme,
    /// organisationunitscheme, dataflow, metadataflow, reportingtaxonomy, provisionagreement, structureset, process,
    /// categorisation, contentconstraint, attachmentconstraint, actualconstraint, allowedconstraint, structure,
    /// transformationscheme, rulesetscheme, userdefinedoperatorscheme, customtypescheme, namepersonalisationscheme,
    /// vtlmappingscheme
    resource: String,
    agency: Option<String>,
    resource_id: Option<String>,
    item: Option<String>,
    /// The metadata version
    version: Option<String>,
}

impl MetadataQuery {
    pub fn new<T: ToString>(items: Vec<T>) -> Result<MetadataQuery> {
        if items.len() > 1 && items.len() < 6 {
            Ok(MetadataQuery {
                resource: items[0].to_string(),
                agency: items.get(1).and_then(|f| Some(f.to_string())),
                resource_id: items.get(2).and_then(|f| Some(f.to_string())),
                version: items.get(3).and_then(|f| Some(f.to_string())),
                item: items.get(4).and_then(|f| Some(f.to_string())),
            })
        } else {
            Err(anyhow!("Incorrect number of items"))
        }
    }

    pub fn to_string(&self) -> String {
        // format!(
        //     "{resource}/{agencyID}/{resourceID}/{version}/{itemID}",
        //     resource = self.resource,
        //     agencyID = self.agency.as_deref().unwrap_or(""),
        //     resourceID = self.resource_id.as_deref().unwrap_or(""),
        //     version = self.version.as_deref().unwrap_or(""),
        //     itemID = self.item.as_deref().unwrap_or(""),
        // )

        let parts: Vec<String> = vec![
            self.resource.clone(),
            self.agency.as_deref().unwrap_or("").to_string(),
            self.resource_id.as_deref().unwrap_or("").to_string(),
            self.version.as_deref().unwrap_or("").to_string(),
            self.item.as_deref().unwrap_or("").to_string(),
        ]
        .into_iter()
        .filter(|s| s != "")
        .collect();

        let path = parts.join("/");
        return path;
    }
}
