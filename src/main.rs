use anyhow::Result;
use tor_chanmgr::{ChanMgr, ChannelConfig, ChannelUsage, Dormancy};
// use tor_chanmgr::factory::ChannelFactory;
use tor_linkspec::OwnedChanTarget;
use tor_netdir::params::NetParameters;
use tor_memquota::mtracker::MemoryQuotaTracker;
use tor_memquota::Config as MemquotaConfig;
use tor_rtcompat::PreferredRuntime;

#[tokio::main]
async fn main() -> Result<()> {
    // RUST_LOG=debug
    tracing_subscriber::fmt::init();

    let runtime = PreferredRuntime::current().unwrap();
    let config = ChannelConfig::default();
    let dormancy = Dormancy::default();
    let netparams = NetParameters::default();

    let memquota_config = MemquotaConfig::builder().build().unwrap();
    let memquota = MemoryQuotaTracker::new(&runtime, memquota_config).unwrap();

    let keymgr = None;  // TODO

    let chanmgr = ChanMgr::new(runtime, &config, dormancy, &netparams, memquota, keymgr);

    // let target = OwnedChanTarget::from_chan_target(target);
    // let usage = ChannelUsage::Dir;

    // let t = chanmgr.get_or_launch(&target, usage).await?;



    // let cb = ChannelBuilder::new();

    Ok(())
}
