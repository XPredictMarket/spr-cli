use crate::pallets::autonomy::Autonomy;
use codec::{Decode, Encode};
use sp_runtime::{
	generic::Header,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiAddress, MultiSignature, OpaqueExtrinsic,
};
use subxt::{
	balances::{AccountData, Balances, BalancesEventTypeRegistry},
	extrinsic::DefaultExtra,
	sudo::{Sudo, SudoEventTypeRegistry},
	system::{System, SystemEventTypeRegistry},
	EventTypeRegistry,
};

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type BalanceOf<T> = <T as Balances>::Balance;
pub type Hash = sp_core::H256;
pub type Index = u32;
pub type Moment = u64;

pub type Balance = u128;
pub type ProposalId = u32;
pub type VersionId = u32;
pub type CategoryId = u32;
pub type BlockNumber = u32;
pub type ChainId = u32;
pub type CurrencyId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub enum RulerModule {
	PlatformDividend,
	CrossChainBurn,
	NotUsed,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XPredictRuntime;

impl subxt::Runtime for XPredictRuntime {
	type Extra = DefaultExtra<Self>;
	type Signature = Signature;

	fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
		event_type_registry.with_system();
		event_type_registry.with_balances();
		event_type_registry.with_sudo();
		subxt::register_default_type_sizes(event_type_registry);
		event_type_registry.register_type_size::<AccountId>("T::AccountId");
		event_type_registry.register_type_size::<BlockNumber>("T::BlockNumber");
		event_type_registry.register_type_size::<Moment>("MomentOf<T>");
		event_type_registry.register_type_size::<Balance>("BalanceOf<T>");
		event_type_registry.register_type_size::<Balance>("T::Balance");
		event_type_registry.register_type_size::<CurrencyId>("T::CurrencyId");
		event_type_registry.register_type_size::<CurrencyId>("CurrencyIdOf<T>");
		event_type_registry.register_type_size::<ProposalId>("T::ProposalId");
		event_type_registry.register_type_size::<ProposalId>("ProposalIdOf<T>");
		event_type_registry.register_type_size::<VersionId>("T::VersionId");
		event_type_registry.register_type_size::<VersionId>("VersionIdOf<T>");
		event_type_registry.register_type_size::<CategoryId>("T::CategoryId");
		event_type_registry.register_type_size::<CategoryId>("CategoryIdOf<T>");
		event_type_registry.register_type_size::<ChainId>("T::ChainId");
		event_type_registry.register_type_size::<RulerModule>("RulerModule");
	}
}

impl System for XPredictRuntime {
	type AccountData = AccountData<BalanceOf<Self>>;
	type AccountId = AccountId;
	type Address = MultiAddress<AccountId, u32>;
	type BlockNumber = BlockNumber;
	type Extrinsic = OpaqueExtrinsic;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = Header<Self::BlockNumber, BlakeTwo256>;
	type Index = Index;
}

impl Balances for XPredictRuntime {
	type Balance = Balance;
}

impl Sudo for XPredictRuntime {}

impl Autonomy for XPredictRuntime {}
