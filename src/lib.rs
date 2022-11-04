use crate::management::{
    CanisterSettings, CreateCanisterArgument, InstallCodeArgument, InstallMode, ManagementMethod,
    WithCanisterId,
};
use ic_kit::prelude::*;
mod management;

const ASSET_WASM: &[u8] = include_bytes!("../wasm/assetstorage.wasm.gz");

#[derive(KitCanister)]
#[candid_path("candid.did")]
pub struct FactoryCanister;

#[init]
fn init() {
    let users = vec![ic::caller()];
    ic::swap(users as Users);
}

type Users = Vec<Principal>;

fn is_user() -> Result<(), String> {
    let caller = ic::caller();
    ic::with(Users::clone)
        .contains(&caller)
        .then_some(())
        .ok_or_else(|| format!("User not found {}", caller))
}

#[update(guard = "is_user")]
fn add_user(user: Principal) -> Result<(), String> {
    let mut users = ic::with(Users::clone);
    users.push(user);
    ic::swap(users as Users);
    Ok(())
}

#[update(guard = "is_user")]
fn del_user(user: Principal) -> Result<(), String> {
    let mut users = ic::with(Users::clone);
    users.retain(|u| *u != user);
    ic::swap(users as Users);
    Ok(())
}

#[update(guard = "is_user")]
async fn deploy() -> Result<Principal, String> {
    let caller = ic::caller();

    // Create a new canister id. Sends with 4T cycles, leaving the child canister with 3T.
    let spawn_args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            // set controller to us, and the caller
            controllers: Some(vec![caller, ic::id()]),
            ..CanisterSettings::default()
        }),
    };
    let id = management::CreateCanister::build((spawn_args,))
        .with_payment(4_000_000_000_000)
        .perform_one::<WithCanisterId>()
        .await
        .map_err(|e| format!("Failed to reserve canister id: {}", e))?
        .canister_id;

    let install_args = InstallCodeArgument {
        canister_id: id,
        wasm_module: ASSET_WASM.to_vec(),
        arg: Vec::new(),
        mode: InstallMode::Install,
    };
    management::InstallCode::build((install_args,))
        .perform_one::<()>()
        .await
        .map_err(|e| format!("Failed to install code: {}", e))?;

    // perform post install call to set caller as authorized
    CallBuilder::new(id, "authorize")
        .with_arg(caller)
        .perform_one::<()>()
        .await
        .map_err(|e| format!("Failed to authorize caller for upload: {}", e))?;

    Ok(id)
}

#[update(guard = "is_user")]
async fn upgrade(id: Principal) -> Result<(), String> {
    let install_args = InstallCodeArgument {
        canister_id: id,
        wasm_module: ASSET_WASM.to_vec(),
        arg: Vec::new(),
        mode: InstallMode::Upgrade,
    };
    management::InstallCode::build((install_args,))
        .perform_one::<()>()
        .await
        .map_err(|e| format!("Failed to install code: {}", e))?;

    Ok(())
}

#[pre_upgrade]
fn pre_upgrade() {
    let users = ic::with(Users::clone);
    ic_kit::stable::stable_store((users,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (users,): (Users,) = ic_kit::stable::stable_restore().unwrap();
    ic::swap(users);
}
