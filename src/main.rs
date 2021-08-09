use proxy_json_xml::App;
use anyhow::Result;

fn main() -> Result<()> {
   let app = App::new();
   app.run()
}
