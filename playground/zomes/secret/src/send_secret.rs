use hdk::prelude::*;
use zome_utils::*;

use zome_delivery_types::*;
use zome_delivery_api::*;
use zome_secret_integrity::*;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendSecretInput {
   pub secret_eh: EntryHash,
   pub strategy: DistributionStrategy,
   pub recipients: Vec<AgentPubKey>,
}


/// Return Distribution ActionHash
#[hdk_extern]
pub fn send_secret(input: SendSecretInput) -> ExternResult<ActionHash> {
   debug!("send_secret() START {:?}", input.secret_eh);
   debug!("send_secret() zome_names: {:?}", dna_info()?.zome_names);
   debug!("send_secret() zome_index: {:?}", zome_info()?.id);
   debug!("send_secret()  zome_name: {:?}", zome_info()?.name);

   /// Determine parcel type depending on Entry
   let maybe_secret: ExternResult<Secret> = get_typed_from_eh(input.secret_eh.clone());
   let zome_name =ZomeName::from("secret_integrity");
   let parcel_kind_info = if let Ok(_secret) = maybe_secret {
      ParcelKind::AppEntry(EntryDefIndex::from(get_variant_index::<SecretEntry>(SecretEntryTypes::Secret)?))
   } else {
      ParcelKind::Manifest("secret".to_string())
   };

   let parcel_description = ParcelDescription {
      name: "".to_owned(),
      size: 0,
      zome_origin: zome_name,
      visibility: EntryVisibility::Private,
      kind_info: parcel_kind_info,
   };
   let distribution = DistributeParcelInput {
      recipients: input.recipients,
      strategy: input.strategy,
      parcel_reference: ParcelReference {
         eh: input.secret_eh,
         description: parcel_description,
      },
   };
   debug!("send_secret() calling distribute_parcel() with: {:?}", distribution);
   let response = call_delivery_zome("distribute_parcel", distribution)?;
   // distribute_parcel(distribution)?;
   let ah: ActionHash = decode_response(response)?;
   debug!("send_secret() END");
   Ok(ah)
}
