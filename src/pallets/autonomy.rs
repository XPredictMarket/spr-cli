use std::marker::PhantomData;

use codec::{Decode, Encode};
use sp_runtime::{traits::Verify, MultiSignature};
use subxt::{module, system::System, Call};

#[module]
pub trait Autonomy: System {}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct Payload<Public> {
	pub proposal_id: u32,
	pub result: u32,
	pub public: Public,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UploadResultCall<T: Autonomy> {
	pub payload: Payload<<MultiSignature as Verify>::Signer>,
	pub signature: MultiSignature,
	pub _runtime: PhantomData<T>,
}
