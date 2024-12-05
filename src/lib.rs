#![deny(
    while_true,
    improper_ctypes,
    non_shorthand_field_patterns,
//     no_mangle_generic_items,
    overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     unconditional_recursion,
//     bad_style,
//     dead_code,
//     unused,
//     unused_allocation,
//     unused_comparisons,
//     unused_parens,
)]

pub mod pmacro;

// Main error crate for Panduza Platform
mod error;
pub use error::Error;

///
mod factory;
pub use factory::production_order::DeviceSettings;
pub use factory::production_order::ProductionOrder;
pub use factory::store::Product;
pub use factory::store::Store;
pub use factory::Factory;
pub use factory::ScanMachine;

//
pub mod instance;
pub use instance::monitor::InstanceMonitor;
pub use instance::Instance;
pub use instance::InstanceInner;
//
mod interface;
pub use interface::builder::InterfaceBuilder;
pub use interface::Class;

//
mod attribute;
pub use attribute::builder::AttributeBuilder;
pub use attribute::server_boolean::BooleanAttServer;
pub use attribute::server_enum::EnumAttServer;
pub use attribute::server_json::JsonAttServer;
pub use attribute::server_mem_cmd::MemoryCommandAttServer;
pub use attribute::server_number::NumberAttServer;
pub use attribute::server_si::SiAttServer;
pub use attribute::server_string::StringAttServer;

// public traits
mod traits;
pub use traits::DriverOperations;
pub use traits::MessageCodec;
pub use traits::MessageHandler;
pub use traits::Producer;
pub use traits::Scanner;

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

/// Module that manage platform traces and logs
///
pub mod tracing;
pub use tracing::AttributeLogger;
pub use tracing::DriverLogger;
pub use tracing::FactoryLogger;
pub use tracing::InstanceLogger;
pub use tracing::PlatformLogger;
pub use tracing::RuntimeLogger;

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

///
///
///
pub mod props;
pub use props::Prop;
pub use props::PropType;
pub use props::Props;
