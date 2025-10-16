mod builder;
pub use builder::PluginBuilder;
mod error;
mod store;
pub use error::Error;
mod version;
pub use version::VERSION;
mod registry;
pub use registry::PluginRegistry;

use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct Plugin {
	/// 插件名称
	pub name: &'static str,
	/// 插件版本
	pub version: &'static str,
	/// 插件作者
	pub author: &'static str,
}

#[derive(Clone)]
pub enum PluginType {
	/// 基于文件路径加载的动态库插件
	Path(PathBuf),
	/// 静态链接的插件
	Builder(&'static dyn PluginBuilder),
}

impl fmt::Debug for PluginType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PluginType::Path(path) => f.debug_struct("Path").field("path", path).finish(),
			PluginType::Builder(_) => f.debug_struct("Builder").finish(),
		}
	}
}
impl From<PathBuf> for PluginType {
	fn from(path: PathBuf) -> Self {
		PluginType::Path(path)
	}
}

impl From<&'static dyn PluginBuilder> for PluginType {
	fn from(builder: &'static dyn PluginBuilder) -> Self {
		PluginType::Builder(builder)
	}
}

#[derive(Debug, Clone)]
pub enum PluginId {
	Index(u64),
	Name(String),
}
impl From<u64> for PluginId {
	fn from(value: u64) -> Self {
		Self::Index(value)
	}
}
impl From<String> for PluginId {
	fn from(value: String) -> Self {
		Self::Name(value)
	}
}
impl From<&str> for PluginId {
	fn from(value: &str) -> Self {
		Self::Name(value.to_string())
	}
}
