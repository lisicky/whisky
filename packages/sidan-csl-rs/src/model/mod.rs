mod action;
mod asset;
mod data;
mod js_vec;
mod protocol;
mod serialized_address;
mod value;
pub use action::*;
pub use asset::*;
pub use data::*;
pub use js_vec::*;
pub use protocol::*;
use serde::{Deserialize, Serialize};
pub use serialized_address::*;
pub use value::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeshTxBuilderBody {
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<Output>,
    pub collaterals: Vec<PubKeyTxIn>,
    pub required_signatures: JsVecString,
    pub reference_inputs: Vec<RefTxIn>,
    pub withdrawals: Vec<Withdrawal>,
    pub mints: Vec<MintItem>,
    pub change_address: String,
    pub change_datum: Option<Datum>,
    pub metadata: Vec<Metadata>,
    pub validity_range: ValidityRange,
    pub certificates: Vec<Certificate>,
    pub signing_key: JsVecString,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub address: String,
    pub amount: Vec<Asset>,
    pub datum: Option<Datum>,
    pub reference_script: Option<ProvidedScriptSource>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidityRange {
    pub invalid_before: Option<u64>,
    pub invalid_hereafter: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TxIn {
    PubKeyTxIn(PubKeyTxIn),
    ScriptTxIn(ScriptTxIn),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefTxIn {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubKeyTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTxIn {
    pub type_: String,
    pub tx_in: TxInParameter,
    pub script_tx_in: ScriptTxInParameter,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxInParameter {
    pub tx_hash: String,
    pub tx_index: u32,
    pub amount: Option<Vec<Asset>>,
    pub address: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptTxInParameter {
    pub script_source: Option<ScriptSource>,
    pub datum_source: Option<DatumSource>,
    pub redeemer: Option<Redeemer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScriptSource {
    ProvidedScriptSource(ProvidedScriptSource),
    InlineScriptSource(InlineScriptSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidedScriptSource {
    pub script_cbor: String,
    pub language_version: LanguageVersion,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineScriptSource {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: String,
    pub language_version: LanguageVersion,
    pub script_size: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LanguageVersion {
    V1,
    V2,
    V3,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DatumSource {
    ProvidedDatumSource(ProvidedDatumSource),
    InlineDatumSource(InlineDatumSource),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvidedDatumSource {
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineDatumSource {
    pub tx_hash: String,
    pub tx_index: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptSourceInfo {
    pub tx_hash: String,
    pub tx_index: u32,
    pub spending_script_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Withdrawal {
    PubKeyWithdrawal(PubKeyWithdrawal),
    PlutusScriptWithdrawal(PlutusScriptWithdrawal),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PubKeyWithdrawal {
    pub address: String,
    pub coin: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlutusScriptWithdrawal {
    pub address: String,
    pub coin: u64,
    pub script_source: Option<ScriptSource>,
    pub redeemer: Option<Redeemer>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MintItem {
    pub type_: String,
    pub policy_id: String,
    pub asset_name: String,
    pub amount: u64,
    pub redeemer: Option<Redeemer>,
    pub script_source: Option<ScriptSource>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Redeemer {
    pub data: String,
    pub ex_units: Budget,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub mem: u64,
    pub steps: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub tag: String,
    pub metadata: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Certificate {
    RegisterPool(RegisterPool),
    RegisterStake(RegisterStake),
    DelegateStake(DelegateStake),
    DeregisterStake(DeregisterStake),
    RetirePool(RetirePool),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterPool {
    pub pool_params: PoolParams,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolParams {
    pub vrf_key_hash: String,
    pub operator: String,
    pub pledge: String,
    pub cost: String,
    pub margin: (u64, u64),
    pub relays: Vec<Relay>,
    pub owners: Vec<String>,
    pub reward_address: String,
    pub metadata: Option<PoolMetadata>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Relay {
    SingleHostAddr(SingleHostAddr),
    SingleHostName(SingleHostName),
    MultiHostName(MultiHostName),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleHostAddr {
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleHostName {
    pub domain_name: String,
    pub port: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiHostName {
    pub domain_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolMetadata {
    pub url: String,
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterStake {
    pub stake_key_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegateStake {
    pub stake_key_hash: String,
    pub pool_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeregisterStake {
    pub stake_key_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetirePool {
    pub pool_id: String,
    pub epoch: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datum {
    pub type_: String, // Currently it is either "Hash" or "Inline"
    pub data: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoInput {
    pub output_index: u32,
    pub tx_hash: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UtxoOutput {
    pub address: String,
    pub amount: Vec<Asset>,
    pub data_hash: Option<String>,
    pub plutus_data: Option<String>,
    pub script_ref: Option<String>,
    pub script_hash: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UTxO {
    pub input: UtxoInput,
    pub output: UtxoOutput,
}
