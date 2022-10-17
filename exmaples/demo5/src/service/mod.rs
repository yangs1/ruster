use std::sync::Arc;

use http::StatusCode;

use crate::{
    error::KvError,
    pb::abi::{command_request::RequestData, CommandRequest, CommandResponse, Hget, Kvpair},
    storage::Storage,
};

pub mod command;

pub trait CommandService {
    fn execute(self, storage: &impl Storage) -> CommandResponse;
}

pub struct Service<Store> {
    inner: Arc<ServiceInner<Store>>,
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store: store }),
        }
    }

    pub fn execute(&self, commandReqeust: CommandRequest) -> CommandResponse {
        let res = dispatch(commandReqeust, &self.inner.store);

        res
    }
}

// 从 Request 中得到 Response，目前处理 HGET/HGETALL/HSET
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(param)) => param.execute(store),
        // Some(RequestData::Hgetall(param)) => param.execute(store),
        // Some(RequestData::Hset(param)) => param.execute(store),
        // None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented".into()).into(),
    }
}
