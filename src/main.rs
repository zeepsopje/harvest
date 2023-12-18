use harvest::auth::OAuth;

fn main() -> anyhow::Result<()> {
    // temp path
    let path = "/home/stefan/.config/harvest.json";
    let oauth = match OAuth::import(path) {
        Some(oauth) => oauth,
        None => OAuth::obtain()?,
    };

    // Save after every operation
    oauth.export(path)?;

    Ok(())
}
