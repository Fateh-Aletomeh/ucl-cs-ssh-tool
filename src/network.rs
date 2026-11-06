use crate::{Machine, Status};
use scraper::{Html, Selector};


pub async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
  reqwest::get(url).await?.error_for_status()?.text().await
}


pub fn parse_machines(html: &str) -> (Vec<Machine>, Vec<Machine>) {
  let document = Html::parse_document(html);
  let row_sel = Selector::parse("tr").unwrap();
  let td_sel = Selector::parse("td").unwrap();

  let mut lab_105 = Vec::new();
  let mut lab_121 = Vec::new();

  for row in document.select(&row_sel).skip(1) {
    let cells: Vec<_> = row
        .select(&td_sel)
        .map(|td| td.text().collect::<String>().trim().to_lowercase())
        .collect();

    // Process cells in pairs: [name, status]
    for (i, pair) in cells.chunks(2).enumerate() {
      if pair.len() == 2 && !pair[0].is_empty() {
        let status = match pair[1].as_str() {
          "linux" => Status::Linux,
          "windows" => Status::Windows,
          _ => Status::Down
        };
        
        let machine = Machine {
          name: pair[0].clone(),
          status,
        };
  
        match i {
          0 => lab_105.push(machine),
          1 => lab_121.push(machine),
          _ => {}
        }
      }
    }
  }

  (lab_105, lab_121)
}
