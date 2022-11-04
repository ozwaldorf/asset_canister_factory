use async_trait::async_trait;
use ic_kit::ic::CallError;
use ic_kit::prelude::*;

/// A method description.
#[async_trait(?Send)]
pub trait ManagementMethod {
    const NAME: &'static str;
    type Argument: CandidType;
    type Response: for<'de> candid::Deserialize<'de> + CandidType;

    #[inline]
    async fn call(arg: Self::Argument) -> Result<Self::Response, CallError> {
        ic::CallBuilder::new(Principal::management_canister(), Self::NAME)
            .with_arg(arg)
            .perform_one()
            .await
    }

    #[inline]
    async fn call_with_payment(
        arg: Self::Argument,
        cycles: u64,
    ) -> Result<Self::Response, CallError> {
        ic::CallBuilder::new(Principal::management_canister(), Self::NAME)
            .with_arg(arg)
            .with_payment(cycles)
            .perform_one()
            .await
    }
}

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct WithCanisterId {
    pub canister_id: Principal,
}

#[derive(Default, Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,
    pub compute_allocation: Option<Nat>,
    pub memory_allocation: Option<Nat>,
    pub freezing_threshold: Option<Nat>,
}

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct DefiniteCanisterSettings {
    pub controllers: Vec<Principal>,
    pub compute_allocation: Nat,
    pub memory_allocation: Nat,
    pub freezing_threshold: Nat,
}

/// Create a canister on the current subnet.
pub struct CreateCanister;

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct CreateCanisterArgument {
    pub settings: Option<CanisterSettings>,
}

impl ManagementMethod for CreateCanister {
    const NAME: &'static str = "create_canister";
    type Argument = CreateCanisterArgument;
    type Response = WithCanisterId;
}

/// Update the settings of a canister.
pub struct UpdateSettings;

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct UpdateSettingsArgument {
    pub canister_id: Principal,
    pub settings: CanisterSettings,
}

impl ManagementMethod for UpdateSettings {
    const NAME: &'static str = "update_settings";
    type Argument = UpdateSettingsArgument;
    type Response = ();
}

/// Installs the given WASM module on the canister.
pub struct InstallCode;

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub enum InstallMode {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "reinstall")]
    Reinstall,
    #[serde(rename = "upgrade")]
    Upgrade,
}

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct InstallCodeArgument {
    pub mode: InstallMode,
    pub canister_id: Principal,
    #[serde(with = "serde_bytes")]
    pub wasm_module: Vec<u8>,
    pub arg: Vec<u8>,
}

impl ManagementMethod for InstallCode {
    const NAME: &'static str = "install_code";
    type Argument = InstallCodeArgument;
    type Response = ();
}

/// Uninstall the code for the given canister.
pub struct UninstallCode;

impl ManagementMethod for UninstallCode {
    const NAME: &'static str = "uninstall_code";
    type Argument = WithCanisterId;
    type Response = ();
}

/// Start the canister.
pub struct StartCanister;

impl ManagementMethod for StartCanister {
    const NAME: &'static str = "start_canister";
    type Argument = WithCanisterId;
    type Response = ();
}

/// Stop the canister.
pub struct StopCanister;

impl ManagementMethod for StopCanister {
    const NAME: &'static str = "stop_canister";
    type Argument = WithCanisterId;
    type Response = ();
}

/// Query the status of a canister.
pub struct CanisterStatus;

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopping")]
    Stopping,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(Deserialize, Debug, Clone, PartialOrd, PartialEq, CandidType)]
pub struct CanisterStatusResponse {
    pub status: Status,
    pub settings: DefiniteCanisterSettings,
    pub module_hash: Option<Vec<u8>>,
    pub memory_size: Nat,
    pub cycles: Nat,
}

impl ManagementMethod for CanisterStatus {
    const NAME: &'static str = "canister_status";
    type Argument = WithCanisterId;
    type Response = (CanisterStatusResponse,);
}

/// Delete the canister.
pub struct DeleteCanister;

impl ManagementMethod for DeleteCanister {
    const NAME: &'static str = "delete_canister";
    type Argument = WithCanisterId;
    type Response = ();
}

/// Deposit the cycles in the call to the given canister's balance.
pub struct DepositCycles;

impl ManagementMethod for DepositCycles {
    const NAME: &'static str = "deposit_cycles";
    type Argument = WithCanisterId;
    type Response = ();
}

/// Return 32 bytes of random data.
pub struct RawRand;

impl ManagementMethod for RawRand {
    const NAME: &'static str = "raw_rand";
    type Argument = ();
    type Response = (Vec<u8>,);
}
