use std::fs::{self, OpenOptions};
use std::io::{self, Write};

/*  
  Maybe you can propagate the error on the function save_url().
*/

pub fn save_url(url: String, r#type: String, desc: String) -> Result<(), String> {
  let url_path: String = format!("./urls/{type}.txt");

  let file = OpenOptions::new()
  .append(true)
  .create(true)
  .write(true)
  .open(url_path);

  match file {
    Ok(mut f)  => {
      let status = writeln!(f, "{url} <- {desc}");
      
      match status {
        Ok(_) => Ok(()),
        Err(e) => Err(e.kind().to_string()),
      }
    },
    Err(e) => Err(e.kind().to_string()),
  }
}

pub fn show_url(r#type: String) -> io::Result<()> {
  let url_path: String = format!("./urls/{type}.txt");
  let content: String = fs::read_to_string(url_path)?;

  println!("{}", content);

  return Ok(());
}