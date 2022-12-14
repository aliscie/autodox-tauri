use ic_kit::candid::{candid_method};
use ic_kit::macros::query;

use ic_stable_memory::{s, utils::ic_types::SPrincipal};
use crate::users::types::UserFiles;
use crate::users::types::Users;
use crate::utils::get_username;
use shared::{id::Id, schema::*};

// #[query]
// #[candid_method(query)]
// pub fn read_element(id: Id) -> Option<ElementTree>{
//     let caller = SPrincipal(ic_cdk::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return None,
//         Some(username) => username
//     };
//     let element_storage_tree = s!(ElementTreeStorage);
//     _get_element_tree(&element_storage_tree, username, id)
// }
//
// #[query]
// #[candid_method(query)]
// pub fn read_files() -> Result<Vec<FileDirectory>, BackendError>{
//     let caller = SPrincipal(ic_cdk::caller());
//     let users = s!(Users);
//     let username = match get_username(caller, &users){
//         None => return Err(BackendError::UserNotRegisted),
//         Some(username) => username
//     };
//     Ok(_get_directories(&username))
// }

#[query]
#[candid_method(query)]
pub fn get_directories() -> Option<FileDirectory> {
    let caller = SPrincipal(ic_cdk::caller());
    let users = s!(Users);
    let username = match get_username(caller, &users) {
        None => return None,
        Some(username) => username,
    };
    let mut user_files: UserFiles = s!(UserFiles);
    user_files.get(&username).map(|s| s.clone())
}
