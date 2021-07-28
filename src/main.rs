use std::marker::PhantomData;

use codec::{Decode, Encode};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::{traits::Verify, MultiSigner};
use substrate_subxt::{
    extrinsic::create_unsigned, module, system::System, Call, ClientBuilder, DefaultNodeRuntime,
    Runtime,
};

type Signature = <DefaultNodeRuntime as Runtime>::Signature;

#[module]
pub trait Autonomy: System {}

impl Autonomy for DefaultNodeRuntime {}

#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub struct Payload<Public> {
    pub proposal_id: u32,
    pub result: u32,
    pub public: Public,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct UploadResultCall<T: Autonomy> {
    pub payload: Payload<<Signature as Verify>::Signer>,
    pub signature: Signature,
    pub _runtime: PhantomData<T>,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // phrase generate by subkey tool, see: https://substrate.dev/docs/en/knowledgebase/integrate/subkey
    // install `subkey` `cargo install --force subkey --git https://github.com/paritytech/substrate --version 2.0.1 --locked`
    // just use command `subkey generate`
    let pair = Pair::from_phrase(
        "series jar carbon quiz pigeon extra lion pilot elevator surprise virtual side",
        None, // password
    )
    .map_err(|_| "unsupport phrase")?
    .0;
    let client = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url("ws://127.0.0.1:9944")
        .build()
        .await?;
    let public: MultiSigner = pair.public().into();
    let payload = Payload {
        proposal_id: 1,
        result: 1,
        public,
    };
    let upload_call = UploadResultCall::<DefaultNodeRuntime> {
        payload: payload.clone().into(),
        signature: pair.sign(&(payload.encode())).into(),
        _runtime: PhantomData,
    };
    let encoded_upload_call = client.encode(upload_call)?;
    let extrinsic = create_unsigned::<DefaultNodeRuntime>(encoded_upload_call);
    let event_result = client.submit_and_watch_extrinsic(extrinsic).await;
    println!("{:?}", event_result);
    Ok(())
}
