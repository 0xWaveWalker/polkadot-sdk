// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use crate::traits::{
	AssetExchange, AssetLock, CallDispatcher, ClaimAssets, ConvertOrigin, DropAssets, EventEmitter,
	ExportXcm, FeeManager, HandleHrmpChannelAccepted, HandleHrmpChannelClosing,
	HandleHrmpNewChannelOpenRequest, OnResponse, ProcessTransaction, RecordXcm, ShouldExecute,
	TransactAsset, VersionChangeNotifier, WeightBounds, WeightTrader,
};
use frame_support::{
	dispatch::{GetDispatchInfo, Parameter, PostDispatchInfo},
	traits::{Contains, ContainsPair, Get, PalletsInfoAccess},
};
use sp_runtime::traits::Dispatchable;
use xcm::prelude::*;

/// The trait to parameterize the `XcmExecutor`.
pub trait Config {
	/// The outer call dispatch type.
	type RuntimeCall: Parameter + Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo;

	/// How to send an onward XCM message.
	///
	/// The sender is tasked with returning the assets it needs to pay for delivery fees.
	/// Only one asset should be returned as delivery fees, any other will be ignored by
	/// the executor.
	type XcmSender: SendXcm;

	/// How to emit XCM events.
	type XcmEventEmitter: EventEmitter;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// How to get a call origin from a `OriginKind` value.
	type OriginConverter: ConvertOrigin<<Self::RuntimeCall as Dispatchable>::RuntimeOrigin>;

	/// Combinations of (Asset, Location) pairs which we trust as reserves.
	type IsReserve: ContainsPair<Asset, Location>;

	/// Combinations of (Asset, Location) pairs which we trust as teleporters.
	type IsTeleporter: ContainsPair<Asset, Location>;

	/// A list of (Origin, Target) pairs allowing a given Origin to be substituted with its
	/// corresponding Target pair.
	type Aliasers: ContainsPair<Location, Location>;

	/// This chain's Universal Location.
	type UniversalLocation: Get<InteriorLocation>;

	/// Whether we should execute the given XCM at all.
	type Barrier: ShouldExecute;

	/// The means of determining an XCM message's weight.
	type Weigher: WeightBounds<Self::RuntimeCall>;

	/// The means of purchasing weight credit for XCM execution.
	type Trader: WeightTrader;

	/// What to do when a response of a query is found.
	type ResponseHandler: OnResponse;

	/// The general asset trap - handler for when assets are left in the Holding Register at the
	/// end of execution.
	type AssetTrap: DropAssets;

	/// Handler for asset locking.
	type AssetLocker: AssetLock;

	/// Handler for exchanging assets.
	///
	/// This is used in the executor to swap the asset wanted for fees with the asset needed for
	/// delivery fees.
	type AssetExchanger: AssetExchange;

	/// The handler for when there is an instruction to claim assets.
	type AssetClaims: ClaimAssets;

	/// How we handle version subscription requests.
	type SubscriptionService: VersionChangeNotifier;

	/// Information on all pallets.
	type PalletInstancesInfo: PalletsInfoAccess;

	/// The maximum number of assets we target to have in the Holding Register at any one time.
	///
	/// NOTE: In the worse case, the Holding Register may contain up to twice as many assets as this
	/// and any benchmarks should take that into account.
	type MaxAssetsIntoHolding: Get<u32>;

	/// Configure the fees.
	type FeeManager: FeeManager;

	/// The method of exporting a message.
	type MessageExporter: ExportXcm;

	/// The origin locations and specific universal junctions to which they are allowed to elevate
	/// themselves.
	type UniversalAliases: Contains<(Location, Junction)>;

	/// The call dispatcher used by XCM.
	///
	/// XCM will use this to dispatch any calls. When no special call dispatcher is required,
	/// this can be set to the same type as `Self::Call`.
	type CallDispatcher: CallDispatcher<Self::RuntimeCall>;

	/// The safe call filter for `Transact`.
	///
	/// Use this type to explicitly whitelist calls that cannot undergo recursion. This is a
	/// temporary measure until we properly account for proof size weights for XCM instructions.
	type SafeCallFilter: Contains<Self::RuntimeCall>;

	/// Transactional processor for XCM instructions.
	type TransactionalProcessor: ProcessTransaction;

	/// Allows optional logic execution for the `HrmpNewChannelOpenRequest` XCM notification.
	type HrmpNewChannelOpenRequestHandler: HandleHrmpNewChannelOpenRequest;
	/// Allows optional logic execution for the `HrmpChannelAccepted` XCM notification.
	type HrmpChannelAcceptedHandler: HandleHrmpChannelAccepted;
	/// Allows optional logic execution for the `HrmpChannelClosing` XCM notification.
	type HrmpChannelClosingHandler: HandleHrmpChannelClosing;
	/// Allows recording the last executed XCM (used by dry-run runtime APIs).
	type XcmRecorder: RecordXcm;
}
