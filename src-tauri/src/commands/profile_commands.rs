use crate::models::profile::Profile;
use crate::services::profile_service;

/// Reads every Zen profile.
///
/// Returns `Err` only when `profiles.ini` exists but cannot be read or parsed.
/// A file that does not exist yet is not an error — it yields an empty list,
/// which the frontend renders as its "no profiles yet" state.
#[tauri::command]
pub fn get_profiles() -> Result<Vec<Profile>, String> {
    profile_service::get_profiles()
}

#[tauri::command]
pub fn launch_profile(profile_id: String) -> Result<(), String> {
    profile_service::launch_profile(profile_id)
}

/// Creates a new Zen Browser profile with the given name.
/// Returns the newly created Profile on success or an error message on failure.
#[tauri::command]
pub fn create_profile(name: String) -> Result<Profile, String> {
    profile_service::create_profile(&name)
}
