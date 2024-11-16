use crate::AttributeBuilder;
use crate::AttributeMode;
use crate::Error;
use crate::MessageClient;
use crate::MessageCodec;
use crate::MessageDispatcher;
use crate::MessageHandler;
use async_trait::async_trait;
use bytes::Bytes;
use rumqttc::QoS;
use std::sync::Arc;
use std::sync::Weak;
use tokio::sync::Mutex;
use tokio::sync::Notify;

///
///
///
#[derive(Clone)]
pub struct AttServer<TYPE: MessageCodec> {
    /// Reactor message dispatcher
    /// (to attach this attribute to the incoming messages)
    message_dispatcher: Weak<Mutex<MessageDispatcher>>,

    ///
    /// The message client (MQTT)
    ///
    pub message_client: MessageClient,

    ///
    /// The topic of the attribute
    ///
    pub topic: String,

    ///
    /// New received messages are stored in this queue
    /// User can 'pop' them in its event callback to that every messages
    ///
    pub in_queue: Vec<TYPE>,

    ///
    /// Last popped value by the user
    ///
    pub last_popped_value: Option<TYPE>,

    ///
    /// Input notifier, alert when a new message has arrived in hte queue
    ///
    pub in_notifier: Arc<Notify>,

    ///
    /// The topic for 'att' topic to send data to user
    ///
    topic_att: String,

    ///
    /// Requested value of the attribute (set by the user)
    ///
    requested_value: Option<TYPE>,

    ///
    ///
    ///
    mode: AttributeMode,
}

impl<TYPE: MessageCodec> AttServer<TYPE> {
    ///
    /// Initialize the attribute
    /// Register the attribute on the message dispatcher then subscribe to att topic
    ///
    pub async fn init(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        self.register(attribute).await?;
        self.subscribe().await
    }

    ///
    /// Subscribe to the topic
    ///
    pub async fn subscribe(&self) -> Result<(), Error> {
        // no need to store the att topic
        let topic_att = format!("{}/cmd", self.topic);
        self.message_client
            .subscribe(topic_att, QoS::AtMostOnce)
            .await
            .map_err(|e| Error::MessageAttributeSubscribeError(e.to_string()))
    }

    ///
    /// Register the attribute to the reactor
    ///
    pub async fn register(&self, attribute: Arc<Mutex<dyn MessageHandler>>) -> Result<(), Error> {
        // no need to store the att topic
        let topic_att = format!("{}/cmd", self.topic);
        self.message_dispatcher
            .upgrade()
            .ok_or(Error::InternalPointerUpgrade)?
            .lock()
            .await
            .register_message_attribute(topic_att, attribute);
        Ok(())
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn pop_cmd(&mut self) -> Option<TYPE> {
        let next = self.in_queue.pop();
        if next.is_some() {
            self.last_popped_value = next.clone();
        }
        return next;
    }

    ///
    /// Get the value of the attribute
    /// If None, the first value is not yet received
    ///
    pub fn get_last_cmd(&self) -> Option<TYPE> {
        return self.last_popped_value.clone();
    }

    ///
    /// Clone the change notifier
    ///
    pub fn in_notifier(&self) -> Arc<Notify> {
        self.in_notifier.clone()
    }

    /// Set the value of the attribute
    ///
    pub async fn set(&mut self, new_value: TYPE) -> Result<(), Error> {
        // // Do not go further if the value is already set
        // if let Some(current_value) = self.value {
        //     if current_value == new_value {
        //         return Ok(());
        //     }
        // }

        // Set the requested value and publish the request
        self.requested_value = Some(new_value);
        match self.requested_value.clone() {
            Some(requested_value) => {
                self.publish(requested_value.into_message_payload()?)
                    .await
                    .unwrap();
            }
            None => {
                return Err(Error::Wtf);
            }
        }

        Ok(())
    }

    /// Publish a command
    ///
    pub async fn publish<V>(&self, value: V) -> Result<(), Error>
    where
        V: Into<Vec<u8>>,
    {
        self.message_client
            .publish(&self.topic_att, QoS::AtMostOnce, true, value)
            .await
            .map_err(|e| Error::MessageAttributePublishError(e.to_string()))
    }
}

#[async_trait]
impl<TYPE: MessageCodec> MessageHandler for AttServer<TYPE> {
    ///
    /// On message, just deserialize then push into the fifo
    ///
    async fn on_message(&mut self, data: &Bytes) -> Result<(), Error> {
        let in_value = TYPE::from_message_payload(data)?;
        self.in_queue.push(in_value);
        self.in_notifier.notify_waiters();
        Ok(())
    }
}

///
/// Allow creation from the builder
///
impl<TYPE: MessageCodec> From<AttributeBuilder> for AttServer<TYPE> {
    fn from(builder: AttributeBuilder) -> Self {
        let topic = builder.topic.as_ref().unwrap().clone();
        Self {
            message_dispatcher: builder.message_dispatcher,
            message_client: builder.message_client,
            topic: topic.clone(),
            in_queue: Vec::new(),
            last_popped_value: None,
            in_notifier: Arc::new(Notify::new()),
            topic_att: format!("{}/att", topic.clone()),
            requested_value: None,
            mode: builder.mode.clone().unwrap(),
        }
    }
}
