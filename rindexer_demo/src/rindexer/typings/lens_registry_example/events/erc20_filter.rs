/// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
///
/// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
/// Any manual changes to this file will be overwritten.
use super::erc20_filter_abi_gen::rindexer_erc20_filter_gen::{self, RindexerERC20FilterGen};
use ethers::{
    abi::Address,
    providers::{Http, Provider, RetryClient},
    types::{Bytes, H256},
};
use rindexer_core::{
    async_trait, generate_random_id,
    generator::event_callback_registry::{
        ContractInformation, EventCallbackRegistry, EventInformation, EventResult, FactoryDetails,
        FilterDetails, NetworkContract, TxInformation,
    },
    manifest::yaml::{Contract, ContractDetails},
    AsyncCsvAppender, FutureExt, PostgresClient,
};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::{any::Any, sync::Arc};

pub type ApprovalData = rindexer_erc20_filter_gen::ApprovalFilter;

#[derive(Debug)]
pub struct ApprovalResult {
    pub event_data: ApprovalData,
    pub tx_information: TxInformation,
}

pub type TransferData = rindexer_erc20_filter_gen::TransferFilter;

#[derive(Debug)]
pub struct TransferResult {
    pub event_data: TransferData,
    pub tx_information: TxInformation,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[async_trait]
trait EventCallback {
    async fn call(&self, events: Vec<EventResult>);
}

pub struct EventContext<TExtensions>
where
    TExtensions: Send + Sync,
{
    pub database: Arc<PostgresClient>,
    pub csv: Arc<AsyncCsvAppender>,
    pub extensions: Arc<TExtensions>,
}

// didn't want to use option or none made harder DX
// so a blank struct makes interface nice
pub struct NoExtensions {}
pub fn no_extensions() -> NoExtensions {
    NoExtensions {}
}

type TransferEventCallbackType<TExtensions> = Arc<
    dyn Fn(&Vec<TransferResult>, Arc<EventContext<TExtensions>>) -> BoxFuture<'_, ()> + Send + Sync,
>;

pub struct TransferEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    callback: TransferEventCallbackType<TExtensions>,
    context: Arc<EventContext<TExtensions>>,
}

impl<TExtensions> TransferEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    pub async fn new(
        callback: TransferEventCallbackType<TExtensions>,
        extensions: TExtensions,
    ) -> Self {
        let csv = AsyncCsvAppender::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/ERC20Filter/erc20filter-transfer.csv".to_string());
        if !Path::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/ERC20Filter/erc20filter-transfer.csv").exists() {
            csv.append_header(vec!["contract_address".into(), "from".into(), "to".into(), "value".into(), "tx_hash".into(), "block_number".into(), "block_hash".into()])
                .await
                .unwrap();
        }

        Self {
            callback,
            context: Arc::new(EventContext {
                database: Arc::new(PostgresClient::new().await.unwrap()),
                csv: Arc::new(csv),
                extensions: Arc::new(extensions),
            }),
        }
    }
}

#[async_trait]
impl<TExtensions> EventCallback for TransferEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    async fn call(&self, events: Vec<EventResult>) {
        let events_len = events.len();

        let result: Vec<TransferResult> = events
            .into_iter()
            .filter_map(|item| {
                item.decoded_data
                    .downcast::<TransferData>()
                    .ok()
                    .map(|arc| TransferResult {
                        event_data: (*arc).clone(),
                        tx_information: item.tx_information,
                    })
            })
            .collect();

        if result.len() == events_len {
            (self.callback)(&result, self.context.clone()).await;
        } else {
            panic!("TransferEvent: Unexpected data type - expected: TransferData")
        }
    }
}

pub enum ERC20FilterEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    Transfer(TransferEvent<TExtensions>),
}

impl<TExtensions> ERC20FilterEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    pub fn topic_id(&self) -> &'static str {
        match self {
            ERC20FilterEventType::Transfer(_) => {
                "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
            }
        }
    }

    pub fn event_name(&self) -> &'static str {
        match self {
            ERC20FilterEventType::Transfer(_) => "Transfer",
        }
    }

    fn contract_information(&self) -> Contract {
        Contract {
            name: "ERC20Filter".to_string(),
            details: vec![ContractDetails::new_with_filter(
                "polygon".to_string(),
                FilterDetails {
                    event_name: "Transfer".to_string(),
                    indexed_1: Some(vec![
                        "0x4A1a2197f307222cD67A1762D9A352F64558d9Be".to_string()
                    ]),
                    indexed_2: None,
                    indexed_3: None,
                },
                Some(1446614065.into()),
                None,
                Some(1000),
            )],
            abi: "/Users/joshstevens/code/rindexer/rindexer_demo/abis/erc20-abi.json".to_string(),
            include_events: None,
            reorg_safe_distance: false,
            generate_csv: true,
        }
    }

    fn get_provider(&self, network: &str) -> Arc<Provider<RetryClient<Http>>> {
        if network == "polygon" {
            return super::super::super::networks::get_polygon_provider();
        } else {
            panic!("Network not supported")
        }
    }

    fn contract(&self, network: &str) -> RindexerERC20FilterGen<Arc<Provider<RetryClient<Http>>>> {
        if network == "polygon" {
            let address: Address = "0x0000000000000000000000000000000000000000"
                .parse()
                .unwrap();
            RindexerERC20FilterGen::new(address, Arc::new(self.get_provider(network).clone()))
        } else {
            panic!("Network not supported");
        }
    }

    fn decoder(
        &self,
        network: &str,
    ) -> Arc<dyn Fn(Vec<H256>, Bytes) -> Arc<dyn Any + Send + Sync> + Send + Sync> {
        let contract = self.contract(network);

        match self {
            ERC20FilterEventType::Transfer(_) => Arc::new(move |topics: Vec<H256>, data: Bytes| {
                match contract.decode_event::<TransferData>("Transfer", topics, data) {
                    Ok(filter) => Arc::new(filter) as Arc<dyn Any + Send + Sync>,
                    Err(error) => Arc::new(error) as Arc<dyn Any + Send + Sync>,
                }
            }),
        }
    }

    pub fn register(self, registry: &mut EventCallbackRegistry) {
        let topic_id = self.topic_id();
        let event_name = self.event_name();
        let contract_information = self.contract_information();
        let contract = ContractInformation {
            name: contract_information.name,
            details: contract_information
                .details
                .iter()
                .map(|c| NetworkContract {
                    id: generate_random_id(10),
                    network: c.network.clone(),
                    provider: self.get_provider(&c.network),
                    decoder: self.decoder(&c.network),
                    indexing_contract_setup: c.indexing_contract_setup(),
                    start_block: c.start_block,
                    end_block: c.end_block,
                    polling_every: c.polling_every,
                })
                .collect(),
            abi: contract_information.abi,
            reorg_safe_distance: contract_information.reorg_safe_distance,
        };

        let callback: Arc<dyn Fn(Vec<EventResult>) -> BoxFuture<'static, ()> + Send + Sync> =
            match self {
                ERC20FilterEventType::Transfer(event) => {
                    let event = Arc::new(event);
                    Arc::new(move |result| {
                        let event = event.clone();
                        async move { event.call(result).await }.boxed()
                    })
                }
            };

        registry.register_event(EventInformation {
            indexer_name: "LensRegistryExample".to_string(),
            event_name: event_name.to_string(),
            topic_id: topic_id.to_string(),
            contract,
            callback,
        });
    }
}