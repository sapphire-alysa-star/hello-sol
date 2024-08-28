// https://docs.rs/solana-program/latest/solana_program/program/fn.set_return_data.html

// Set the running program’s return data.

// Return data is a dedicated per-transaction buffer for data passed from cross-program invoked programs back to their caller.

// The maximum size of return data is MAX_RETURN_DATA. Return data is retrieved by the caller with get_return_data.


// https://docs.rs/solana-program/latest/solana_program/program/fn.get_return_data.html


// For every transaction there is a single buffer with maximum length MAX_RETURN_DATA, paired with a Pubkey representing the program ID of the program that most recently set the return data. Thus the return data is a global resource and care must be taken to ensure that it represents what is expected: called programs are free to set or not set the return data; and the return data may represent values set by programs multiple calls down the call stack, depending on the circumstances of transaction execution.

// Function solana_program::program::set_return_dataCopy item path
// source · [−]
// pub fn set_return_data(data: &[u8])

// Return data is set by the callee with set_return_data.

// Return data is cleared before every CPI invocation — a program that has invoked no other programs can expect the return data to be None; if no return data was set by the previous CPI invocation, then this function returns None.


// Return data is not cleared after returning from CPI invocations —  a program that has called another program may retrieve return data that was not set by the called program, but instead set by a program further down the call stack; or,  if a program calls itself recursively, it is possible that the return data was not set by the immediate call to that program, but by a subsequent recursive call to that program. Likewise, an external RPC caller may see return data that was not set by the program it is directly calling, but by a program that program called.

// Function solana_program::program::get_return_dataCopy item path
// source · [−]
// pub fn get_return_data() -> Option<(Pubkey, Vec<u8>)>


// https://solana.com/docs/rpc/http#gettransaction

// return data is in the transactions meta

// rpc methods --