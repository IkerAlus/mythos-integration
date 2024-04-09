mod eth_signer;

use subxt::OnlineClient;

use eth_signer::{EthereumSigner, EthereumSignature, AccountId20};
// Metadata that we'll use for our example
#[subxt::subxt(
    runtime_metadata_path = "./metadata/metadata.scale"
)]
mod eth_runtime {}

pub enum EthRuntimeConfig {}

impl subxt::Config for EthRuntimeConfig {
    type Hash = subxt::utils::H256;
    type AccountId = AccountId20;
    type Address = AccountId20;
    type Signature = EthereumSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, subxt::config::substrate::BlakeTwo256>;
    type ExtrinsicParams = subxt::config::SubstrateExtrinsicParams<Self>;
    type AssetId = u32;
}

// This helper makes it easy to use our `eth_signer::AccountId20`'s with generated
// code that expects a generated `eth_runtime::runtime_types::foo::AccountId20` type.
// an alternative is to do some type substitution in the generated code itself, but
// mostly I'd avoid doing that unless absolutely necessary.
impl From<eth_signer::AccountId20> for eth_runtime::runtime_types::account::AccountId20 {
  fn from(val: eth_signer::AccountId20) -> Self {
      Self(val.0)
  }
}

const ALITH: (&str, &str)     = ("02509540919faacf9ab52146c9aa40db68172d83777250b28e4679176e49ccdd9f", "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133");
const BALTHASAR: (&str, &str) = ("033bc19e36ff1673910575b6727a974a9abd80c9a875d41ab3e2648dbfb9e4b518", "8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b");

const URI: &str = "wss://polkadot-mythos-rpc.polkadot.io";

#[tokio::main]
async fn main() {
  let api = OnlineClient::<EthRuntimeConfig>::from_url(URI).await.unwrap();

  let balthasar = EthereumSigner::from_private_key_hex(BALTHASAR.1).unwrap();
  let dest = balthasar.account_id();

  let balance_transfer_tx = eth_runtime::tx().balances().transfer_allow_death(dest.into(), 10_001);

  let alith = EthereumSigner::from_private_key_hex(ALITH.1).unwrap();

  let events = api
      .tx()
      .create_signed(&balance_transfer_tx, &alith, Default::default())
      .await.unwrap();


  println!("Encoded extrinsic: 0x{}", hex::encode(events.encoded()));

  ()
}
