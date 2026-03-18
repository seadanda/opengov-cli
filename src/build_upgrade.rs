use anyhow::{bail, ensure, Context, Result};
use crate::*;
use clap::Parser as ClapParser;
use std::fs;
use std::path::Path;

/// Generate a single call that will upgrade all system chains in a given network.
#[derive(Debug, ClapParser)]
pub(crate) struct UpgradeArgs {
	/// Network on which to submit the referendum. `polkadot` or `kusama`.
	#[clap(long = "network", short)]
	pub(crate) network: String,

	/// Only include the runtimes explicitly specified.
	#[clap(long = "only")]
	pub(crate) only: bool,

	/// Use local WASM files instead of downloading from GitHub. Files are assumed to already be in
	/// the upgrade directory.
	#[clap(long = "local")]
	pub(crate) local: bool,

	/// The Fellowship release version. Should be semver and correspond to the release published.
	#[clap(long = "relay-version")]
	pub(crate) relay_version: Option<String>,

	/// Optional. The runtime version of Asset Hub to which to upgrade. If not provided, it will use
	/// the Relay Chain's version.
	#[clap(long = "asset-hub")]
	pub(crate) asset_hub: Option<String>,

	/// Optional. The runtime version of Bridge Hub to which to upgrade. If not provided, it will use
	/// the Relay Chain's version.
	#[clap(long = "bridge-hub")]
	pub(crate) bridge_hub: Option<String>,

	/// Optional. The runtime version of Collectives to which to upgrade. If not provided, it will
	/// use the Relay Chain's version.
	#[clap(long = "collectives")]
	pub(crate) collectives: Option<String>,

	/// Optional. The runtime version of Encointer to which to upgrade. If not provided, it will use
	/// the Relay Chain's version.
	#[clap(long = "encointer")]
	pub(crate) encointer: Option<String>,

	/// Optional. The runtime version of People to which to upgrade. If not provided, it will use
	/// the Relay Chain's version.
	#[clap(long = "people")]
	pub(crate) people: Option<String>,

	/// Optional. The runtime version of Coretime to which to upgrade. If not provided, it will use
	/// the Relay Chain's version.
	#[clap(long = "coretime")]
	pub(crate) coretime: Option<String>,

	/// Name of the file to which to write the output. If not provided, a default will be
	/// constructed.
	#[clap(long = "filename")]
	pub(crate) filename: Option<String>,

	/// Some additional call that you want executed on the Relay Chain along with the upgrade.
	#[clap(long = "additional")]
	pub(crate) additional: Option<String>,
}

// The sub-command's "main" function.
pub(crate) async fn build_upgrade(prefs: UpgradeArgs) -> Result<()> {
	// 0. Find out what to do.
	let use_local = prefs.local;
	let upgrade_details = parse_inputs(prefs)?;

	// 1. Download all the Wasm files needed from the release pages (unless using local files).
	if use_local {
		println!("\nUsing local WASM files from {}\n", upgrade_details.directory);
	} else {
		download_runtimes(&upgrade_details).await?;
	}

	// 2. Construct the `authorize_upgrade` call on each chain.
	let authorization_calls = generate_authorize_upgrade_calls(&upgrade_details)?;

	// 3. Construct a `force_batch` call with everything.
	let batch = construct_batch(&upgrade_details, authorization_calls)?;

	// 4. Write this call as a file that can then be passed to `submit_referendum`.
	write_batch(&upgrade_details, batch)?;
	Ok(())
}

fn chain_version(chain: Option<String>, default: Option<String>, only: bool) -> Option<String> {
	// if the user specified a version for this particular chain, use it
	if let Some(v) = chain {
		Some(String::from(v.trim_start_matches('v')))
	} else {
		// if the user only wants to upgrade specific chains, and have not specified this one, then
		// return None so that it will not be added to the batch of upgrades
		if only {
			None
		// otherwise, use the default version
		} else {
			assert!(default.is_some(), "no version specified");
			default
		}
	}
}

// Parse the CLI inputs and return a typed struct with all the details needed.
pub(crate) fn parse_inputs(prefs: UpgradeArgs) -> Result<UpgradeDetails> {
	let mut networks = Vec::new();
	let only = prefs.only;

	if !only {
		ensure!(
			prefs.relay_version.is_some(),
			"--relay-version must be specified unless using --only"
		);
	}
	let relay_version = chain_version(prefs.relay_version, None, only);
	let asset_hub_version = chain_version(prefs.asset_hub, relay_version.clone(), only);
	let bridge_hub_version = chain_version(prefs.bridge_hub, relay_version.clone(), only);
	let people_version = chain_version(prefs.people, relay_version.clone(), only);
	let coretime_version = chain_version(prefs.coretime, relay_version.clone(), only);
	let encointer_version = chain_version(prefs.encointer, relay_version.clone(), only);
	let collectives_version = chain_version(prefs.collectives, relay_version.clone(), only);

	let relay = match prefs.network.to_ascii_lowercase().as_str() {
		"polkadot" => {
			if let Some(v) = relay_version.clone() {
				networks.push(VersionedNetwork { network: Network::Polkadot, version: v });
			}
			if let Some(v) = asset_hub_version.clone() {
				networks.push(VersionedNetwork { network: Network::PolkadotAssetHub, version: v });
			}
			if let Some(v) = collectives_version.clone() {
				networks
					.push(VersionedNetwork { network: Network::PolkadotCollectives, version: v });
			}
			if let Some(v) = bridge_hub_version.clone() {
				networks.push(VersionedNetwork { network: Network::PolkadotBridgeHub, version: v });
			}
			if let Some(v) = people_version.clone() {
				networks.push(VersionedNetwork { network: Network::PolkadotPeople, version: v });
			}
			if let Some(v) = coretime_version.clone() {
				networks.push(VersionedNetwork { network: Network::PolkadotCoretime, version: v });
			}
			Network::Polkadot
		},
		"kusama" => {
			if let Some(v) = relay_version.clone() {
				networks.push(VersionedNetwork { network: Network::Kusama, version: v });
			}
			if let Some(v) = asset_hub_version.clone() {
				networks.push(VersionedNetwork { network: Network::KusamaAssetHub, version: v });
			}
			if let Some(v) = encointer_version.clone() {
				networks.push(VersionedNetwork { network: Network::KusamaEncointer, version: v });
			}
			if let Some(v) = bridge_hub_version.clone() {
				networks.push(VersionedNetwork { network: Network::KusamaBridgeHub, version: v });
			}
			if let Some(v) = people_version.clone() {
				networks.push(VersionedNetwork { network: Network::KusamaPeople, version: v });
			}
			if let Some(v) = coretime_version.clone() {
				networks.push(VersionedNetwork { network: Network::KusamaCoretime, version: v });
			}
			Network::Kusama
		},
		other => bail!("`network` must be `polkadot` or `kusama`, got \"{}\"", other),
	};

	let additional = match prefs.additional {
		Some(c) => {
			let additional_bytes = get_proposal_bytes(c.clone())
				.context("Failed to parse --additional call data")?;
			match relay {
				// This match isn't as intuitive post-ahm, as these are AH calls.
				Network::Polkadot =>
					Some(CallInfo::from_bytes(&additional_bytes, Network::PolkadotAssetHub)),
				Network::Kusama =>
					Some(CallInfo::from_bytes(&additional_bytes, Network::KusamaAssetHub)),
				_ => bail!("`network` must be `polkadot` or `kusama`"),
			}
		},
		None => None,
	};

	// Get a version from one of the args. (This still feels dirty.)
	let version = relay_version.clone().unwrap_or(asset_hub_version.unwrap_or(
		bridge_hub_version.unwrap_or(encointer_version.unwrap_or(collectives_version.unwrap_or(
			coretime_version.unwrap_or(people_version.unwrap_or(String::from("no-version"))),
		))),
	));

	// Set up a directory to store information fetched/written during this program.
	let directory = format!("./upgrade-{}-{}/", &prefs.network, &version);
	let output_file = if let Some(user_filename) = prefs.filename {
		format!("{directory}{user_filename}")
	} else {
		let network = &prefs.network;
		format!("{directory}{network}-{version}.call")
	};

	make_version_directory(directory.as_str())?;

	Ok(UpgradeDetails { relay, networks, directory, output_file, additional })
}

// Create a directory into which to place runtime blobs and the final call data.
fn make_version_directory(dir_name: &str) -> Result<()> {
	if !Path::new(dir_name).is_dir() {
		fs::create_dir_all(dir_name)
			.with_context(|| format!("Failed to create directory: {}", dir_name))?;
	}
	Ok(())
}

// Convert a semver version (e.g. "1.2.3") to an integer runtime version (e.g. 1002003).
fn semver_to_intver(semver: &str) -> String {
	// M.m.p => M_mmm_ppp
	let points =
		semver.bytes().enumerate().filter(|(_, b)| *b == b'.').map(|(i, _)| i).collect::<Vec<_>>();

	assert!(points.len() == 2, "not semver");

	let major = &semver[..points[0]];
	let minor = &semver[points[0] + 1..points[1]];
	let patch = &semver[points[1] + 1..];

	format!("{major}{minor:0>3}{patch:0>3}")
}

// Fetch all the runtime Wasm blobs from a Fellowship release.
async fn download_runtimes(upgrade_details: &UpgradeDetails) -> Result<()> {
	// Relay Form
	// https://github.com/polkadot-fellows/runtimes/releases/download/v1.0.0/polkadot_runtime-v1000000.compact.compressed.wasm
	//
	// Parachains Form
	// https://github.com/polkadot-fellows/runtimes/releases/download/v1.0.0/asset_hub_kusama_runtime-v1000000.compact.compressed.wasm

	println!("\nDownloading runtimes.\n");
	for chain in &upgrade_details.networks {
		let chain_name = chain.network.release_chain_name();
		let runtime_version = semver_to_intver(&chain.version);
		let fname = format!("{chain_name}_runtime-v{runtime_version}.compact.compressed.wasm");

		let version = &chain.version;
		let download_url = format!(
			"https://github.com/polkadot-fellows/runtimes/releases/download/v{version}/{fname}"
		);

		let download_url = download_url.as_str();
		let directory = &upgrade_details.directory;
		let path_name = format!("{directory}{fname}");
		println!("Downloading... {fname}");
		let response = reqwest::get(download_url).await
			.with_context(|| format!("Failed to download {}", fname))?;
		ensure!(
			response.status().is_success(),
			"Failed to download runtime: {} returned HTTP {}. Check that the release version exists at {} and that this runtime is included in the release.",
			fname,
			response.status(),
			download_url,
		);
		let runtime = response.bytes().await
			.with_context(|| format!("Failed to read response bytes for {}", fname))?;
		// todo: we could actually just hash the file, mutate UpgradeDetails, and not write it.
		// saving it may be more convenient anyway though, since someone needs to upload it after
		// the referendum enacts.
		fs::write(&path_name, runtime)
			.with_context(|| format!("Failed to write runtime to {}", path_name))?;
	}
	Ok(())
}

// Read a runtime WASM blob from disk and return its blake2-256 hash.
fn read_and_hash_runtime(directory: &str, chain: &VersionedNetwork) -> Result<[u8; 32]> {
	let runtime_version = semver_to_intver(&chain.version);
	let chain_name = chain.network.release_chain_name();
	let fname = format!("{chain_name}_runtime-v{runtime_version}.compact.compressed.wasm");
	let path = format!("{directory}{fname}");

	let runtime = fs::read(&path)
		.with_context(|| format!("Failed to read runtime file: {}", path))?;
	let runtime_hash = blake2_256(&runtime);
	println!("{} Runtime Hash: 0x{}", chain.network.display_name(), hex::encode(runtime_hash));
	Ok(runtime_hash)
}

// Generate the `authorize_upgrade` calls that will need to execute on each parachain.
fn generate_authorize_upgrade_calls(upgrade_details: &UpgradeDetails) -> Result<Vec<CallInfo>> {
	println!("\nGenerating parachain authorization calls. The runtime hashes are logged if you would like to verify them with srtool.\n");
	let mut authorization_calls = Vec::new();
	for chain in &upgrade_details.networks {
		let runtime_hash = read_and_hash_runtime(&upgrade_details.directory, chain)?;
		let network_call = match chain.network {
			Network::Kusama => {
				use kusama_relay::runtime_types::frame_system::pallet::Call as SystemCall;
				NetworkRuntimeCall::Kusama(KusamaRuntimeCall::System(
					SystemCall::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::KusamaAssetHub => {
				use kusama_asset_hub::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::KusamaAssetHub(KusamaAssetHubRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::KusamaBridgeHub => {
				use kusama_bridge_hub::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::KusamaBridgeHub(KusamaBridgeHubRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::KusamaPeople => {
				use kusama_people::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::KusamaPeople(KusamaPeopleRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::KusamaCoretime => {
				use kusama_coretime::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::KusamaCoretime(KusamaCoretimeRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::KusamaEncointer => {
				use kusama_encointer::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::KusamaEncointer(KusamaEncointerRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::Polkadot => {
				use polkadot_relay::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::Polkadot(PolkadotRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::PolkadotAssetHub => {
				use polkadot_asset_hub::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::PolkadotAssetHub(PolkadotAssetHubRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::PolkadotCollectives => {
				use polkadot_collectives::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::PolkadotCollectives(CollectivesRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::PolkadotBridgeHub => {
				use polkadot_bridge_hub::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::PolkadotBridgeHub(PolkadotBridgeHubRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::PolkadotPeople => {
				use polkadot_people::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::PolkadotPeople(PolkadotPeopleRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
			Network::PolkadotCoretime => {
				use polkadot_coretime::runtime_types::frame_system::pallet::Call;
				NetworkRuntimeCall::PolkadotCoretime(PolkadotCoretimeRuntimeCall::System(
					Call::authorize_upgrade { code_hash: H256(runtime_hash) },
				))
			},
		};
		authorization_calls.push(CallInfo::from_runtime_call(network_call));
	}
	Ok(authorization_calls)
}

// Take the parachain authorization calls and the Relay Chain call, and batch them into one call
// that can be executed on the Relay Chain. The call returned here is the proposal to put to
// referendum.
fn construct_batch(upgrade_details: &UpgradeDetails, calls: Vec<CallInfo>) -> Result<CallInfo> {
	println!("\nBatching calls.");
	match upgrade_details.relay {
		Network::Kusama => construct_kusama_batch(calls, upgrade_details.additional.clone()),
		Network::Polkadot => construct_polkadot_batch(calls, upgrade_details.additional.clone()),
		Network::KusamaAssetHub
		| Network::KusamaBridgeHub
		| Network::KusamaPeople
		| Network::KusamaCoretime
		| Network::KusamaEncointer
		| Network::PolkadotAssetHub
		| Network::PolkadotCollectives
		| Network::PolkadotBridgeHub
		| Network::PolkadotPeople
		| Network::PolkadotCoretime => bail!("Expected relay network, got {:?}", upgrade_details.relay),
	}
}

fn construct_kusama_batch(calls: Vec<CallInfo>, additional: Option<CallInfo>) -> Result<CallInfo> {
	use kusama_asset_hub::runtime_types::pallet_utility::pallet::Call as UtilityCall;

	let mut batch_calls = Vec::new();
	for auth in calls {
		if matches!(auth.network, Network::KusamaAssetHub) {
			batch_calls.push(auth.get_kusama_asset_hub_call().expect("just constructed"));
		} else {
			batch_calls.push(send_as_superuser_kusama(&auth));
		}
	}
	if let Some(a) = additional {
		batch_calls.push(a.get_kusama_asset_hub_call().expect("kusama call"))
	}
	ensure!(!batch_calls.is_empty(), "No calls to batch");
	Ok(match batch_calls.len() {
		1 =>
			CallInfo::from_runtime_call(NetworkRuntimeCall::KusamaAssetHub(batch_calls[0].clone())),
		_ => CallInfo::from_runtime_call(NetworkRuntimeCall::KusamaAssetHub(
			KusamaAssetHubRuntimeCall::Utility(UtilityCall::force_batch { calls: batch_calls }),
		)),
	})
}

fn construct_polkadot_batch(calls: Vec<CallInfo>, additional: Option<CallInfo>) -> Result<CallInfo> {
	use polkadot_asset_hub::runtime_types::pallet_utility::pallet::Call as UtilityCall;

	let mut batch_calls = Vec::new();
	for auth in calls {
		if matches!(auth.network, Network::PolkadotAssetHub) {
			batch_calls.push(auth.get_polkadot_asset_hub_call().expect("just constructed"));
		} else {
			batch_calls.push(send_as_superuser_polkadot(&auth));
		}
	}
	if let Some(a) = additional {
		batch_calls.push(a.get_polkadot_asset_hub_call().expect("polkadot call"))
	}
	ensure!(!batch_calls.is_empty(), "No calls to batch");
	Ok(match batch_calls.len() {
		1 => CallInfo::from_runtime_call(NetworkRuntimeCall::PolkadotAssetHub(
			batch_calls[0].clone(),
		)),
		_ => CallInfo::from_runtime_call(NetworkRuntimeCall::PolkadotAssetHub(
			PolkadotAssetHubRuntimeCall::Utility(UtilityCall::force_batch { calls: batch_calls }),
		)),
	})
}

macro_rules! impl_send_as_superuser {
	($fn_name:ident, $runtime_mod:path, $runtime_call:ident) => {
		fn $fn_name(auth: &CallInfo) -> $runtime_call {
			use $runtime_mod::{
				pallet_xcm::pallet::Call as XcmCall,
				staging_xcm::v5::{
					junction::Junction::Parachain, junctions::Junctions::Here,
					junctions::Junctions::X1, location::Location, Instruction, Xcm,
				},
				xcm::{
					double_encoded::DoubleEncoded, v3::OriginKind, v3::WeightLimit,
					VersionedLocation, VersionedXcm::V5,
				},
			};

			let location = match auth.network.get_para_id() {
				Ok(para_id) => Location { parents: 1, interior: X1([Parachain(para_id)]) },
				Err(_) => Location { parents: 1, interior: Here },
			};

			$runtime_call::PolkadotXcm(XcmCall::send {
				dest: Box::new(VersionedLocation::V5(location)),
				message: Box::new(V5(Xcm(vec![
					Instruction::UnpaidExecution {
						weight_limit: WeightLimit::Unlimited,
						check_origin: None,
					},
					Instruction::Transact {
						origin_kind: OriginKind::Superuser,
						fallback_max_weight: None,
						call: DoubleEncoded { encoded: auth.encoded.clone() },
					},
				]))),
			})
		}
	};
}

impl_send_as_superuser!(
	send_as_superuser_kusama,
	kusama_asset_hub::runtime_types,
	KusamaAssetHubRuntimeCall
);
impl_send_as_superuser!(
	send_as_superuser_polkadot,
	polkadot_asset_hub::runtime_types,
	PolkadotAssetHubRuntimeCall
);

// Write the call needed to disk and provide instructions to the user about how to propose it.
fn write_batch(upgrade_details: &UpgradeDetails, batch: CallInfo) -> Result<()> {
	let fname = upgrade_details.output_file.as_str();
	let mut info_to_write = "0x".to_owned();
	info_to_write.push_str(hex::encode(batch.encoded).as_str());
	fs::write(fname, info_to_write)
		.with_context(|| format!("Failed to write call data to {}", fname))?;

	println!("\nSuccess! The call data was written to {fname}");
	println!("To submit this as a referendum in OpenGov, run:");
	let network = match upgrade_details.relay {
		Network::Kusama => "kusama",
		Network::Polkadot => "polkadot",
		_ => bail!("Expected relay network (polkadot or kusama)"),
	};
	println!("\nopengov-cli submit-referendum \\");
	println!("    --proposal \"{fname}\" \\");
	println!("    --network \"{network}\" --track <\"root\" or \"whitelistedcaller\">");
	Ok(())
}
