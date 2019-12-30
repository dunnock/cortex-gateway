use crate::Handler;
use async_trait::async_trait;
use hyper::{Body, Error, Method, Request, Response};
use evmap::{WriteHandle, ReadHandle};

type ReadMap = ReadHandle<u32, Handler>;
type WriteMap = WriteHandle<u32, Handler>;

#[derive(Clone)]
pub struct EvmapRouter {
	// evmap read handles
	reader: ReadMap
}

/// Routes writer - only single instance of writer per map of routes exists
/// - should it be one instance per one client? 
/// 	-- in this case domain matching should be performed before?
/// 	-- if single instance per server then domain param should be added to handler structure
/// 	-- if single instance per server there will be some redundancy when copying evmap_reader
/// - add/remove routes
pub struct EvmapRouterWriter {
	// evmap write handle
	writer: WriteMap 
}

impl EvmapRouterWriter {
	pub fn new(writer: WriteHandle<u32, Handler>) -> Self {
		Self {
			writer
		}
	}
    pub fn add(&mut self, method: Method, route_pattern: &str, handler: Handler) {

	}
    pub fn remove(&mut self, route_pattern: &str) {

	}
}

/// Thread safe router
/// - handle routes
/// - safe to clone between threads (it should only make shallow copy of evmap reader)
#[async_trait]
pub trait Router: Send+Sync+Clone {
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Error>;
}

impl EvmapRouter {
    pub fn new(reader: ReadHandle<u32, Handler>) -> Self {
        Self {
			reader
		}
    }
}

#[async_trait]
impl Router for EvmapRouter {
    async fn handle(&self, req: Request<Body>) -> Result<Response<Body>, Error> {
        let response = async {
			// TODO: 
			// let topic = self.evmap_matcher.match(req.path()).get_topic();
			// nats.post(topic, req.body()).await
			// nats.get().await
            "Hello, World".into()
        }
        .await;
        Ok(Response::new(response))
    }
}

unsafe impl Send for EvmapRouter {}
unsafe impl Sync for EvmapRouter {}