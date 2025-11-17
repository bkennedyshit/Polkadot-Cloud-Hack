use std::sync::Arc;

use clap::Parser;
use futures::FutureExt;
use sc_cli::{ChainSpec, Role, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use sp_core::crypto::Ss58AddressFormat;

use reputechain_runtime::{self, opaque::Block, RuntimeApi};

type FullClient = sc_service::TFullClient<Block, RuntimeApi, sc_executor::NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		reputechain_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		reputechain_runtime::native_version()
	}
}

/// Parse command line arguments into service configuration.
#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: sc_cli::RunCmd,
}

/// Possible subcommands of the main binary.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against a specified runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against a specified runtime state. Note: `try-runtime` feature must be
	/// enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"ReputeChain Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2024
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"" | "local" => Box::new(chain_spec::local_testnet_config()?),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&reputechain_runtime::VERSION
	}
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(sp_consensus_aura::sr25519::AuthorityId, sp_consensus_grandpa::AuthorityId)>,
	root_key: sp_core::sr25519::Public,
	endowed_accounts: Vec<sp_core::sr25519::Public>,
	_enable_println: bool,
) -> reputechain_runtime::GenesisConfig {
	reputechain_runtime::GenesisConfig {
		system: reputechain_runtime::SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: reputechain_runtime::BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		aura: reputechain_runtime::AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: reputechain_runtime::GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: reputechain_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}

pub mod chain_spec {
	use super::*;
	use sp_core::{sr25519, Pair, Public};
	use sp_runtime::traits::{IdentifyAccount, Verify};
	use reputechain_runtime::{AccountId, Signature};

	/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
	pub type ChainSpec = sc_service::GenericChainSpec<reputechain_runtime::GenesisConfig>;

	/// Generate a crypto pair from seed.
	pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
		TPublic::Pair::from_string(&format!("//{}", seed), None)
			.expect("static values are valid; qed")
			.public()
	}

	type AccountPublic = <Signature as Verify>::Signer;

	/// Generate an account ID from seed.
	pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
	where
		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
	{
		AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
	}

	/// Generate an Aura authority key.
	pub fn authority_keys_from_seed(s: &str) -> (sp_consensus_aura::sr25519::AuthorityId, sp_consensus_grandpa::AuthorityId) {
		(
			get_from_seed::<sp_consensus_aura::sr25519::AuthorityId>(s),
			get_from_seed::<sp_consensus_grandpa::AuthorityId>(s),
		)
	}

	pub fn development_config() -> Result<ChainSpec, String> {
		let wasm_binary = reputechain_runtime::WASM_BINARY
			.ok_or_else(|| "Development wasm not available".to_string())?;

		Ok(ChainSpec::from_genesis(
			// Name
			"Development",
			// ID
			"dev",
			sc_service::ChainType::Development,
			move || {
				testnet_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice")],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					],
					true,
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			None,
			// Properties
			None,
			// Extensions
			None,
		))
	}

	pub fn local_testnet_config() -> Result<ChainSpec, String> {
		let wasm_binary = reputechain_runtime::WASM_BINARY
			.ok_or_else(|| "Development wasm not available".to_string())?;

		Ok(ChainSpec::from_genesis(
			// Name
			"Local Testnet",
			// ID
			"local_testnet",
			sc_service::ChainType::Local,
			move || {
				testnet_genesis(
					wasm_binary,
					// Initial PoA authorities
					vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
					// Sudo account
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					// Pre-funded accounts
					vec![
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
					],
					true,
				)
			},
			// Bootnodes
			vec![],
			// Telemetry
			None,
			// Protocol ID
			None,
			None,
			// Properties
			None,
			// Extensions
			None,
		))
	}
}

fn main() -> sc_cli::Result<()> {
	let cli = Cli::parse();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } = new_partial(&config)?;
				let aux_revert = Box::new(|client, _, blocks| {
					sc_consensus_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					frame_benchmarking_cli::BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err("Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`.".into())
						}

						cmd.run::<Block, ExecutorDispatch>(config)
					}
					frame_benchmarking_cli::BenchmarkCmd::Block(cmd) => {
						let PartialComponents { client, .. } = new_partial(&config)?;
						cmd.run(client)
					}
					#[cfg(not(feature = "runtime-benchmarks"))]
					_ => Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`.".into()),
					#[cfg(feature = "runtime-benchmarks")]
					_ => Err("Sub-command is not implemented".into()),
				}
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				Ok((cmd.run::<Block, ExecutorDispatch>(config), task_manager))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
	}
}

/// Builds a new service for a full client.
pub fn new_full(config: sc_service::Configuration) -> Result<sc_service::TaskManager, sc_service::Error> {
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (rpc_extensions_builder, _import_setup, rpc_setup),
	} = new_partial(&config)?;

	let mut net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

	let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
		&client.chain_info().genesis_hash,
		&config.chain_spec,
	);

	net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(
		grandpa_protocol_name.clone(),
	));

	let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		import_setup.1.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, tx_handler_controller, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: Some(sc_service::WarpSyncParams::WithProvider(warp_sync)),
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		config,
		backend: backend.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		network: network.clone(),
		rpc_builder: rpc_extensions_builder,
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		system_rpc_tx,
		tx_handler_controller,
		telemetry: None,
	})?;

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			None,
		);

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

		let aura = sc_consensus_aura::start_aura::<sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _, _, _, _, _, _>(
			sc_consensus_aura::StartAuraParams {
				slot_duration,
				client: client.clone(),
				select_chain,
				block_import: rpc_setup,
				proposer_factory,
				create_inherent_data_providers: move |_, ()| async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

					Ok((slot, timestamp))
				},
				force_authoring,
				backoff_authoring_blocks,
				keystore: keystore_container.sync_keystore(),
				sync_oracle: network.clone(),
				justification_sync_link: network.clone(),
				block_proposal_slot_portion: sc_consensus_aura::SlotProportion::new(2f32 / 3f32),
				max_block_proposal_slot_portion: None,
				telemetry: None,
			},
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"aura",
			Some("block-authoring"),
			aura,
		);
	}

	// if the node isn't running as a validator, run the grandpa observer protocol instead.
	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_consensus_grandpa::Config {
			// FIXME #1578 make this available through chainspec
			gossip_duration: std::time::Duration::from_millis(333),
			justification_period: 512,
			name: Some(name),
			observer_enabled: false,
			keystore: Some(keystore_container.sync_keystore()),
			local_role: role,
			telemetry: None,
			protocol_name: grandpa_protocol_name,
		};

		// start the full GRANDPA voter
		// NOTE: unlike in substrate we are currently running the full
		// GRANDPA voter protocol for all full nodes (regardless of whether
		// they're validators or not). at this point the full voter should
		// provide better guarantees of block and vote data availability than
		// the observer.

		// add a custom voting rule to temporarily stop voting for new blocks
		// after the given pause block is finalized and restarting after the
		// given delay.
		let voting_rule = sc_consensus_grandpa::VotingRulesBuilder::default().build();

		let grandpa_params = sc_consensus_grandpa::GrandpaParams {
			config: grandpa_config,
			link: import_setup.1,
			network: network.clone(),
			sync: Arc::new(network),
			voting_rule,
			prometheus_registry,
			shared_voter_state: sc_consensus_grandpa::SharedVoterState::empty(),
			telemetry: None,
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_consensus_grandpa::run_grandpa_voter(grandpa_params)?,
		);
	}

	network_starter.start_network();
	Ok(task_manager)
}

pub fn new_partial(
	config: &sc_service::Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			impl Fn(
				sc_rpc::DenyUnsafe,
				sc_rpc::SubscriptionTaskExecutor,
			) -> Result<jsonrpsee::RpcModule<()>, sc_service::Error>,
			(
				sc_consensus_aura::AuraBlockImport<
					Block,
					FullClient,
					sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
					sp_consensus_aura::sr25519::AuthorityPair,
				>,
				sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
			),
			sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
		),
	>,
	sc_service::Error,
> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = sc_telemetry::TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = sc_executor::NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
		client.clone(),
		&(client.clone() as Arc<_>),
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, sp_consensus_aura::sr25519::AuthorityPair>::new(
		grandpa_block_import.clone(),
		client.clone(),
	);

	let import_queue =
		sc_consensus_aura::import_queue::<sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _>(
			sc_consensus_aura::ImportQueueParams {
				block_import: aura_block_import.clone(),
				justification_import: Some(Box::new(grandpa_block_import.clone())),
				client: client.clone(),
				create_inherent_data_providers: move |_, ()| async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							sc_consensus_aura::slot_duration(&*client)?,
						);

					Ok((slot, timestamp))
				},
				spawner: &task_manager.spawn_essential_handle(),
				registry: config.prometheus_registry(),
				check_for_equivocation: sc_consensus_aura::CheckForEquivocation::Yes,
				telemetry: telemetry.as_ref().map(|x| x.handle()),
				compatibility_mode: sc_consensus_aura::CompatibilityMode::None,
			},
		)?;

	let import_setup = (aura_block_import, grandpa_link);

	let (rpc_extensions_builder, rpc_setup) = {
		let (_, grandpa_link) = &import_setup;

		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = sc_consensus_grandpa::SharedVoterState::empty();
		let rpc_setup = shared_voter_state.clone();

		let finality_proof_provider = sc_consensus_grandpa::FinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		let client = client.clone();
		let pool = transaction_pool.clone();
		let select_chain = select_chain.clone();
		let keystore = keystore_container.sync_keystore();
		let chain_spec = config.chain_spec.cloned_box();

		let rpc_backend = backend.clone();
		let rpc_extensions_builder = move |deny_unsafe, subscription_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: pool.clone(),
				select_chain: select_chain.clone(),
				chain_spec: chain_spec.cloned_box(),
				deny_unsafe,
				babe: None,
				grandpa: crate::rpc::GrandpaDeps {
					shared_voter_state: shared_voter_state.clone(),
					shared_authority_set: shared_authority_set.clone(),
					justification_stream: justification_stream.clone(),
					subscription_executor,
					finality_provider: finality_proof_provider.clone(),
				},
			};

			crate::rpc::create_full(deps).map_err(Into::into)
		};

		(rpc_extensions_builder, rpc_setup)
	};

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		keystore_container,
		select_chain,
		import_queue,
		transaction_pool,
		other: (rpc_extensions_builder, import_setup, rpc_setup),
	})
}

pub mod rpc {
	use std::sync::Arc;

	use jsonrpsee::RpcModule;
	use reputechain_runtime::{opaque::Block, AccountId, Balance, Index};
	use sc_transaction_pool_api::TransactionPool;
	use sp_api::ProvideRuntimeApi;
	use sp_block_builder::BlockBuilder;
	use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
	use sp_consensus::SelectChain;
	use sp_consensus_grandpa::{
		fg_primitives, GrandpaApi, GrandpaCall, NumberFor, SetId, VoterSet,
	};

	/// Light client extra dependencies.
	pub struct LightDeps<C, F, P> {
		/// The client instance to use.
		pub client: Arc<C>,
		/// Transaction pool instance.
		pub pool: Arc<P>,
		/// Remote access to the blockchain (async).
		pub remote_blockchain: Arc<dyn sc_client_api::light::RemoteBlockchain<Block>>,
		/// Fetcher instance.
		pub fetcher: Arc<F>,
	}

	/// Extra dependencies for GRANDPA
	pub struct GrandpaDeps<B> {
		/// Voting round info.
		pub shared_voter_state: sc_consensus_grandpa::SharedVoterState,
		/// Authority set info.
		pub shared_authority_set: sc_consensus_grandpa::SharedAuthoritySet<sp_runtime::generic::Header<u32, sp_runtime::traits::BlakeTwo256>, Block>,
		/// Receives notifications about justification events from Grandpa.
		pub justification_stream: sc_consensus_grandpa::GrandpaJustificationStream<Block>,
		/// Executor to drive the subscription manager in the Grandpa RPC handler.
		pub subscription_executor: sc_rpc::SubscriptionTaskExecutor,
		/// Finality proof provider.
		pub finality_provider: Arc<sc_consensus_grandpa::FinalityProofProvider<B, Block>>,
	}

	/// Full client dependencies.
	pub struct FullDeps<C, P, SC, B> {
		/// The client instance to use.
		pub client: Arc<C>,
		/// Transaction pool instance.
		pub pool: Arc<P>,
		/// The SelectChain Strategy
		pub select_chain: SC,
		/// A copy of the chain spec.
		pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
		/// Whether to deny unsafe calls
		pub deny_unsafe: sc_rpc::DenyUnsafe,
		/// BABE specific dependencies.
		pub babe: Option<()>,
		/// GRANDPA specific dependencies.
		pub grandpa: GrandpaDeps<B>,
	}

	/// Instantiate all full RPC extensions.
	pub fn create_full<C, P, SC, B>(
		deps: FullDeps<C, P, SC, B>,
	) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
	where
		C: ProvideRuntimeApi<Block>,
		C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
		C: Send + Sync + 'static,
		C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
		C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
		C::Api: BlockBuilder<Block>,
		C::Api: GrandpaApi<Block>,
		P: TransactionPool + 'static,
		SC: SelectChain<Block> + 'static,
		B: sc_client_api::Backend<Block> + Send + Sync + 'static,
		B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
	{
		use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
		use sc_consensus_grandpa_rpc::{Grandpa, GrandpaApiServer};
		use substrate_frame_rpc_system::{System, SystemApiServer};

		let mut module = RpcModule::new(());
		let FullDeps { client, pool, select_chain, chain_spec, deny_unsafe, babe: _, grandpa } =
			deps;

		let GrandpaDeps {
			shared_voter_state,
			shared_authority_set,
			justification_stream,
			subscription_executor,
			finality_provider,
		} = grandpa;

		module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
		module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

		// Extend this RPC with a custom API by using the following syntax.
		// `YourRpcStruct` should have a reference to a client, which is needed
		// to call into the runtime.
		// `module.merge(YourRpcTrait::into_rpc(YourRpcStruct::new(ReferenceToClient, ...)))?;`

		// GRANDPA RPC
		module.merge(
			Grandpa::new(
				subscription_executor,
				shared_authority_set.clone(),
				shared_voter_state,
				justification_stream,
				finality_provider,
			)
			.into_rpc(),
		)?;

		Ok(module)
	}
}