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

/// TODO: documentation
impl Server {
    /// TODO: documentation
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// TODO: documentation
impl Server {
    /// TODO: documentation
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_inner(self) -> InterceptedServer<UserServer<Controller>> {
        self.inner
    }
}

/// TODO: documentation
impl Default for Server {
    /// TODO: documentation
    fn default() -> Self {
        let config = Config::new();
        let service = Service::new(config.clone());
        let controller = Controller::new(config, service);

        Self {
            inner: InterceptedServer::new(UserServer::new(controller)),
        }
    }
}
