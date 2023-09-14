use hdk::prelude::*;
use zome_utils::*;
use zome_delivery_integrity::*;
use zome_delivery_types::*;


///
#[hdk_extern]
pub fn publish_parcel(input: PublishParcelInput) -> ExternResult<EntryHash> {
   trace!(" START - {}", input.name);
   std::panic::set_hook(Box::new(zome_panic_hook));
   if input.manifest.chunks.is_empty() {
      return error("No chunks in Manifest");
   }
   /// Commit PublicManifest entry
   let manifest_eh = hash_entry(input.manifest.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::PublicManifest(input.manifest.clone()))?;
   /// Determine size
   let last_chunk: ParcelChunk = get_typed_from_eh(input.manifest.chunks.last().unwrap().to_owned())?;
   let size = (input.manifest.chunks.len() - 1) * CHUNK_MAX_SIZE + last_chunk.data.len();
   /// Create Description
   let description = ParcelDescription {
      name: input.name,
      size,
      reference: ParcelReference {
         eh: manifest_eh,
         zome_origin: input.zome_origin,
         visibility: EntryVisibility::Public,
         kind_info: ParcelKind::Manifest(input.data_type),
      }
   };
   /// Commit PublicParcel entry
   let desc_eh = hash_entry(description.clone())?;
   let _ = create_entry_relaxed(DeliveryEntry::PublicParcel(description))?;
   /// Done
   Ok(desc_eh)
}
