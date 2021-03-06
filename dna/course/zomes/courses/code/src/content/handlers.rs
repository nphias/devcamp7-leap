use hdk::error::{ZomeApiError, ZomeApiResult};
use hdk::holochain_persistence_api::cas::content::Address;
use holochain_entry_utils::HolochainEntry;
use hdk::prelude::*;

use super::entry::Content;
use crate::section;
use crate::section::anchor::SECTION_TO_CONTENT_LINK;

pub fn create(
    name: String,
    url: String,
    description: String,
    timestamp: u64,
    section_anchor_address:Address
) -> ZomeApiResult<Address> {
    let latest_section_result = section::handlers::get_latest_section(&section_anchor_address)?;
    match latest_section_result {
        Some((_current_section, _current_section_address)) => {
            let new_content = Content::new(name, url, description, timestamp);
            let new_content_address = hdk::commit_entry(&new_content.entry())?;
            hdk::link_entries(
                &section_anchor_address,
                &new_content_address,
                SECTION_TO_CONTENT_LINK,
                "",
            )?;

            Ok(new_content_address)
        }
        None => {
            return Err(ZomeApiError::from(
                "Can't create a content in deleted section".to_owned(),
            ));
        }
    }
}

pub fn get_contents(section_anchor_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &section_anchor_address,
        LinkMatch::Exactly(SECTION_TO_CONTENT_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn update(
    content_address: Address,
    name: String,
    url: String,
    description: String,
    section_anchor_address: Address
) -> ZomeApiResult<Address> {
    let mut content: Content = hdk::utils::get_as_type(content_address.clone())?;
    content.description = description;
    content.name = name;
    content.url = url;
    // commit updates to the content entry and get it's new address
    let updated_content_address = hdk::update_entry(content.clone().entry(), &content_address)?;

    // remove link to previous version of content
    hdk::remove_link(
        &section_anchor_address,
        &content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )?;

    // create link to the updated version of content
    hdk::link_entries(
        &section_anchor_address,
        &updated_content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )?;

    // return address of the updated content entry
    Ok(updated_content_address)
}

pub fn delete(content_address: Address, section_anchor_address: Address) -> ZomeApiResult<Address> {
    //let content: Content = hdk::utils::get_as_type(content_address.clone())?;

    hdk::remove_link(
        &section_anchor_address,
        &content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )?;

    Ok(content_address)
    // content is reusable in other sections
   // hdk::remove_entry(&content_address)
}
