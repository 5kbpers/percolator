use std::time::Duration;

use crate::msg::*;
use crate::service::*;
use crate::*;

use labrpc::{RpcFuture};

// TTL is used for a lock key. 
// If the key's lifetime exceeds this value, it should be cleaned up.
// Otherwise, the operation should back off.
const TTL: u64 = Duration::from_millis(100).as_nanos() as u64;

impl KvTable {
    // Reads the latest key-value record from a specified column
    // in MemoryStorage with a given key and a timestamp range.
    #[inline]
    fn read(
        &self,
        key: Vec<u8>,
        column: Column,
        ts_start_inclusive: Option<u64>,
        ts_end_inclusive: Option<u64>,
    ) -> Option<(&Key, &Value)> {
        // Your code here.
        unimplemented!()
    }

    // Writes a record to a specified column in MemoryStorage.
    #[inline]
    fn write(&mut self, key: Vec<u8>, column: Column, ts: u64, value: Value) {
        // Your code here.
        unimplemented!()
    }

    #[inline]
    // Erases a record from a specified column in MemoryStorage.
    fn erase(&mut self, key: Vec<u8>, column: Column, commit_ts: u64) {
        // Your code here.
        unimplemented!()
    }
}

impl transaction::Service for MemoryStorage {
    // example get RPC handler.
    fn get(&self, req: GetRequest) -> RpcFuture<GetResponse> {
        // Your code here.
        unimplemented!()
    }

    // example prewrite RPC handler.
    fn prewrite(&self, req: PrewriteRequest) -> RpcFuture<PrewriteResponse> {
        // Your code here.
        unimplemented!()
    }

    // example commit RPC handler.
    fn commit(&self, req: CommitRequest) -> RpcFuture<CommitResponse> {
        // Your code here.
        unimplemented!()
    }
}

impl MemoryStorage {

    fn back_off_maybe_clean_up_lock(&self, start_ts: u64, key: Vec<u8>) {
        // Your code here.
        unimplemented!()
    }
}

impl timestamp::Service for TimestampOracle {
    // example get_timestamp RPC handler.
    fn get_timestamp(&self, _: TimestampRequest) -> RpcFuture<TimestampResponse> {
        // Your code here.
        unimplemented!()
    }
}
