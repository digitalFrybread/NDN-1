use hex_literal::hex;
use node_primitives::*;
use node_template_runtime::{
	constants::currency::*, opaque::SessionKeys, BabeConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, MaxNominations,
	SessionConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	BABE_GENESIS_EPOCH_CONFIG, wasm_binary_unwrap,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
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
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
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
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		None,
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
<<<<<<< HEAD
		"Smartrib3 ndn",
=======
		"Substrate ndn",
>>>>>>> b311f291e24dc5d8145a48f551dcca41ae2304e2
		"ndn_network",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		None,
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			
				// 5H6bwRpu1HHoN4wi8KeQRz2swGY7yMMnzAEPp4BLM6Dt1JWM
				hex!["de9fbc8e60ab7bf86c7976ea60fea36bfd32d0247a7c49f2fd63f37a2da9833c"].into(),
				// 5CD31TTD8QEx85ukV3fUWH5gP94ssfdyfcmFLGgi5DBVEeW2
				hex!["065a6ed8db6de27eb9daa16e001939ec69fbf6a70a403b63a48b0237497ff349"].into(),
				// 5HeauZtbYLEdxB5ZMM6LitSmMGnJhUe2HY5MZV5mBAAMztfJ
				hex!["f7041d7c3d241509a14e7e3883f53c8f5ba8926764a9915d3e7016d72a71c2a9"].unchecked_into(),
				// 5ERHU5S45Z8bUUMuhMbF6UQRTzSq8JSui8oScqEc1zT4Ky5t
				hex!["682a849b1869ba0d3a88acab00f0b43bd2fb1da4a67ac2929d5a417cf43c7b0f"].unchecked_into(),
				// 5CUcvyZb6Kavn288xeroARkQK2YWG6YUuM7UVSvnTUHUbNas
				hex!["123d4ee5fb50ef27fc4861dbd8557496fdb97bcf832b95ed7b084f300e526d1e"].unchecked_into(),
				
				
		),
		(
               // 5GpEr1YaFPqsyEERdiKqz9vJABoxmbKi217pwuFoRCPSDFLE
               hex!["d224cda3bf119f955a338faad375453e32875fda28c0f422ff6cf8cbac815a5c"].into(),
               // 5Fh55Yt8qkHcDeJjUuqfku9FfGSEfvGkJ49wob6GfCVKMfiZ
               hex!["a070f9911116141dee3e18c55053ebe4243a42d3fb55cf1187205ae4d002410b"].into(),
               // 5DKK2NCP2KtJvptydxSGcANCqUTgeuN2V3sHSygQZ1DVp7Kb
               hex!["37600d6dd603cc13a218d547cd1bf714ea104907b441ada6d554f0589fc4bcd8"].unchecked_into(),
               // 5Dtp2ojbA8NmWcn8dvvJtGZA7Kk9qtGxDbxT4AtRqkbfRzNb
               hex!["50eccc8f1f79a0c40ee81719067dd4336a8ef61a612de51c88d3df72b0fa1e2b"].unchecked_into(),
               // 5CqY7ZQNKDHtNDtxu6S51ybxr3HYXpuy5qJBQeWFNnNQHf7A
               hex!["22313e94308c1c073076c642d04eb3ae032cc80ac6892e9c1ed3d605dd4ac76d"].unchecked_into(),
		),
		(
			   // 5CVRQVPzjVngpzYT1Hd5ozxfjUVGGebEHKZHvFHxHZRyv4Xj
			   hex!["12d9c142d38adb92e64217b19a5556718c498e2d70675374ee87deacad08ad11"].into(),
			   // 5DZWE3XHnorTReNvbXqLxCXSg8TeeMKK4afGy5xd7SzjcwqN 
			   hex!["423331f92ae770f67eca55836add7d0a302e1648eae3fa948fcf22a8ebee5957"].into(),
		       // 5DEAswn7dNkLjd5MMQM9rFFjrLu4kchLXpDqBVExJNno4nZi 
			   hex!["33746802210a9f9e2191035fd32daca27a8787b1d402cb80c94e395b7e84df39"].unchecked_into(),
			   // 5ERnFELe4BXQKvSb94N9g1rA5nMJxZ4D1yCWgLobaRYxMmju 
			   hex!["688b6524ae4d84ce18dcf86c17dab2b7e1ddfbe3a890da8bed9e3b03b85abc6a"].unchecked_into(),
			   // 5HYAAyqKZpni1pJomHimFA3Cx8XQ8UikK8KNtPfcJuCuykUD
			   hex!["f21d613ad05c6a475b3cb224d01ddaed0a6fa514979a3265b9621964c791cb08"].unchecked_into(),
		),
		(
			  // 5DqhNzgHNZUgXV3BFheAro6X4xnvSxckUiy61bjzAeX22Mgc
              hex!["4e8caba5289fdd5dceaafdadc5ee303f0548b7681e6ee5d6d2fcb6a0febfc15b"].into(),
              // 5GWWdpb1FEtLKmcsRD9vQ4YD1ei9ZDv3Fn65iDfUVgHDCUo7
              hex!["c49f8a1390f51c3c67c4a2422707e8a505484c45a16d168d227bcb046692d372"].into(),
              // 5Fe9tvZX6bJXbu4SQk3jFVLkNzNXNtqjfNLJp1ZmAmmCMZ7G
             hex!["9e377412fc9c0604a290b839c690956ce41a97704b4b532da86ee4aba9804bd4"].unchecked_into(),
             // 5CK2fd7EhJcrNNktHU6hCeYb2z7osh4jWR5nTZsFtKu8dS57
            hex!["0aecc11943c291dd03d5decfbb3dde6086816a166d7f398d509a864c0b902956"].unchecked_into(),
            // 5HpFP51Kp6ZXNCQEgZPzhmw42SeQtPMdL9Qwr6MBFQz9Rxsg
            hex!["fe62d6e49b047ea64cfe67680d88ba9027880f7f50ce54fe20d243f4b642a876"].unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5GW1p1K6YYoLLP1fTRTU5WdahktnLDgCSbtjjsG94LhFKMyX
		"c43e81fe6d22df10d20292abfc29ce6de948cf2c19d674eed19b7a7fb5a63245"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
