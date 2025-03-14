// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2020 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! RPC types

mod account_info;
mod block;
mod block_number;
mod bytes;
mod call_request;
mod fee;
mod filter;
mod index;
mod log;
mod receipt;
mod sync;
mod transaction;
mod transaction_request;
mod work;
mod content;
mod inspect;

pub mod pubsub;

use ethereum::TransactionV2 as EthereumTransaction;
use ethereum_types::{H160, H256, U256};
use serde::Serialize;
use std::collections::HashMap;

pub type TransactionMap<T> = HashMap<H160, HashMap<U256, T>>;

#[derive(Debug, Serialize)]
pub struct TxPoolResult<T: Serialize> {
	pub pending: T,
	pub queued: T,
}

pub trait Get {
	fn get(hash: H256, from_address: H160, txn: &EthereumTransaction) -> Self;
}

pub use self::{
	account_info::{AccountInfo, EthAccount, ExtAccountInfo, RecoveredAccount, StorageProof},
	block::{Block, BlockTransactions, Header, Rich, RichBlock, RichHeader},
	block_number::BlockNumber,
	bytes::Bytes,
	call_request::CallRequest,
	fee::{FeeHistory, FeeHistoryCache, FeeHistoryCacheItem},
	filter::{
		Filter, FilterAddress, FilterChanges, FilterPool, FilterPoolItem, FilterType,
		FilteredParams, Topic, VariadicValue,
	},
	index::Index,
	log::Log,
	receipt::Receipt,
	sync::{
		ChainStatus, EthProtocolInfo, PeerCount, PeerInfo, PeerNetworkInfo, PeerProtocolsInfo,
		Peers, PipProtocolInfo, SyncInfo, SyncStatus, TransactionStats,
	},
	transaction::{LocalTransactionStatus, RichRawTransaction, Transaction},
	transaction_request::{TransactionMessage, TransactionRequest},
	work::Work,
	content::TransactionContent,
	inspect::Summary,
};
