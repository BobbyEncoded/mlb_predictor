
pub fn get_html(url : &str) -> String{
    let thread = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let request = thread.block_on(async{reqwest::get(url).await});
    let response = match request {
        Ok(response) => response,
        Err(error) => panic!("Problem sending request: {:?}", error),
    };
    let text = thread.block_on(async{response.text().await});
    let grabbed_text = match text {
        Ok(text) => text,
        Err(error) => panic!("Problem creating text: {:?}", error),
    };
    grabbed_text
}

//Headers are a dictionary organized by the title of the reader, and the value is the position it appears from the left as a rank.
//The table itself is iterable though and those iterations contain rows which can be iterated through which contain the elements inside as a string.
pub fn get_table_from_html(html_text : &str) -> table_extract::Table {
    //println!("{}", html_text);
    save_string_to_file(html_text).unwrap();
    let table = table_extract::Table::find_first(html_text);
    let matched_table = table.unwrap();
    let x = matched_table.headers();
    x.iter().for_each(|(string, size)| println!("{:?} , {:?}", &string, &size));
    matched_table.iter().for_each(|x| {x.iter().for_each(|y| println!("{}", y))});
    matched_table
}

#[test]
fn test_getting_table() {
    let html = get_html(r#"https://www.teamrankings.com/nba/stat/offensive-efficiency"#);
    let data = get_table_from_html(&html);
    data.iter().for_each(|row| {
        row.iter().for_each(|element| println!("{}", element.as_str()));
    });
}

fn save_string_to_file(string_to_save : &str) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;

    let path = "lines.html";

    let mut output = File::create(path)?;
    write!(output, "{}", &string_to_save)?;

    Ok(())
}

#[allow(dead_code)]
fn remap_headers_to_vec(input_header : &std::collections::HashMap<String, usize>) -> std::vec::Vec<&str> {
    let mut unsorted_list = input_header.into_iter().map(|(header, index)| {(index, header)}).collect::<Vec<(&usize, &String)>>();
    unsorted_list.sort_by(|(a, _), (b, _)| a.cmp(b));
    unsorted_list.iter().map(|(_ , header)| header.as_str()).collect()
} 

#[test]
fn test_header_sorting() {
    use std::collections::HashMap;

    let input = HashMap::from([
        ("Team".to_string(), 1),
        ("2022".to_string(), 2),
        ("Last 1".to_string(), 4),
        ("Away".to_string(), 6),
        ("2021".to_string(), 7),
        ("Rank".to_string(), 0),
        ("Last 3".to_string(), 3),
        ("Home".to_string(), 5),
    ]);

    let result = remap_headers_to_vec(&input);
    assert_eq!(result, vec!["Rank", "Team", "2022", "Last 3", "Last 1", "Home", "Away", "2021"]); 
    result.iter().for_each(|x| println!("{}", x));
}
