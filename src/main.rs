mod api;
mod model;
mod ui;
use crate::ui::gui;

//REWRITE TO BE LESS SPAGHETTI EVENTUALLY
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Please paste in your token. If you don't know what that is, please google");
    let mut token = String::new();
    std::io::stdin()
        .read_line(&mut token)
        .expect("Could not read input");
    token.retain(|c| !c.is_whitespace()); //get rid of any whitespace

    //Simplified way of passing token and client
    api::gateway_thread::start_thread(&token).await

    //gui::summon_gooey().await.expect("Could not run the main script. Possibly incorrect token.");
}
