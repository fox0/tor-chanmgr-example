use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use anyhow::Result;
use tor_chanmgr::builder::ChanBuilder;
use tor_chanmgr::factory::{BootstrapReporter, ChannelFactory};
use tor_chanmgr::transport::proxied::ExternalProxyPlugin;
use tor_chanmgr::{
    ChanMgr, ChanMgrConfig, ChannelConfig, ChannelConfigBuilder, ChannelUsage, Dormancy,
};
use tor_linkspec::OwnedChanTarget;
use tor_memquota::Config as MemquotaConfig;
use tor_memquota::mtracker::MemoryQuotaTracker;
use tor_netdir::params::NetParameters;
use tor_proto::channel::ChannelBuilder;
use tor_proto::memquota::ChannelAccount;
use tor_rtcompat::PreferredRuntime;
use tor_socksproto::SocksVersion;

#[tokio::main]
async fn main() -> Result<()> {
    // RUST_LOG=debug
    tracing_subscriber::fmt::init();

    let runtime = PreferredRuntime::current().unwrap();

    // let ccb = ChannelConfigBuilder::new();

    let config = ChannelConfig::default();
    config.outbound_proxy = None;  // TODO

    // let dormancy = Dormancy::default();
    // let netparams = NetParameters::default();

    // let memquota_config = MemquotaConfig::builder().build().unwrap();
    // let memquota = MemoryQuotaTracker::new(&runtime, memquota_config).unwrap();

    // let chan_manager = ChanMgr::new(
    //     runtime.clone(),
    //     ChanMgrConfig::default(),
    //     dormancy,
    //     &netparams,
    //     memquota,
    // );

    // let target = OwnedChanTarget::from_chan_target(target);
    // let usage = ChannelUsage::Dir;

    // let (channel, _) = chan_manager.get_or_launch(&target, usage).await?;

    // let cb = ChannelBuilder::new();

    //////////////////////////////////////////////////////////////////////////////////////

    let proxy_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let proxy_version = SocksVersion::V4; // TODO
    let transport = ExternalProxyPlugin::new(runtime.clone(), proxy_addr, proxy_version);

    // let builder = ChanBuilder::new(runtime, transport);
    // builder.connect_via_transport(target, reporter, memquota);

    //////////////////////////////////////////////////////////////////////////////////////

    // let memquota_config = MemquotaConfig::builder().build().unwrap();
    // let memquota: ChannelAccount = MemoryQuotaTracker::new(&runtime, memquota_config)
    //     .unwrap()
    //     .into();
    // // let memquota = ChannelAccount::default();

    // let channel = builder
    //     .connect_via_transport(target, reporter, memquota)
    //     .await
    //     .unwrap();

    Ok(())
}
