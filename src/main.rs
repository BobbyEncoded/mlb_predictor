mod grab_data_from_html;
mod sqlite_storage;

use grab_data_from_html::{get_html, get_table_from_html};
const PAGETOCHECK: &str = r#"https://www.teamrankings.com/mlb/stat/opponent-runs-per-game?date=2022-05-18"#;
const PAGETOCHECK2: &str = r#"https://www.teamrankings.com/nba/stat/offensive-efficiency?date=2022-07-04"#;
const PAGE_HEADER: &str = r"https://www.teamrankings.com/nba/stat/";
static PAGES: &'static [&str] = &["offensive-efficiency", "effective-field-goal-pct", "ftm-per-100-possessions", "offensive-rebounding-pct", "assist--per--turnover-ratio", "defensive-efficiency"];

/*
NBA Team Stats
- Offensive Efficiency
- Effective Field Goal %
- FTM per 100 Possessions
- Offensive Rebounding %
- Assist / Turnover Ratio
- Defensive Efficiency
Ignore for now - Odds & Trends -> Team Trends -> Cover % from the Season
*/

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let html = get_html(PAGETOCHECK2);
        let table = "offensive-officiency";
        //println!("{:?}", html);
        let collected_table = get_table_from_html(&html);
        let mut database = sqlite_storage::connect_and_create_database_if_not_exists().await.unwrap();
        let mapped_headers = grab_data_from_html::remap_headers_to_vec(&collected_table.headers());
        let establish_table = sqlite_storage::create_table(&mut database, &table, &mapped_headers).await.unwrap();

    });
}
