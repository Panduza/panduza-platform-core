use crate::{BooleanAttServer, Class, Error};

use super::attribute::server::EnablementDisablement;

#[derive(Clone)]
pub enum Element {
    Class(Class),
    AsBoolean(BooleanAttServer),
}

impl Element {
    /// Request attribute server enablement
    ///
    pub async fn change_enablement(&mut self, enabled: bool) -> Result<(), Error> {
        match self {
            Element::Class(class) => Ok(()),
            Element::AsBoolean(boolean_att_server) => {
                boolean_att_server.change_enablement(enabled).await
            }
        }
    }

    /// Request attribute server enablement
    ///
    pub async fn enable(&mut self) -> Result<(), Error> {
        self.change_enablement(true).await
    }

    /// Request attribute server disablement
    ///
    pub async fn disable(&mut self) -> Result<(), Error> {
        self.change_enablement(false).await
    }
}
