//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	let version = sc_cli::VersionInfo {
		name: "TswapChain Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "tswapchain",
		author: "Anonymous",
		description: "Tswapchain Node",
		support_url: "support.anonymous.an",
		copyright_start_year: 2020,
	};

	command::run(version)
}