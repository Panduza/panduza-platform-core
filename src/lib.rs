pub mod pmacro;

// Main error crate for Panduza Platform
mod error;
pub use error::Error;

/// Loggers
mod logger;
pub use logger::DeviceLogger;
pub use logger::DriverLogger;
pub use logger::FactoryLogger;
pub use logger::GenericLogger;
pub use logger::PlatformLogger;
pub use logger::RuntimeLogger;

///
mod factory;
pub use factory::production_order::DeviceSettings;
pub use factory::production_order::ProductionOrder;
pub use factory::Factory;

//
pub mod device;
pub use device::monitor::DeviceMonitor;
pub use device::Device;
pub use device::DeviceInner;
//
mod interface;
pub use interface::builder::InterfaceBuilder;
pub use interface::Interface;

//
mod attribute;
pub use attribute::att_only_msg_att::AttOnlyMsgAtt;
pub use attribute::bidir_msg_att::BidirMsgAtt;
pub use attribute::builder::AttributeBuilder;
pub use attribute::cmd_only_msg_att::CmdOnlyMsgAtt;

pub use attribute::server_boolean::BooleanAttServer;
pub use attribute::server_enum::EnumAttServer;
pub use attribute::server_si::SiAttServer;
pub use attribute::server_string::StringAttServer;

// public traits
mod traits;
pub use traits::DeviceOperations;
pub use traits::MessageCodec;
pub use traits::MessageHandler;
pub use traits::Producer;

//
mod reactor;
pub use reactor::message_dispatcher::MessageDispatcher;
pub use reactor::settings::ReactorSettings;
pub use reactor::Reactor;

// This module manage the message attributes (MQTT/TCP)
// pub mod msg;
pub type MessageClient = rumqttc::AsyncClient;

//
mod codec;
pub use codec::boolean::BooleanCodec;
pub use codec::eenum::EnumCodec;
pub use codec::json::JsonCodec;
pub use codec::memory_command::MemoryCommandCodec;
pub use codec::memory_command::MemoryCommandMode;
pub use codec::number::NumberCodec;
pub use codec::number_list::NumberListCodec;
pub use codec::raw::RawCodec;
pub use codec::si::SiCodec;
pub use codec::string::StringCodec;
pub use codec::string_list::StringListCodec;

mod task_channel;
pub use task_channel::create_task_channel;
pub use task_channel::TaskReceiver;
pub use task_channel::TaskSender;

/// Return type for spawned task
pub type TaskResult = Result<(), Error>;

//
pub mod plugin;
pub use plugin::Plugin;

pub mod runtime;
pub use runtime::Runtime;

pub mod log;

pub mod env;

mod notification;
pub use notification::group::NotificationGroup;
pub use notification::structural::attribute::AttributeMode;
pub use notification::structural::AttributeNotification;
pub use notification::structural::InterfaceNotification;
pub use notification::AlertNotification;
pub use notification::Notification;
pub use notification::StateNotification;
pub use notification::StructuralNotification;

pub mod settings;
pub use settings::eenum::EnumSettings;
pub use settings::si::SiSettings;

/// Built-in drivers to help coding plugins
///
/// # Enabling
///
/// Specific features need to be activated to enable drivers
///
/// - usb => for usb drivers (also enable usb)
/// - serial => for serial drivers (also enable usb)
///
pub mod drivers;

/// Currently we put here a trait waiting to see if there is a better use later
///
pub mod protocol;
