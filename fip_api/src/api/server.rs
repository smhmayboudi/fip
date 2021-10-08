use crate::{
    api::{
        config::Config, controller::Controller, proto::server::api_server::ApiServer,
        service::Service,
    },
    common::intercepted_server::InterceptedServer,
};

#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<ApiServer<Controller>>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    pub fn into_inner(self) -> InterceptedServer<ApiServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(ApiServer::new(controller)),
        }
    }
}
