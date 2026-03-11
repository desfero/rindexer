use alloy::primitives::U64;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use crate::event::callback_registry::{
    EventCallbackRegistryInformation, TraceCallbackRegistryInformation,
};
use crate::events::RindexerEventEmitter;
use crate::RindexerEvent;

/// Progress is stored as basis points (0–10000) representing 0.00%–100.00%.
#[derive(Clone, Debug, Hash)]
pub enum IndexingEventProgressStatus {
    Syncing { progress: u16 },
    Live,
    Completed,
    Failed,
}

impl IndexingEventProgressStatus {
    fn as_str(&self) -> &str {
        match self {
            Self::Syncing { .. } => "SYNCING",
            Self::Live => "LIVE",
            Self::Completed => "COMPLETED",
            Self::Failed => "FAILED",
        }
    }

    pub fn log(&self) -> &str {
        self.as_str()
    }

    pub fn syncing_log() -> ColoredString {
        "SYNCING".green()
    }

    pub fn live_log() -> ColoredString {
        "LIVE".green()
    }

    pub fn completed_log() -> ColoredString {
        "COMPLETED".green()
    }
}

#[derive(Clone, Debug, Hash)]
pub struct IndexingEventProgress {
    pub id: String,
    pub contract_name: String,
    pub event_name: String,
    pub starting_block: U64,
    pub last_synced_block: U64,
    pub syncing_to_block: U64,
    pub network: String,
    pub chain_id: u64,
    pub live_indexing: bool,
    pub status: IndexingEventProgressStatus,
    pub info_log: String,
}

impl IndexingEventProgress {
    #[allow(clippy::too_many_arguments)]
    fn running(
        id: String,
        contract_name: String,
        event_name: String,
        starting_block: U64,
        last_synced_block: U64,
        syncing_to_block: U64,
        network: String,
        chain_id: u64,
        live_indexing: bool,
        info_log: String,
    ) -> Self {
        Self {
            id,
            contract_name,
            event_name,
            starting_block,
            last_synced_block,
            syncing_to_block,
            network,
            chain_id,
            live_indexing,
            status: IndexingEventProgressStatus::Syncing { progress: 0 },
            info_log,
        }
    }
}

/// Opaque handle to shared block-level progress tracking.
pub struct IndexingEventsProgressState {
    events: Mutex<Vec<IndexingEventProgress>>,
    block_progress: Option<Arc<BlockProgressAggregator>>,
}

#[derive(thiserror::Error, Debug)]
pub enum SyncError {
    #[error("Event with id {0} not found")]
    EventNotFound(String),

    #[error("Block number conversion error for total blocks: from {0} to {1}")]
    BlockNumberConversionTotalBlocksError(U64, U64),

    #[error("Block number conversion error for synced blocks: from {0} to {1}")]
    BlockNumberConversionSyncedBlocksError(U64, U64),
}

/// Info needed for block-level progress reporting after releasing the events lock.
struct BlockReport {
    chain_id: u64,
    event_id: String,
    block: U64,
}

impl IndexingEventsProgressState {
    pub(super) async fn monitor_events(
        event_information: &Vec<EventCallbackRegistryInformation>,
        block_progress: Option<Arc<BlockProgressAggregator>>,
    ) -> Arc<IndexingEventsProgressState> {
        let mut events = Vec::new();
        let mut network_latest_cache: HashMap<String, U64> = HashMap::new();

        for event_info in event_information {
            for network_contract in &event_info.contract.details {
                let network = network_contract.network.clone();
                let latest_block_cached = network_latest_cache.get(&network);
                let latest_block = match latest_block_cached {
                    Some(b) => {
                        debug!("Got block for {} from cache", &network);
                        Ok(*b)
                    }
                    None => {
                        let block = network_contract.cached_provider.get_block_number().await;
                        if let Ok(b) = block {
                            network_latest_cache.insert(network, b);
                        }
                        block
                    }
                };

                match latest_block {
                    Ok(latest_block) => {
                        let start_block = network_contract.start_block.unwrap_or(latest_block);
                        let end_block = network_contract.end_block.unwrap_or(latest_block);

                        let chain_id = network_contract.cached_provider.chain.id();

                        if let Some(ref bp) = block_progress {
                            bp.register(chain_id, &network_contract.id, start_block).await;
                        }

                        events.push(IndexingEventProgress::running(
                            network_contract.id.to_string(),
                            event_info.contract.name.clone(),
                            event_info.event_name.to_string(),
                            start_block,
                            start_block,
                            if latest_block > end_block { end_block } else { latest_block },
                            network_contract.network.clone(),
                            chain_id,
                            network_contract.end_block.is_none(),
                            event_info.info_log_name(),
                        ));
                    }
                    Err(e) => {
                        error!(
                            "Failed to get latest block for network {}: {}",
                            network_contract.network, e
                        );
                    }
                }
            }
        }

        Arc::new(Self { events: Mutex::new(events), block_progress })
    }

    pub(super) async fn monitor_traces(
        event_information: &Vec<TraceCallbackRegistryInformation>,
        block_progress: Option<Arc<BlockProgressAggregator>>,
    ) -> Arc<IndexingEventsProgressState> {
        let mut events = Vec::new();

        for event_info in event_information {
            for network_traces in &event_info.trace_information.details {
                let latest_block = network_traces.cached_provider.get_block_number().await;
                match latest_block {
                    Ok(latest_block) => {
                        let start_block = network_traces.start_block.unwrap_or(latest_block);
                        let end_block = network_traces.end_block.unwrap_or(latest_block);

                        let chain_id = network_traces.cached_provider.chain.id();

                        if let Some(ref bp) = block_progress {
                            bp.register(chain_id, &event_info.id, start_block).await;
                        }

                        events.push(IndexingEventProgress::running(
                            event_info.id.to_string(),
                            event_info.contract_name.clone(),
                            event_info.event_name.to_string(),
                            start_block,
                            start_block,
                            if latest_block > end_block { end_block } else { latest_block },
                            network_traces.network.clone(),
                            chain_id,
                            network_traces.end_block.is_none(),
                            event_info.info_log_name(),
                        ));
                    }
                    Err(e) => {
                        error!(
                            "Failed to get latest block for tracing network {}: {}",
                            network_traces.network, e
                        );
                    }
                }
            }
        }

        Arc::new(Self { events: Mutex::new(events), block_progress })
    }

    pub async fn update_last_synced_block(
        &self,
        id: &str,
        new_last_synced_block: U64,
    ) -> Result<(), SyncError> {
        let report = {
            let mut events = self.events.lock().await;
            Self::update_event(&mut events, id, new_last_synced_block)?
        };

        if let Some(report) = report {
            if let Some(ref aggregator) = self.block_progress {
                aggregator.report_progress(report.chain_id, &report.event_id, report.block).await;
            }
        }

        Ok(())
    }

    fn update_event(
        events: &mut Vec<IndexingEventProgress>,
        id: &str,
        new_last_synced_block: U64,
    ) -> Result<Option<BlockReport>, SyncError> {
        for event in events.iter_mut() {
            if event.id == id {
                if let IndexingEventProgressStatus::Syncing { ref mut progress } = event.status {
                    if *progress < 10_000 {
                        if event.syncing_to_block > event.last_synced_block {
                            let total_blocks: u64 = event
                                .syncing_to_block
                                .checked_sub(event.starting_block)
                                .ok_or(SyncError::BlockNumberConversionTotalBlocksError(
                                    event.syncing_to_block,
                                    event.starting_block,
                                ))?
                                .try_into()
                                .map_err(|_| {
                                    SyncError::BlockNumberConversionTotalBlocksError(
                                        event.syncing_to_block,
                                        event.starting_block,
                                    )
                                })?;

                            let blocks_synced: u64 = new_last_synced_block
                                .checked_sub(event.starting_block)
                                .ok_or(SyncError::BlockNumberConversionSyncedBlocksError(
                                    new_last_synced_block,
                                    event.starting_block,
                                ))?
                                .try_into()
                                .map_err(|_| {
                                    SyncError::BlockNumberConversionSyncedBlocksError(
                                        new_last_synced_block,
                                        event.starting_block,
                                    )
                                })?;

                            *progress =
                                ((blocks_synced * 10_000 / total_blocks) as u16).min(10_000);
                        }

                        if new_last_synced_block >= event.syncing_to_block {
                            info!("{}::{} - 100.00% progress", event.info_log, event.network,);
                            event.status = if event.live_indexing {
                                IndexingEventProgressStatus::Live
                            } else {
                                IndexingEventProgressStatus::Completed
                            };
                        } else {
                            info!(
                                "{}::{} - {:.2}% progress",
                                event.info_log,
                                event.network,
                                *progress as f64 / 100.0
                            );
                        }
                    }
                }

                let chain_id = event.chain_id;
                let event_id = event.id.clone();
                event.last_synced_block = new_last_synced_block;

                return Ok(Some(BlockReport { chain_id, event_id, block: new_last_synced_block }));
            }
        }

        Err(SyncError::EventNotFound(id.to_string()))
    }
}

struct NetworkBlockProgress {
    events: HashMap<String, U64>,
    last_emitted_min: U64,
}

struct AggregatorInner {
    networks: HashMap<u64, NetworkBlockProgress>,
}

/// Aggregates block progress across all event processors on each network.
/// Only emits `BlockIndexingComplete` when all events have reached the minimum block.
pub struct BlockProgressAggregator {
    inner: Mutex<AggregatorInner>,
    emitter: RindexerEventEmitter,
}

impl BlockProgressAggregator {
    pub fn new(emitter: RindexerEventEmitter) -> Self {
        Self { inner: Mutex::new(AggregatorInner { networks: HashMap::new() }), emitter }
    }

    async fn register(&self, chain_id: u64, event_id: &str, start_block: U64) {
        let mut inner = self.inner.lock().await;
        let network_progress = inner.networks.entry(chain_id).or_insert_with(|| {
            NetworkBlockProgress { events: HashMap::new(), last_emitted_min: U64::ZERO }
        });
        network_progress.events.insert(event_id.to_string(), start_block);
    }

    async fn report_progress(&self, chain_id: u64, event_id: &str, to_block: U64) {
        let mut inner = self.inner.lock().await;

        let Some(network_progress) = inner.networks.get_mut(&chain_id) else {
            debug!("BlockProgressAggregator: unknown chain_id {}", chain_id);
            return;
        };

        if let Some(block) = network_progress.events.get_mut(event_id) {
            *block = to_block;
        } else {
            debug!("BlockProgressAggregator: unknown event_id {} on chain {}", event_id, chain_id);
            return;
        }

        let min_block = network_progress.events.values().copied().min().unwrap_or(U64::ZERO);

        if min_block > network_progress.last_emitted_min {
            network_progress.last_emitted_min = min_block;
            drop(inner);
            self.emitter.emit(RindexerEvent::BlockIndexingCompleted {
                chain_id,
                block_number: min_block.to::<u64>(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U64;

    use crate::events::{RindexerEventEmitter, RindexerEventStream};

    #[tokio::test]
    async fn test_emits_only_when_min_advances() {
        let stream = RindexerEventStream::new();
        let mut rx = stream.subscribe();
        let emitter = RindexerEventEmitter::from_stream(stream);
        let aggregator = BlockProgressAggregator::new(emitter);

        aggregator.register(1, "event_a", U64::from(0)).await;
        aggregator.register(1, "event_b", U64::from(0)).await;

        // Event A advances to 100, but event B is still at 0 -> no emission
        aggregator.report_progress(1, "event_a", U64::from(100)).await;
        assert!(rx.try_recv().is_err());

        // Event B advances to 50 -> min advances from 0 to 50
        aggregator.report_progress(1, "event_b", U64::from(50)).await;
        let event = rx.try_recv().unwrap();
        match event {
            RindexerEvent::BlockIndexingCompleted { chain_id, block_number } => {
                assert_eq!(chain_id, 1);
                assert_eq!(block_number, 50);
            }
            _ => panic!("Expected BlockIndexingComplete"),
        }

        // Event B advances to 150 -> min advances from 50 to 100 (A is at 100)
        aggregator.report_progress(1, "event_b", U64::from(150)).await;
        let event = rx.try_recv().unwrap();
        match event {
            RindexerEvent::BlockIndexingCompleted { chain_id, block_number } => {
                assert_eq!(chain_id, 1);
                assert_eq!(block_number, 100);
            }
            _ => panic!("Expected BlockIndexingComplete"),
        }
    }

    #[tokio::test]
    async fn test_different_networks_are_independent() {
        let stream = RindexerEventStream::new();
        let mut rx = stream.subscribe();
        let emitter = RindexerEventEmitter::from_stream(stream);
        let aggregator = BlockProgressAggregator::new(emitter);

        aggregator.register(1, "eth_event", U64::from(0)).await;
        aggregator.register(42161, "arb_event", U64::from(0)).await;

        aggregator.report_progress(1, "eth_event", U64::from(100)).await;
        let event = rx.try_recv().unwrap();
        match event {
            RindexerEvent::BlockIndexingCompleted { chain_id, block_number } => {
                assert_eq!(chain_id, 1);
                assert_eq!(block_number, 100);
            }
            _ => panic!("wrong variant"),
        }

        aggregator.report_progress(42161, "arb_event", U64::from(500)).await;
        let event = rx.try_recv().unwrap();
        match event {
            RindexerEvent::BlockIndexingCompleted { chain_id, block_number } => {
                assert_eq!(chain_id, 42161);
                assert_eq!(block_number, 500);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[tokio::test]
    async fn test_different_start_blocks() {
        let stream = RindexerEventStream::new();
        let mut rx = stream.subscribe();
        let emitter = RindexerEventEmitter::from_stream(stream);
        let aggregator = BlockProgressAggregator::new(emitter);

        aggregator.register(1, "event_a", U64::from(0)).await;
        aggregator.register(1, "event_b", U64::from(5000)).await;

        aggregator.report_progress(1, "event_a", U64::from(1000)).await;
        let event = rx.try_recv().unwrap();
        match event {
            RindexerEvent::BlockIndexingCompleted { chain_id, block_number } => {
                assert_eq!(chain_id, 1);
                assert_eq!(block_number, 1000);
            }
            _ => panic!("wrong variant"),
        }
    }
}
