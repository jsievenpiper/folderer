use colored::Colorize;

fn main() -> std::io::Result<()> {
  let root_dir = std::env::current_dir()?;
  let ls = std::process::Command::new("/bin/ls")
    .current_dir(&root_dir)
    .output()?;

  let movies = regex::Regex::new(r#"(?xm)
    (?P<title>.*)
    \s?
    \(
    (?P<year>\d{4})
    \)
    (?P<rest>[-A-Za-z0-9]*)?
    \.
    (?P<extension>[a-zA-Z0-9\.]{2,})
  "#).expect("wow I can't even regex correctly");

  let files = String::from_utf8(ls.stdout)
    .expect("definitely utf-8 for my purposes")
    .split("\n")
    .filter(|it| !it.is_empty())
    .map(|it| String::from(it))
    .collect::<Vec<String>>();

  for file in &files {
    let captures = movies.captures(file.as_str());

    if let Some(captures) = captures {
      let title = captures.name("title").expect("couldn't extract title");
      let year = captures.name("year").expect("couldn't extract year");

      let mut folder_path = root_dir.clone();
      folder_path.push(format!("{} ({})", title.as_str(), year.as_str()));

      let mut file_path = root_dir.clone();
      file_path.push(file.as_str());

      std::process::Command::new("/bin/mkdir")
        .arg("-p")
        .arg(folder_path.as_os_str())
        .status()?;

      std::process::Command::new("/bin/mv")
        .args(&[file_path.as_os_str(), folder_path.as_os_str()])
        .status()?;
    }

    else {
      let mut folder_check = root_dir.clone();
      folder_check.push(file.as_str());

      if !folder_check.is_dir() {
        eprintln!("{} {}", "FAIL".on_red().white().bold(), file.as_str());
      }
    }
  }

  Ok(())
}
