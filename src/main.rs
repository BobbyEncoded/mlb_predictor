mod grab_data_from_html;
mod sqlite_storage;

use grab_data_from_html::{get_html, get_table_from_html};
const PAGETOCHECK: &str = r#"https://www.teamrankings.com/mlb/stat/opponent-runs-per-game?date=2022-05-18"#;
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
    println!("Hello, world!");
    let html = get_html(PAGETOCHECK);
    //println!("{:?}", html);
    get_table_from_html(&html);
}
