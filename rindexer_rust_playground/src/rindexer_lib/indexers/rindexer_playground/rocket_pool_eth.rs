#![allow(non_snake_case)]
use std::{path::PathBuf, sync::Arc};

use rindexer::{
    event::callback_registry::EventCallbackRegistry, rindexer_error, rindexer_info,
    EthereumSqlTypeWrapper, PgType, RindexerColorize,
};

use super::super::super::typings::rindexer_playground::events::rocket_pool_eth::{
    no_extensions, ApprovalEvent, RocketPoolETHEventType, TransferEvent,
};

async fn approval_handler(manifest_path: &PathBuf, registry: &mut EventCallbackRegistry) {
    RocketPoolETHEventType::Approval(
        ApprovalEvent::handler(
            |results, context| async move {
                if results.is_empty() {
                    return Ok(());
                }

                let mut postgres_bulk_data: Vec<Vec<EthereumSqlTypeWrapper>> = vec![];
                let mut csv_bulk_data: Vec<Vec<String>> = vec![];
                for result in results.iter() {
                    csv_bulk_data.push(vec![
                        format!("{:?}", result.tx_information.address),
                        format!("{:?}", result.event_data.owner,),
                        format!("{:?}", result.event_data.spender,),
                        result.event_data.value.to_string(),
                        format!("{:?}", result.tx_information.transaction_hash),
                        result.tx_information.block_number.to_string(),
                        result.tx_information.block_hash.to_string(),
                        result.tx_information.network.to_string(),
                        result.tx_information.transaction_index.to_string(),
                        result.tx_information.log_index.to_string(),
                    ]);
                    let data = vec![
                        EthereumSqlTypeWrapper::Address(result.tx_information.address),
                        EthereumSqlTypeWrapper::Address(result.event_data.owner),
                        EthereumSqlTypeWrapper::Address(result.event_data.spender),
                        EthereumSqlTypeWrapper::U256(result.event_data.value),
                        EthereumSqlTypeWrapper::B256(result.tx_information.transaction_hash),
                        EthereumSqlTypeWrapper::U64(result.tx_information.block_number),
                        EthereumSqlTypeWrapper::B256(result.tx_information.block_hash),
                        EthereumSqlTypeWrapper::String(result.tx_information.network.to_string()),
                        EthereumSqlTypeWrapper::U64(result.tx_information.transaction_index),
                        EthereumSqlTypeWrapper::U256(result.tx_information.log_index),
                    ];
                    postgres_bulk_data.push(data);
                }

                if !csv_bulk_data.is_empty() {
                    let csv_result = context.csv.append_bulk(csv_bulk_data).await;
                    if let Err(e) = csv_result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Approval inserting csv data: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                }

                if postgres_bulk_data.is_empty() {
                    return Ok(());
                }

                if postgres_bulk_data.len() > 100 {
                    let result = context
                        .database
                        .bulk_insert_via_copy(
                            "rindexer_playground_rocket_pool_eth.approval",
                            &[
                                "contract_address".to_string(),
                                "owner".to_string(),
                                "spender".to_string(),
                                "value".to_string(),
                                "tx_hash".to_string(),
                                "block_number".to_string(),
                                "block_hash".to_string(),
                                "network".to_string(),
                                "tx_index".to_string(),
                                "log_index".to_string(),
                            ],
                            &postgres_bulk_data
                                .first()
                                .ok_or("No first element in bulk data, impossible")?
                                .iter()
                                .map(|param| param.to_type())
                                .collect::<Vec<PgType>>(),
                            &postgres_bulk_data,
                        )
                        .await;

                    if let Err(e) = result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Approval inserting bulk data via COPY: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                } else {
                    let result = context
                        .database
                        .bulk_insert(
                            "rindexer_playground_rocket_pool_eth.approval",
                            &[
                                "contract_address".to_string(),
                                "owner".to_string(),
                                "spender".to_string(),
                                "value".to_string(),
                                "tx_hash".to_string(),
                                "block_number".to_string(),
                                "block_hash".to_string(),
                                "network".to_string(),
                                "tx_index".to_string(),
                                "log_index".to_string(),
                            ],
                            &postgres_bulk_data,
                        )
                        .await;

                    if let Err(e) = result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Approval inserting bulk data via INSERT: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                }

                rindexer_info!(
                    "RocketPoolETH::Approval - {} - {} events",
                    "INDEXED".green(),
                    results.len(),
                );

                Ok(())
            },
            no_extensions(),
        )
        .await,
    )
    .register(manifest_path, registry)
    .await;
}

async fn transfer_handler(manifest_path: &PathBuf, registry: &mut EventCallbackRegistry) {
    RocketPoolETHEventType::Transfer(
        TransferEvent::handler(
            |results, context| async move {
                if results.is_empty() {
                    return Ok(());
                }

                let mut postgres_bulk_data: Vec<Vec<EthereumSqlTypeWrapper>> = vec![];
                let mut csv_bulk_data: Vec<Vec<String>> = vec![];
                for result in results.iter() {
                    csv_bulk_data.push(vec![
                        format!("{:?}", result.tx_information.address),
                        format!("{:?}", result.event_data.from,),
                        format!("{:?}", result.event_data.to,),
                        result.event_data.value.to_string(),
                        format!("{:?}", result.tx_information.transaction_hash),
                        result.tx_information.block_number.to_string(),
                        result.tx_information.block_hash.to_string(),
                        result.tx_information.network.to_string(),
                        result.tx_information.transaction_index.to_string(),
                        result.tx_information.log_index.to_string(),
                    ]);
                    let data = vec![
                        EthereumSqlTypeWrapper::Address(result.tx_information.address),
                        EthereumSqlTypeWrapper::Address(result.event_data.from),
                        EthereumSqlTypeWrapper::Address(result.event_data.to),
                        EthereumSqlTypeWrapper::U256(result.event_data.value),
                        EthereumSqlTypeWrapper::B256(result.tx_information.transaction_hash),
                        EthereumSqlTypeWrapper::U64(result.tx_information.block_number),
                        EthereumSqlTypeWrapper::B256(result.tx_information.block_hash),
                        EthereumSqlTypeWrapper::String(result.tx_information.network.to_string()),
                        EthereumSqlTypeWrapper::U64(result.tx_information.transaction_index),
                        EthereumSqlTypeWrapper::U256(result.tx_information.log_index),
                    ];
                    postgres_bulk_data.push(data);
                }

                if !csv_bulk_data.is_empty() {
                    let csv_result = context.csv.append_bulk(csv_bulk_data).await;
                    if let Err(e) = csv_result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Transfer inserting csv data: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                }

                if postgres_bulk_data.is_empty() {
                    return Ok(());
                }

                if postgres_bulk_data.len() > 100 {
                    let result = context
                        .database
                        .bulk_insert_via_copy(
                            "rindexer_playground_rocket_pool_eth.transfer",
                            &[
                                "contract_address".to_string(),
                                "from".to_string(),
                                "to".to_string(),
                                "value".to_string(),
                                "tx_hash".to_string(),
                                "block_number".to_string(),
                                "block_hash".to_string(),
                                "network".to_string(),
                                "tx_index".to_string(),
                                "log_index".to_string(),
                            ],
                            &postgres_bulk_data
                                .first()
                                .ok_or("No first element in bulk data, impossible")?
                                .iter()
                                .map(|param| param.to_type())
                                .collect::<Vec<PgType>>(),
                            &postgres_bulk_data,
                        )
                        .await;

                    if let Err(e) = result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Transfer inserting bulk data via COPY: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                } else {
                    let result = context
                        .database
                        .bulk_insert(
                            "rindexer_playground_rocket_pool_eth.transfer",
                            &[
                                "contract_address".to_string(),
                                "from".to_string(),
                                "to".to_string(),
                                "value".to_string(),
                                "tx_hash".to_string(),
                                "block_number".to_string(),
                                "block_hash".to_string(),
                                "network".to_string(),
                                "tx_index".to_string(),
                                "log_index".to_string(),
                            ],
                            &postgres_bulk_data,
                        )
                        .await;

                    if let Err(e) = result {
                        rindexer_error!(
                            "RocketPoolETHEventType::Transfer inserting bulk data via INSERT: {:?}",
                            e
                        );
                        return Err(e.to_string());
                    }
                }

                rindexer_info!(
                    "RocketPoolETH::Transfer - {} - {} events",
                    "INDEXED".green(),
                    results.len(),
                );

                Ok(())
            },
            no_extensions(),
        )
        .await,
    )
    .register(manifest_path, registry)
    .await;
}
pub async fn rocket_pool_eth_handlers(
    manifest_path: &PathBuf,
    registry: &mut EventCallbackRegistry,
) {
    approval_handler(manifest_path, registry).await;

    transfer_handler(manifest_path, registry).await;
}
