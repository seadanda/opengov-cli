pub(super) use parity_scale_codec::Encode as _;
pub(super) use sp_core::blake2_256;
pub(super) use subxt::utils::H256;

// Kusama Chains -----------------------------------------------------------------------------------

#[subxt::subxt(
	runtime_metadata_path = "metadata/kusama.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod kusama_relay {}
pub(super) use kusama_relay::runtime_types::staging_kusama_runtime::{
	governance::origins::pallet_custom_origins::Origin as KusamaOpenGovOrigin,
	OriginCaller as KusamaOriginCaller, RuntimeCall as KusamaRuntimeCall,
};

#[subxt::subxt(
	runtime_metadata_path = "metadata/kusama_asset_hub.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod kusama_asset_hub {}
pub(super) use kusama_asset_hub::runtime_types::asset_hub_kusama_runtime::{
	governance::origins::pallet_custom_origins::Origin as KusamaAssetHubOpenGovOrigin,
	OriginCaller as KusamaAssetHubOriginCaller, RuntimeCall as KusamaAssetHubRuntimeCall,
};

#[subxt::subxt(runtime_metadata_path = "metadata/kusama_bridge_hub.scale")]
pub mod kusama_bridge_hub {}
pub(super) use kusama_bridge_hub::runtime_types::bridge_hub_kusama_runtime::RuntimeCall as KusamaBridgeHubRuntimeCall;

#[subxt::subxt(runtime_metadata_path = "metadata/kusama_encointer.scale")]
pub mod kusama_encointer {}
pub(super) use kusama_encointer::runtime_types::encointer_kusama_runtime::RuntimeCall as KusamaEncointerRuntimeCall;

#[subxt::subxt(runtime_metadata_path = "metadata/kusama_people.scale")]
pub mod kusama_people {}
pub(super) use kusama_people::runtime_types::people_kusama_runtime::RuntimeCall as KusamaPeopleRuntimeCall;

#[subxt::subxt(runtime_metadata_path = "metadata/kusama_coretime.scale")]
pub mod kusama_coretime {}
pub(super) use kusama_coretime::runtime_types::coretime_kusama_runtime::RuntimeCall as KusamaCoretimeRuntimeCall;

// Polkadot Chains ---------------------------------------------------------------------------------

#[subxt::subxt(
	runtime_metadata_path = "metadata/polkadot.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod polkadot_relay {}
pub(super) use polkadot_relay::runtime_types::polkadot_runtime::RuntimeCall as PolkadotRuntimeCall;

#[subxt::subxt(
	runtime_metadata_path = "metadata/polkadot_asset_hub.scale",
	derive_for_all_types = "PartialEq, Clone"
)]
pub mod polkadot_asset_hub {}
pub(super) use polkadot_asset_hub::runtime_types::asset_hub_polkadot_runtime::{
	governance::origins::pallet_custom_origins::Origin as PolkadotAssetHubOpenGovOrigin,
	OriginCaller as PolkadotAssetHubOriginCaller, RuntimeCall as PolkadotAssetHubRuntimeCall,
};

#[subxt::subxt(runtime_metadata_path = "metadata/polkadot_collectives.scale")]
pub mod polkadot_collectives {}
pub(super) use polkadot_collectives::runtime_types::collectives_polkadot_runtime::{
	fellowship::origins::pallet_origins::Origin as FellowshipOrigins,
	RuntimeCall as CollectivesRuntimeCall,
};

#[subxt::subxt(runtime_metadata_path = "metadata/polkadot_bridge_hub.scale")]
pub mod polkadot_bridge_hub {}
pub(super) use polkadot_bridge_hub::runtime_types::bridge_hub_polkadot_runtime::RuntimeCall as PolkadotBridgeHubRuntimeCall;

#[subxt::subxt(runtime_metadata_path = "metadata/polkadot_people.scale")]
pub mod polkadot_people {}
pub(super) use polkadot_people::runtime_types::people_polkadot_runtime::RuntimeCall as PolkadotPeopleRuntimeCall;

#[subxt::subxt(runtime_metadata_path = "metadata/polkadot_coretime.scale")]
pub mod polkadot_coretime {}
pub(super) use polkadot_coretime::runtime_types::coretime_polkadot_runtime::RuntimeCall as PolkadotCoretimeRuntimeCall;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Network {
	Kusama,
	KusamaAssetHub,
	KusamaEncointer,
	KusamaBridgeHub,
	KusamaPeople,
	KusamaCoretime,
	Polkadot,
	PolkadotAssetHub,
	PolkadotCollectives,
	PolkadotBridgeHub,
	PolkadotPeople,
	PolkadotCoretime,
}

impl Network {
	/// Return the `ParaId` of a given network. Returns an error if the network is not a parachain.
	pub(super) fn get_para_id(&self) -> Result<u32, &'static str> {
		use Network::*;
		match &self {
			// Kusama
			Kusama => Err("relay chain"),
			KusamaAssetHub => Ok(1_000),
			KusamaBridgeHub => Ok(1_002),
			KusamaPeople => Ok(1_004),
			KusamaCoretime => Ok(1_005),
			KusamaEncointer => Ok(1_001),
			// Polkadot
			Polkadot => Err("relay chain"),
			PolkadotAssetHub => Ok(1_000),
			PolkadotBridgeHub => Ok(1_002),
			PolkadotCollectives => Ok(1_001),
			PolkadotPeople => Ok(1_004),
			PolkadotCoretime => Ok(1_005),
		}
	}

	/// The chain name used in release artifact filenames (e.g. "asset-hub-kusama").
	pub(super) fn release_chain_name(&self) -> &'static str {
		use Network::*;
		match self {
			Kusama => "kusama",
			Polkadot => "polkadot",
			KusamaAssetHub => "asset-hub-kusama",
			KusamaBridgeHub => "bridge-hub-kusama",
			KusamaPeople => "people-kusama",
			KusamaCoretime => "coretime-kusama",
			KusamaEncointer => "encointer-kusama",
			PolkadotAssetHub => "asset-hub-polkadot",
			PolkadotCollectives => "collectives-polkadot",
			PolkadotBridgeHub => "bridge-hub-polkadot",
			PolkadotPeople => "people-polkadot",
			PolkadotCoretime => "coretime-polkadot",
		}
	}

	/// Human-readable display name for logging (e.g. "Kusama Asset Hub").
	pub(super) fn display_name(&self) -> &'static str {
		use Network::*;
		match self {
			Kusama => "Kusama Relay Chain",
			Polkadot => "Polkadot Relay Chain",
			KusamaAssetHub => "Kusama Asset Hub",
			KusamaBridgeHub => "Kusama Bridge Hub",
			KusamaPeople => "Kusama People",
			KusamaCoretime => "Kusama Coretime",
			KusamaEncointer => "Kusama Encointer",
			PolkadotAssetHub => "Polkadot Asset Hub",
			PolkadotCollectives => "Polkadot Collectives",
			PolkadotBridgeHub => "Polkadot Bridge Hub",
			PolkadotPeople => "Polkadot People",
			PolkadotCoretime => "Polkadot Coretime",
		}
	}

	/// The network ID used in PAPI links (e.g. "kusama_asset_hub").
	pub(super) fn papi_network_id(&self) -> &'static str {
		use Network::*;
		match self {
			Kusama => "kusama",
			Polkadot => "polkadot",
			KusamaAssetHub => "kusama_asset_hub",
			KusamaBridgeHub => "kusama_bridge_hub",
			KusamaPeople => "kusama_people",
			KusamaCoretime => "kusama_coretime",
			KusamaEncointer => "kusama_encointer",
			PolkadotAssetHub => "polkadot_asset_hub",
			PolkadotCollectives => "polkadot_collectives",
			PolkadotBridgeHub => "polkadot_bridge_hub",
			PolkadotPeople => "polkadot_people",
			PolkadotCoretime => "polkadot_coretime",
		}
	}

	/// The default RPC endpoint for this network.
	pub(super) fn rpc_endpoint(&self) -> &'static str {
		use Network::*;
		match self {
			Kusama => "wss%3A%2F%2Fkusama-rpc.dwellir.com",
			Polkadot => "wss%3A%2F%2Fpolkadot-rpc.dwellir.com",
			KusamaAssetHub => "wss%3A%2F%2Fasset-hub-kusama-rpc.dwellir.com",
			KusamaBridgeHub => "wss%3A%2F%2Fbridge-hub-kusama-rpc.dwellir.com",
			KusamaPeople => "wss%3A%2F%2Fpeople-kusama-rpc.dwellir.com",
			KusamaCoretime => "wss%3A%2F%2Fcoretime-kusama-rpc.dwellir.com",
			KusamaEncointer => "wss%3A%2F%2Fencointer-kusama-rpc.dwellir.com",
			PolkadotAssetHub => "wss%3A%2F%2Fasset-hub-polkadot-rpc.dwellir.com",
			PolkadotCollectives => "wss%3A%2F%2Fpolkadot-collectives-rpc.polkadot.io",
			PolkadotBridgeHub => "wss%3A%2F%2Fbridge-hub-polkadot-rpc.dwellir.com",
			PolkadotPeople => "wss%3A%2F%2Fpeople-polkadot-rpc.dwellir.com",
			PolkadotCoretime => "wss%3A%2F%2Fcoretime-polkadot-rpc.dwellir.com",
		}
	}
}

// Info and preferences provided by the user for proposal submission.
pub(super) struct ProposalDetails {
	// The proposal, generated elsewhere and pasted here.
	pub(super) proposal: String,
	// The track to submit on.
	pub(super) track: NetworkTrack,
	// When do you want this to enact. `At(block)` or `After(blocks)`.
	pub(super) dispatch: EnactmentTime,
	// How you would like to view the output.
	pub(super) output: Output,
	// Cutoff length in bytes for printing the output. If too long, it will print the hash of the
	// call you would need to submit so that you can verify before submission.
	pub(super) output_len_limit: u32,
	// Whether or not to group all calls into a batch. Uses `force_batch` in case the account does
	// not have funds for pre-image deposits or is not a fellow.
	pub(super) print_batch: bool,
	// Whether to use light client endpoints in PAPI links (default true).
	pub(super) use_light_client: bool,
	// Whether to use the Polkadot Fellowship (on Collectives) instead of the Kusama Fellowship.
	// Only applicable for Kusama WhitelistedCaller track.
	pub(super) fellowship_on_polkadot: bool,
}

// Info and preferences provided by the user for runtime upgrade construction.
pub(super) struct UpgradeDetails {
	// The Relay Network for this upgrade, Polkadot or Kusama.
	pub(super) relay: Network,
	// All networks to upgrade.
	pub(super) networks: Vec<VersionedNetwork>,
	// The directory into which to write information needed.
	pub(super) directory: String,
	// The filename of the output.
	pub(super) output_file: String,
	// An additional call to be enacted in the same batch as the system upgrade.
	pub(super) additional: Option<CallInfo>,
}

// A network and the version to which it will upgrade.
#[derive(Debug, PartialEq)]
pub(super) struct VersionedNetwork {
	// A network identifier.
	pub(super) network: Network,
	// A runtime version number (i.e. "9430", not "0.9.43").
	pub(super) version: String,
}

// The network and OpenGov track this proposal should be voted on.
pub(super) enum NetworkTrack {
	KusamaRoot,
	Kusama(KusamaAssetHubOpenGovOrigin),
	PolkadotRoot,
	Polkadot(PolkadotAssetHubOpenGovOrigin),
}

// A runtime call wrapped in the network it should execute on.
pub(super) enum NetworkRuntimeCall {
	Kusama(KusamaRuntimeCall),
	KusamaAssetHub(KusamaAssetHubRuntimeCall),
	KusamaBridgeHub(KusamaBridgeHubRuntimeCall),
	KusamaPeople(KusamaPeopleRuntimeCall),
	KusamaCoretime(KusamaCoretimeRuntimeCall),
	KusamaEncointer(KusamaEncointerRuntimeCall),
	Polkadot(PolkadotRuntimeCall),
	PolkadotAssetHub(PolkadotAssetHubRuntimeCall),
	PolkadotCollectives(CollectivesRuntimeCall),
	PolkadotBridgeHub(PolkadotBridgeHubRuntimeCall),
	PolkadotPeople(PolkadotPeopleRuntimeCall),
	PolkadotCoretime(PolkadotCoretimeRuntimeCall),
}

impl NetworkRuntimeCall {
	/// SCALE-encode the inner call and return the bytes.
	pub(super) fn encode_call(&self) -> Vec<u8> {
		use NetworkRuntimeCall::*;
		match self {
			Kusama(c) => c.encode(),
			KusamaAssetHub(c) => c.encode(),
			KusamaBridgeHub(c) => c.encode(),
			KusamaPeople(c) => c.encode(),
			KusamaCoretime(c) => c.encode(),
			KusamaEncointer(c) => c.encode(),
			Polkadot(c) => c.encode(),
			PolkadotAssetHub(c) => c.encode(),
			PolkadotCollectives(c) => c.encode(),
			PolkadotBridgeHub(c) => c.encode(),
			PolkadotPeople(c) => c.encode(),
			PolkadotCoretime(c) => c.encode(),
		}
	}

	/// Return which `Network` this call targets.
	pub(super) fn network(&self) -> Network {
		use NetworkRuntimeCall::*;
		match self {
			Kusama(_) => Network::Kusama,
			KusamaAssetHub(_) => Network::KusamaAssetHub,
			KusamaBridgeHub(_) => Network::KusamaBridgeHub,
			KusamaPeople(_) => Network::KusamaPeople,
			KusamaCoretime(_) => Network::KusamaCoretime,
			KusamaEncointer(_) => Network::KusamaEncointer,
			Polkadot(_) => Network::Polkadot,
			PolkadotAssetHub(_) => Network::PolkadotAssetHub,
			PolkadotCollectives(_) => Network::PolkadotCollectives,
			PolkadotBridgeHub(_) => Network::PolkadotBridgeHub,
			PolkadotPeople(_) => Network::PolkadotPeople,
			PolkadotCoretime(_) => Network::PolkadotCoretime,
		}
	}
}

// How the user would like to see the output of the program.
pub(super) enum Output {
	// Print just the call data (e.g. 0x1234).
	CallData,
	// Print a clickable link to view the decoded call on Polkadot JS Apps UI.
	AppsUiLink,
}

// Local concrete type to use in each runtime's `DispatchTime`
pub(super) enum EnactmentTime {
	At(u32),
	After(u32),
}

// A call or a hash. Used for printing (or rather, to avoid printing large calls).
// The Hash variant is only used when calls exceed the output length limit, which is rare.
#[allow(clippy::large_enum_variant)]
pub(super) enum PreimageOrHash {
	Call(NetworkRuntimeCall),
	Hash([u8; 32]),
}

// All the info associated with a call in the forms you may need it in.
#[derive(Clone)]
pub(super) struct CallInfo {
	pub(super) network: Network,
	pub(super) encoded: Vec<u8>,
	pub(super) hash: [u8; 32],
	pub(super) length: u32,
}

macro_rules! impl_get_call {
	($fn_name:ident, $network_variant:ident, $call_type:ty, $err:expr) => {
		#[allow(dead_code)]
		pub(super) fn $fn_name(&self) -> Result<$call_type, &'static str> {
			match &self.network {
				Network::$network_variant => {
					let bytes = &self.encoded;
					Ok(<$call_type as parity_scale_codec::Decode>::decode(&mut &bytes[..]).unwrap())
				},
				_ => Err($err),
			}
		}
	};
}

impl CallInfo {
	pub(super) fn from_runtime_call(call: NetworkRuntimeCall) -> Self {
		let network = call.network();
		let encoded = call.encode_call();
		let hash = blake2_256(&encoded);
		let length: u32 = (encoded.len()).try_into().unwrap();
		Self { network, encoded, hash, length }
	}

	pub(super) fn from_bytes(encoded: &[u8], network: Network) -> Self {
		let hash = blake2_256(encoded);
		let length = (encoded.len()).try_into().unwrap();
		Self { network, encoded: encoded.to_vec(), hash, length }
	}

	impl_get_call!(get_kusama_call, Kusama, KusamaRuntimeCall, "not a kusama call");
	impl_get_call!(
		get_kusama_asset_hub_call,
		KusamaAssetHub,
		KusamaAssetHubRuntimeCall,
		"not a kusama asset hub call"
	);
	impl_get_call!(
		get_kusama_bridge_hub_call,
		KusamaBridgeHub,
		KusamaBridgeHubRuntimeCall,
		"not a kusama bridge hub call"
	);
	impl_get_call!(
		get_kusama_encointer_call,
		KusamaEncointer,
		KusamaEncointerRuntimeCall,
		"not a kusama encointer call"
	);
	impl_get_call!(
		get_kusama_people_call,
		KusamaPeople,
		KusamaPeopleRuntimeCall,
		"not a kusama people call"
	);
	impl_get_call!(
		get_kusama_coretime_call,
		KusamaCoretime,
		KusamaCoretimeRuntimeCall,
		"not a kusama coretime call"
	);
	impl_get_call!(get_polkadot_call, Polkadot, PolkadotRuntimeCall, "not a polkadot call");
	impl_get_call!(
		get_polkadot_asset_hub_call,
		PolkadotAssetHub,
		PolkadotAssetHubRuntimeCall,
		"not a polkadot asset hub call"
	);
	impl_get_call!(
		get_polkadot_collectives_call,
		PolkadotCollectives,
		CollectivesRuntimeCall,
		"not a polkadot collectives call"
	);
	impl_get_call!(
		get_polkadot_bridge_hub_call,
		PolkadotBridgeHub,
		PolkadotBridgeHubRuntimeCall,
		"not a polkadot bridge hub call"
	);
	impl_get_call!(
		get_polkadot_people_call,
		PolkadotPeople,
		PolkadotPeopleRuntimeCall,
		"not a polkadot people call"
	);
	impl_get_call!(
		get_polkadot_coretime_call,
		PolkadotCoretime,
		PolkadotCoretimeRuntimeCall,
		"not a polkadot coretime call"
	);

	// Take `Self` and a length limit as input. If the call length exceeds the limit, just return
	// its hash. Call length is recomputed and will be 2 bytes longer than the actual preimage
	// length. This is because the call is `preimage.note_preimage(call)`, so the outer pallet/call
	// indices have a length of 2 bytes.
	pub(super) fn create_print_output(&self, length_limit: u32) -> (PreimageOrHash, u32) {
		let print_output = if self.length > length_limit {
			PreimageOrHash::Hash(self.hash)
		} else {
			match &self.network {
				Network::Kusama => {
					let kusama_call = self.get_kusama_call().expect("kusama");
					PreimageOrHash::Call(NetworkRuntimeCall::Kusama(kusama_call))
				},
				Network::KusamaAssetHub => {
					let kusama_asset_hub_call =
						self.get_kusama_asset_hub_call().expect("kusama asset hub");
					PreimageOrHash::Call(NetworkRuntimeCall::KusamaAssetHub(kusama_asset_hub_call))
				},
				Network::Polkadot => {
					let polkadot_call = self.get_polkadot_call().expect("polkadot");
					PreimageOrHash::Call(NetworkRuntimeCall::Polkadot(polkadot_call))
				},
				Network::PolkadotAssetHub => {
					let polkadot_asset_hub_call =
						self.get_polkadot_asset_hub_call().expect("polkadot asset hub");
					PreimageOrHash::Call(NetworkRuntimeCall::PolkadotAssetHub(
						polkadot_asset_hub_call,
					))
				},
				Network::PolkadotCollectives => {
					let collectives_call =
						self.get_polkadot_collectives_call().expect("collectives");
					PreimageOrHash::Call(NetworkRuntimeCall::PolkadotCollectives(collectives_call))
				},
				Network::KusamaBridgeHub => {
					let call = self.get_kusama_bridge_hub_call().expect("kusama bridge hub");
					PreimageOrHash::Call(NetworkRuntimeCall::KusamaBridgeHub(call))
				},
				Network::KusamaPeople => {
					let call = self.get_kusama_people_call().expect("kusama people");
					PreimageOrHash::Call(NetworkRuntimeCall::KusamaPeople(call))
				},
				Network::KusamaCoretime => {
					let call = self.get_kusama_coretime_call().expect("kusama coretime");
					PreimageOrHash::Call(NetworkRuntimeCall::KusamaCoretime(call))
				},
				Network::KusamaEncointer => {
					let call = self.get_kusama_encointer_call().expect("kusama encointer");
					PreimageOrHash::Call(NetworkRuntimeCall::KusamaEncointer(call))
				},
				Network::PolkadotBridgeHub => {
					let call = self.get_polkadot_bridge_hub_call().expect("polkadot bridge hub");
					PreimageOrHash::Call(NetworkRuntimeCall::PolkadotBridgeHub(call))
				},
				Network::PolkadotPeople => {
					let call = self.get_polkadot_people_call().expect("polkadot people");
					PreimageOrHash::Call(NetworkRuntimeCall::PolkadotPeople(call))
				},
				Network::PolkadotCoretime => {
					let call = self.get_polkadot_coretime_call().expect("polkadot coretime");
					PreimageOrHash::Call(NetworkRuntimeCall::PolkadotCoretime(call))
				},
			}
		};
		(print_output, self.length)
	}
}

// The set of calls that some user will need to sign and submit to initiate a referendum.
pub(super) struct ReferendumCalls {
	// `Some` if using the Fellowship to Whitelist a call. The second value is the length of the
	// call, which may be relevant to the print output.
	//
	// ```
	// preimage.note(whitelist.whitelist_call(hash(proposal)));
	// ```
	pub(super) preimage_for_whitelist_call: Option<(PreimageOrHash, u32)>,
	// The preimage for the public referendum. Should always be `Some`. When not using the
	// Whitelist, this will just be the proposal itself. When using the Whitelist, it will be the
	// proposal nested in a call to dispatch via Whitelist. The second value is the length of the
	// call, which may be relevant to the print output.
	//
	// ```
	// // Without Fellowship
	// preimage.note(proposal);
	//
	// // With Fellowship
	// preimage.note(whitelist.dispatch_whitelisted_call_with_preimage(proposal));
	// ```
	pub(super) preimage_for_public_referendum: Option<(PreimageOrHash, u32)>,
	// The actual submission of the Fellowship referendum to Whitelist a call. `None` when not using
	// Whitelist.
	pub(super) fellowship_referendum_submission: Option<NetworkRuntimeCall>,
	// The actual submission of the public referendum. The `proposal` is the proposal itself when
	// not using the Whitelist, or the dispatch call with nested proposal when using the Whitelist.
	pub(super) public_referendum_submission: Option<NetworkRuntimeCall>,
}
