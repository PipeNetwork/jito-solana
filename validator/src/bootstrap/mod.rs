use {
    solana_clock::Slot,
    solana_core::validator::{ValidatorConfig, ValidatorStartProgress},
    solana_gossip::{contact_info::ContactInfo, node::Node},
    solana_keypair::Keypair,
    solana_pubkey::Pubkey,
    solana_streamer::socket::SocketAddrSpace,
    std::{
        net::SocketAddr,
        path::Path,
        sync::{Arc, RwLock},
    },
};

pub mod gossip;
pub mod solanacdn;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SnapshotBootstrapMode {
    /// Use SolanaCDN snapshot manifest + HTTP range downloads (Pipe snapshot service).
    SolanaCdnSnapshots,
    /// Use the legacy gossip snapshot-hash discovery + RPC snapshot download flow.
    GossipSnapshots,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RpcBootstrapConfig {
    pub no_genesis_fetch: bool,
    pub no_snapshot_fetch: bool,
    pub only_known_rpc: bool,
    pub max_genesis_archive_unpacked_size: u64,
    pub check_vote_account: Option<String>,
    pub incremental_snapshot_fetch: bool,

    pub snapshot_bootstrap_mode: SnapshotBootstrapMode,

    /// Optional explicit RPC nodes to use for bootstrap instead of gossip discovery.
    pub bootstrap_rpc_addrs: Vec<SocketAddr>,
    /// Optional URL that returns a JSON list of RPC socket addresses for bootstrap.
    pub bootstrap_rpc_addrs_url: Option<String>,

    pub snapshot_manifest_url: String,
    pub snapshot_download_concurrency: usize,
    pub snapshot_download_chunk_size_bytes: u64,
    pub snapshot_download_timeout_ms: u64,
    pub snapshot_download_max_retries: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn rpc_bootstrap(
    node: &Node,
    identity_keypair: &Arc<Keypair>,
    ledger_path: &Path,
    vote_account: &Pubkey,
    authorized_voter_keypairs: Arc<RwLock<Vec<Arc<Keypair>>>>,
    cluster_entrypoints: &[ContactInfo],
    validator_config: &mut ValidatorConfig,
    bootstrap_config: RpcBootstrapConfig,
    do_port_check: bool,
    use_progress_bar: bool,
    maximum_local_snapshot_age: Slot,
    should_check_duplicate_instance: bool,
    start_progress: &Arc<RwLock<ValidatorStartProgress>>,
    minimal_snapshot_download_speed: f32,
    maximum_snapshot_download_abort: u64,
    socket_addr_space: SocketAddrSpace,
) {
    let snapshot_bootstrap_mode = if matches!(
        bootstrap_config.snapshot_bootstrap_mode,
        SnapshotBootstrapMode::SolanaCdnSnapshots
    ) && bootstrap_config
        .snapshot_manifest_url
        .trim()
        .eq_ignore_ascii_case("false")
    {
        SnapshotBootstrapMode::GossipSnapshots
    } else {
        bootstrap_config.snapshot_bootstrap_mode
    };

    match snapshot_bootstrap_mode {
        SnapshotBootstrapMode::SolanaCdnSnapshots => solanacdn::rpc_bootstrap(
            node,
            identity_keypair,
            ledger_path,
            vote_account,
            authorized_voter_keypairs,
            cluster_entrypoints,
            validator_config,
            bootstrap_config,
            do_port_check,
            use_progress_bar,
            maximum_local_snapshot_age,
            should_check_duplicate_instance,
            start_progress,
            minimal_snapshot_download_speed,
            maximum_snapshot_download_abort,
            socket_addr_space,
        ),
        SnapshotBootstrapMode::GossipSnapshots => gossip::rpc_bootstrap(
            node,
            identity_keypair,
            ledger_path,
            vote_account,
            authorized_voter_keypairs,
            cluster_entrypoints,
            validator_config,
            bootstrap_config,
            do_port_check,
            use_progress_bar,
            maximum_local_snapshot_age,
            should_check_duplicate_instance,
            start_progress,
            minimal_snapshot_download_speed,
            maximum_snapshot_download_abort,
            socket_addr_space,
        ),
    }
}
