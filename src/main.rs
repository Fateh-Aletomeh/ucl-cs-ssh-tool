mod model;
use model::{Machine, Status};
use scraper::{Html, Selector};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let url = "https://tsg.cs.ucl.ac.uk/cs-labs/cslabs.html";
  let html = fetch_html(url).await?;

  let (lab_105, lab_121) = parse_machines(&html);
  
  Ok(())
}


fn parse_machines(html: &str) -> (Vec<Machine>, Vec<Machine>) {
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
          "down" => Status::Down,
          _ => Status::Unknown
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

  return (lab_105, lab_121)
}


async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
  reqwest::get(url).await?.error_for_status()?.text().await
}
