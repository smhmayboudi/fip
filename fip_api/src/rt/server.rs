use crate::{
    common::intercepted_server::InterceptedServer,
    rt::{
        config::Config, controller::Controller, proto::server::rt_server::RtServer,
        service::Service,
    },
};

#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<RtServer<Controller>>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    pub fn into_inner(self) -> InterceptedServer<RtServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::default();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(RtServer::new(controller)),
        }
    }
}
