use std::marker::PhantomData;

use codec::Encode;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use sp_runtime::MultiSigner;
use subxt::{extrinsic::create_unsigned, ClientBuilder};

use crate::{
    pallets::autonomy::{Payload, UploadResultCall},
    runtime::XPredictRuntime,
};

mod pallets;
mod runtime;

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
    let client = ClientBuilder::<XPredictRuntime>::new()
        .set_url("ws://127.0.0.1:9944")
        .build()
        .await?;
    let public: MultiSigner = pair.public().into();
    let payload = Payload {
        proposal_id: 1,
        result: 1,
        public,
    };
    let encoded_upload_call = client.encode(UploadResultCall {
        payload: payload.clone().into(),
        signature: pair.sign(&(payload.encode())).into(),
        _runtime: PhantomData,
    })?;
    let extrinsic = create_unsigned::<XPredictRuntime>(encoded_upload_call);
    let event_result = client.submit_and_watch_extrinsic(extrinsic).await;
    println!("{:?}", event_result);
    Ok(())
}
