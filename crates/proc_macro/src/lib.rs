#![cfg(not(target_arch = "wasm32"))]

extern crate proc_macro;
use proc_macro::{TokenStream};

use quote::{quote, format_ident};

/// Proc macro that generates an easy to use api function to use directly in rust out of
/// a hdk_extern function.
/// "zome_*" is prepended to the function name
#[proc_macro_attribute]
pub fn zome_api(_metadata: TokenStream, item: TokenStream) -> TokenStream {
   // -- Parse input and retrieve function signature
   let item_fn = syn::parse_macro_input!(item as syn::ItemFn);
   let external_fn_ident = item_fn.sig.ident.clone();
   let input_type = if let Some(syn::FnArg::Typed(pat_type)) = item_fn.sig.inputs.first() {
      pat_type.ty.clone()
   } else {
      unreachable!();
   };
   let output_type = if let syn::ReturnType::Type(_, ref ty) = item_fn.sig.output {
      ty.clone()
   } else {
      unreachable!();
   };

   // Get the type within the ExternResult
   let path_type = if let syn::Type::Path(tp) = *output_type.clone() {
      tp.clone()
   } else {
      unreachable!();
   };
   let angle_type = if let syn::PathArguments::AngleBracketed(ab) = &path_type.path.segments[0].arguments{
      ab.clone()
   } else {
      unreachable!();
   };
   let type_type = if let syn::GenericArgument::Type(tt) = &angle_type.args[0] {
      tt.clone()
   } else {
      unreachable!();
   };
   let inner_path_type = if let syn::Type::Path(tp) = type_type {
      tp.clone()
   } else {
      unreachable!();
   };
   let inner_type = &inner_path_type.path.segments[0];
   //println!("\n\n input.inner_type: \"{:?}\"\n\n", inner_type);

   // -- Output api function
   // TODO: Should use zome-name as prefix to generated function name
   let output_fn = format_ident!("zome_{}", external_fn_ident);
   println!("Generated: {}()", output_fn);

   // Output
   let output: TokenStream = (quote! {
      #item_fn
      #[cfg(not(target_arch = "wasm32"))]
      pub fn #output_fn(conductor: holochain::conductor::ConductorHandle, arg: #input_type) -> crate::api_error::SnapmailApiResult<#inner_type> {
         let DEFAULT_TIMEOUT = std::time::Duration::from_secs(9);
         let payload = ExternIO::encode(arg).expect("Serialization should never fail");
         //println!(" payload = {:?}", payload);
         let fn_name = std::stringify!(#external_fn_ident);
         //println!(" fn_name = {:?}", fn_name);
         let result = holochain_util::tokio_helper::block_on(async {
            // -- call_zome
            let cell_ids = conductor.list_cell_ids(None);
            //println!("Cell IDs : {:?}", cell_ids);
            assert!(!cell_ids.is_empty());
            let cell_id = cell_ids[0].clone();
            let provenance = cell_ids[0].agent_pubkey().to_owned();
            let call_result = conductor.call_zome(holochain_conductor_api::ZomeCall {
               cell_id,
               zome_name: zome_info()?.name.into(),
               fn_name: fn_name.into(),
               payload,
               cap_secret: None,
               provenance,
            })
            .await
            .map_err(|e| holochain::conductor::api::error::ConductorApiError(e))?
            .map_err(|e| holochain::core::ribosome::RibosomeError(e))?;

            // println!("  ZomeCall result = {:?}", call_result);
            // - Handle result
            let api_result = match call_result {
               ZomeCallResponse::Ok(io) => {
                  let maybe_ret: #inner_type = io.decode().expect("Deserialization should never fail");
                  Ok(maybe_ret)
               },
               ZomeCallResponse::Unauthorized(_, _, _, _) => Err(holochain_zome_types::ZomeCallResponse::Unauthorized),
               ZomeCallResponse::NetworkError(err) => Err(holochain_zome_types::ZomeCallResponse::NetworkError(err)),
               ZomeCallResponse::CountersigningSession(err) => Err(holochain_zome_types::ZomeCallResponse::Unauthorized),
            };
            api_result
         }, DEFAULT_TIMEOUT).map_err(|_e| KitsuneTimeout)?;
         //println!(" block_on result = {:?}", result);
         result
      }
   }).into();
   //println!("\n\n output: \"{}\"\n\n", output.to_string());
   output
}
