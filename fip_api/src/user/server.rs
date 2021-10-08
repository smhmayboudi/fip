use crate::{
    common::intercepted_server::InterceptedServer,
    user::{
        config::Config, controller::Controller, proto::server::user_server::UserServer,
        service::Service,
    },
};

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: InterceptedServer<UserServer<Controller>>,
}

impl Server {
    /// TODO: documentation
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    /// TODO: documentation
    pub fn into_inner(self) -> InterceptedServer<UserServer<Controller>> {
        self.inner
    }
}

impl Default for Server {
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(UserServer::new(controller)),
        }
    }
}
