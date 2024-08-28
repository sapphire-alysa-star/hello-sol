// // https://solana.com/developers/guides/advanced/how-to-optimize-compute



// // 11962 CU !!
// // Base58 encoding is expensive, concatenation is expensive
// compute_fn! { "Log a pubkey to account info" =>
//     msg!("A string {0}", ctx.accounts.counter.to_account_info().key());
// }
 
// // 357 CU - string concatenation is expensive
// compute_fn! { "Log a pubkey simple concat" =>
//     msg!("A string {0}", "5w6z5PWvtkCd4PaAV7avxE6Fy5brhZsFdbRLMt8UefRQ");
// }

// // 262 cu
// compute_fn! { "Log a pubkey" =>
//     ctx.accounts.counter.to_account_info().key().log();
// }

// // 357
// compute_fn! { "Push Vector u64 " =>
//     let mut a: Vec<u64> = Vec::new();
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
// }
 
// // 211 CU
// compute_fn! { "Vector u8 " =>
//     let mut a: Vec<u8> = Vec::new();
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
//     a.push(1);
// }


// // 6302 CU
// pub fn initialize(_ctx: Context<InitializeCounter>) -> Result<()> {
//     Ok(())
// }
 
// // 5020 CU
// pub fn initialize_zero_copy(_ctx: Context<InitializeCounterZeroCopy>) -> Result<()> {
//     Ok(())
// }

// // 108 CU - total CU including serialization 2600
// let counter = &mut ctx.accounts.counter;
// compute_fn! { "Borsh Serialize" =>
//     counter.count = counter.count.checked_add(1).unwrap();
// }
 
// // 151 CU - total CU including serialization 1254
// let counter = &mut ctx.accounts.counter_zero_copy.load_mut()?;
// compute_fn! { "Zero Copy Serialize" =>
//     counter.count = counter.count.checked_add(1).unwrap();
// }

// pub fn pdas(ctx: Context<PdaAccounts>) -> Result<()> {
//     let program_id = Pubkey::from_str("5w6z5PWvtkCd4PaAV7avxE6Fy5brhZsFdbRLMt8UefRQ").unwrap();
 
//     // 12,136 CUs
//     compute_fn! { "Find PDA" =>
//         Pubkey::find_program_address(&[b"counter"], ctx.program_id);
//     }
 
//     // 1,651 CUs
//     compute_fn! { "Find PDA" =>
//         Pubkey::create_program_address(&[b"counter", &[248_u8]], &program_id).unwrap();
//     }
 
//     Ok(())
// }

// There are many other ways to optimize your program's compute usage, such as writing in native instead of anchor, but it all comes at a cost.