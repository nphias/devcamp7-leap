use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub timestamp: u64,
    pub anchor_address: Address,
}

impl HolochainEntry for Section {
    fn entry_type() -> String {
        String::from("section")
    }
}

impl Section {
    pub fn new(title: String, timestamp: u64, anchor_address: Address) -> Self {
        Section {
            title: title,
            timestamp: timestamp,
            anchor_address: anchor_address,
        }
    }
}

// Holochain entry definition for Course
pub fn section_entry_def() -> ValidatingEntryType {
    entry!(
        name: Section::entry_type(),
        description: "this is the definition of section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data {
                EntryValidationData::Create { .. } => {
                    Ok(())
                },
                EntryValidationData::Modify { .. } => {
                    Ok(())
                },
                EntryValidationData::Delete { .. } => {
                    Ok(())
                }
            }
        },
        // All links that course should have are defined for CoureAnchor and so this entry doesn't have any
        links: []
    )
}
