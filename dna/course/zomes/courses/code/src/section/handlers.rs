use hdk::error::{ZomeApiError, ZomeApiResult};
use hdk::holochain_persistence_api::cas::content::Address;
use holochain_entry_utils::HolochainEntry;

use super::anchor::SectionAnchor;
use super::entry::Section;
use crate::anchor_trait::AnchorTrait;
use crate::course;
use crate::helper;

pub fn create(
    title: String,
    course_anchor_address: Address,
    timestamp: u64,
) -> ZomeApiResult<Address> {
    let latest_course_result = course::handlers::get_latest_course(course_anchor_address)?;
    match latest_course_result {
        Some((_course, _course_address)) => {
            // initialize SectionAnchor instance to represent this particular section
            let section_anchor =
                SectionAnchor::new(title.clone(), course_anchor_address.clone(), timestamp);
            // commit SectionAnchor to DHT
            let section_anchor_address = hdk::commit_entry(&section_anchor.entry())?;

            // create new Section entry
            let new_section = Section::new(title, timestamp, section_anchor_address.clone());
            // commit this entry to DHT and save it's address
            let new_section_address = hdk::commit_entry(&new_section.entry())?;

            // link sectionAnchor to Section entry
            hdk::link_entries(
                &section_anchor_address,
                &new_section_address,
                SectionAnchor::link_type(),
                "".to_owned(),
            )?;

            // add section into the course
            // this is commented because we haven't yet implemented this function in course::handlers
            //course::handlers::add_section(&course_anchor_address, &section_anchor_address)?;
            // SectionAnchor serves as this section's ID so we return it
            Ok(section_anchor_address)
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't create a section in deleted course".to_owned(),
            ));
        }
    }
}

pub fn get_latest_section(
    section_anchor_address: &Address,
) -> ZomeApiResult<Option<(Section, Address)>> {
    helper::get_latest_data_entry::<Section>(section_anchor_address, &SectionAnchor::link_type())
}

// NOTE: this function isn't public because it's only needed in the current module
fn commit_update(
    section: Section,
    previous_section_address: &Address,
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    // commit updated course to DHT and get it's new address
    let new_section_address = hdk::update_entry(section.entry(), previous_section_address)?;

    // remove link to previous version of section
    hdk::remove_link(
        section_anchor_address,
        &previous_section_address,
        SectionAnchor::link_type(),
        "".to_string(),
    )?;

    // create link to new version of section
    hdk::link_entries(
        section_anchor_address,
        &new_section_address,
        SectionAnchor::link_type(),
        "".to_string(),
    )?;

    Ok(section_anchor_address.to_owned())
}

pub fn update(
    title: String,
    // NOTE(e-nastasia): since we have separate methods for section management
    // (add_section and delete_section) we might not need to have sections_addresses
    // here because it leaves us with inconsistent API. This needs further discussion.
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    let latest_section_result = get_latest_section(section_anchor_address)?;
    match latest_section_result {
        Some((mut previous_section, previous_section_address)) => {
            // update this course
            previous_section.title = title;

            commit_update(
                previous_section,
                &previous_section_address,
                section_anchor_address,
            )?;

            // returning address of the course anchor. Sure, it doesn't change, but it makes our API consistent with hdk:: API
            // that always returns address of an updated entry
            return Ok(section_anchor_address.clone());
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't update a deleted section".to_owned(),
            ));
        }
    }
}

pub fn delete(section_anchor_address: Address) -> ZomeApiResult<Address> {
    // retrieve course_anchor entry. If it doesn't exist, we'll fail with error here so we're also validating input
    //let section_anchor: SectionAnchor = hdk::utils::get_as_type(section_anchor_address.clone())?;

    // NOTE: let's try only deleting an anchor! (and don't touch links from anchor to Section entry and Section entry itself)
    // reasons:
    // 1) without it, we won't be able to reach the Section because everywhere we link to section we only use anchor address
    // 2) we'll avoid polluting DHT by new deletion metadata
    hdk::remove_entry(&section_anchor_address)
}
