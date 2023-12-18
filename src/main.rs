use harvest::auth::OAuth;
use harvest::Harvest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // temp path
    let path = "/home/stefan/.config/harvest.json";
    let oauth = match OAuth::import(path) {
        Some(oauth) => oauth,
        None => OAuth::obtain()?,
    };

    let harvest = Harvest::new(&oauth.access_token);

    let time_entries = harvest.get_time_entries().await;

    for entry in time_entries? {
        println!("{}", entry.id);
    }

    // Save after every operation
    oauth.export(path)?;

    Ok(())
}
