use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::entry::Section;
use crate::anchor_trait::AnchorTrait;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SectionAnchor {
    pub title: String,
    pub anchor_address: Address,
    pub timestamp: u64,
}

impl AnchorTrait for SectionAnchor {
    fn entry_type() -> String {
        String::from("section_anchor")
    }
    fn link_to() -> String {
        Section::entry_type()
    }
    fn link_type() -> String {
        "section_anchor->section".to_owned()
    }
}

impl SectionAnchor {
    pub fn new(title: String, anchor_address: Address, timestamp: u64) -> Self {
        SectionAnchor {
            title: title,
            anchor_address: Address,
            timestamp: timestamp,
        }
    }
}

pub fn section_anchor_def() -> ValidatingEntryType {
    entry!(
        name: SectionAnchor::entry_type(),
        description: "Anchor to the valid section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<CourseAnchor>| {
            match validation_data{
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
        links:[
            // link that connects SectionAnchor to the latest Section entry
            // This is a necessary link that allows access to section data
            to!(
                SectionAnchor::link_to(),
                link_type: SectionAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                   Ok(())
                }
            )
        ]
    )
}
