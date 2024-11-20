use super::server_si::SiAttServer;
use super::{att_only_msg_att::AttOnlyMsgAtt, cmd_only_msg_att::CmdOnlyMsgAtt};
use crate::{notification::structural::attribute::AttributeMode, Notification};
use crate::{
    BidirMsgAtt, BooleanAttServer, EnumAttServer, Error, JsonAttServer, MessageClient,
    MessageCodec, MessageDispatcher, StringAttServer,
};
use serde_json::json;
use std::sync::Weak;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;

#[derive(Clone)]
///
/// Object that allow to build an generic attribute
///
pub struct AttributeBuilder {
    /// The mqtt client
    pub message_client: MessageClient,

    /// The Object that allow the reactor to dispatch
    /// incoming messages on attributes
    pub message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    ///
    pub r_notifier: Option<Sender<Notification>>,

    /// Topic of the attribute
    pub topic: Option<String>,

    ///
    /// Attribute Settings
    ///
    pub settings: Option<serde_json::Value>,

    pub mode: Option<AttributeMode>,

    pub r#type: Option<String>,

    pub info: Option<String>,
}

impl AttributeBuilder {
    /// Create a new builder
    pub fn new(
        message_client: MessageClient,
        message_dispatcher: Weak<Mutex<MessageDispatcher>>,
        r_notifier: Option<Sender<Notification>>,
    ) -> AttributeBuilder {
        AttributeBuilder {
            message_client,
            message_dispatcher,
            r_notifier,
            topic: None,
            settings: None,
            mode: None,
            r#type: None,
            info: None,
        }
    }
    /// Attach a topic
    pub fn with_topic<T: Into<String>>(mut self, topic: T) -> Self {
        self.topic = Some(topic.into());
        self
    }

    ///
    /// Attach settings to the attribute
    ///
    pub fn with_settings(mut self, settings: serde_json::Value) -> Self {
        self.settings = Some(settings);
        self
    }

    pub fn with_ro(mut self) -> Self {
        self.mode = Some(AttributeMode::AttOnly);
        self
    }
    pub fn with_wo(mut self) -> Self {
        self.mode = Some(AttributeMode::CmdOnly);
        self
    }
    pub fn with_rw(mut self) -> Self {
        self.mode = Some(AttributeMode::Bidir);
        self
    }

    pub fn with_info<T: Into<String>>(mut self, info: T) -> Self {
        self.info = Some(info.into());
        self
    }

    ///
    ///
    ///
    pub async fn finish_as_si<N: Into<String>>(
        mut self,
        unit: N,
        min: i32,
        max: i32,
        decimals: u32,
    ) -> Result<SiAttServer, Error> {
        self.r#type = Some(SiAttServer::r#type());
        let unit_string = unit.into();
        self.settings = Some(json!(
            {
                "unit": unit_string.clone(),
                "min": min,
                "max": max,
                "decimals": decimals,
            }
        ));
        let att = SiAttServer::new(self.clone(), unit_string, min, max, decimals);
        att.inner.lock().await.init(att.inner.clone()).await?;
        self.send_creation_notification();
        Ok(att)
    }

    ///
    /// Finish attribute building and configure it with 'boolean' type.
    ///
    pub async fn finish_as_boolean(mut self) -> Result<BooleanAttServer, Error> {
        self.r#type = Some(BooleanAttServer::r#type());
        let att = BooleanAttServer::new(self.clone());
        att.inner.lock().await.init(att.inner.clone()).await?;
        self.send_creation_notification();
        Ok(att)
    }

    ///
    /// Finish attribute building and configure it with 'string' type.
    ///
    pub async fn finish_as_string(mut self) -> Result<StringAttServer, Error> {
        self.r#type = Some(StringAttServer::r#type());
        let att = StringAttServer::new(self.clone());
        att.inner.lock().await.init(att.inner.clone()).await?;
        self.send_creation_notification();
        Ok(att)
    }

    ///
    /// Finish attribute building and configure it with 'enum' type.
    ///
    pub async fn finish_as_enum(mut self, choices: Vec<String>) -> Result<EnumAttServer, Error> {
        self.r#type = Some(EnumAttServer::r#type());
        let att = EnumAttServer::new(self.clone(), choices);
        att.inner.lock().await.init(att.inner.clone()).await?;
        self.send_creation_notification();
        Ok(att)
    }

    ///
    /// Finish attribute building and configure it with 'json' type.
    ///
    pub async fn finish_as_json(mut self) -> Result<JsonAttServer, Error> {
        self.r#type = Some(JsonAttServer::r#type());
        let att = JsonAttServer::new(self.clone());
        att.inner.lock().await.init(att.inner.clone()).await?;
        self.send_creation_notification();
        Ok(att)
    }

    ///
    ///
    ///
    fn send_creation_notification(&self) {
        //
        //
        let bis = self.topic.clone().unwrap();
        if let Some(r_notifier) = self.r_notifier.clone() {
            r_notifier
                .try_send(Notification::new_attribute_element_created_notification(
                    bis,
                    self.r#type.clone().unwrap(),
                    self.mode.clone().unwrap(),
                    self.info.clone(),
                    self.settings.clone(),
                ))
                .unwrap();
        }
    }

    #[deprecated]
    pub fn message(self) -> MessageAttributeBuilder {
        MessageAttributeBuilder { base: self }
    }

    #[deprecated]
    pub fn stream(self) {
        todo!()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct MessageAttributeBuilder {
    base: AttributeBuilder,
}

impl MessageAttributeBuilder {
    pub fn with_cmd_only_access(self) -> CmdOnlyMsgAttBuilder {
        CmdOnlyMsgAttBuilder { base: self.base }
    }

    pub fn with_bidir_access(self) -> BidirMsgAttBuilder {
        BidirMsgAttBuilder { base: self.base }
    }

    pub fn with_att_only_access(self) -> AttOnlyMsgBuilder {
        AttOnlyMsgBuilder { base: self.base }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Ro Attribute
pub struct CmdOnlyMsgAttBuilder {
    base: AttributeBuilder,
}

impl CmdOnlyMsgAttBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> CmdOnlyMsgAtt<TYPE> {
        //
        //

        let bis = self.base.topic.clone().unwrap();

        if let Some(r_notifier) = self.base.r_notifier.clone() {
            r_notifier
                .try_send(Notification::new_attribute_element_created_notification(
                    bis,
                    TYPE::typee(),
                    AttributeMode::CmdOnly,
                    None,
                    self.base.settings.clone(),
                ))
                .unwrap();
        }

        CmdOnlyMsgAtt::from(self.base).init().await.unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for Rw Attribute
pub struct BidirMsgAttBuilder {
    base: AttributeBuilder,
}

impl BidirMsgAttBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> BidirMsgAtt<TYPE> {
        //
        //
        let bis = self.base.topic.clone().unwrap();
        if let Some(r_notifier) = self.base.r_notifier.clone() {
            r_notifier
                .try_send(Notification::new_attribute_element_created_notification(
                    bis,
                    TYPE::typee(),
                    AttributeMode::Bidir,
                    None,
                    self.base.settings.clone(),
                ))
                .unwrap();
        }

        BidirMsgAtt::from(self.base)
            .init()
            .await
            .unwrap()
            .init()
            .await
            .unwrap()
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Builder specialisation for att only Attribute
pub struct AttOnlyMsgBuilder {
    base: AttributeBuilder,
}

impl AttOnlyMsgBuilder {
    pub async fn finish_with_codec<TYPE: MessageCodec>(self) -> AttOnlyMsgAtt<TYPE> {
        //
        //
        let bis = self.base.topic.clone().unwrap();
        if let Some(r_notifier) = self.base.r_notifier.clone() {
            r_notifier
                .try_send(Notification::new_attribute_element_created_notification(
                    bis,
                    TYPE::typee(),
                    AttributeMode::AttOnly,
                    None,
                    self.base.settings.clone(),
                ))
                .unwrap();
        }

        AttOnlyMsgAtt::from(self.base)
    }
}
