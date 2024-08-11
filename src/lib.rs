rofi_mode::export_mode!(Mode<'_>);

struct Mode<'r> {
    _api: rofi_mode::Api<'r>,
    entries: Vec<Entry>,
}

impl<'r> rofi_mode::Mode<'r> for Mode<'r> {
    const NAME: &'static str = "crun\0";

    /// Initialize the plugin
    /// - read configuration file;
    /// - create instance.
    fn init(_api: rofi_mode::Api<'r>) -> Result<Self, ()> {
        let conf_path = get_config_path();
        let entries = read_json_config(conf_path);
        let this = Self {
            _api,
            entries,
        };
        Ok(this)
    }

    /// Return the entry amount
    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    /// Return the string to be displayed for the entry
    fn entry_content(&self, line: usize) -> rofi_mode::String {
        rofi_mode::String::from(self.entries[line].name.as_str())
    }

    /// Handle events
    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {

        match event {
            rofi_mode::Event::Cancel { selected: _ } => return rofi_mode::Action::Exit,
            rofi_mode::Event::Ok { alt: _, selected } => {
                // Launch the selected entry 
                match self.entries.get(selected) {
                    Some(entry) => entry.launch(),
                    None => {}
                }
            }
            rofi_mode::Event::Complete {
                selected: Some(selected),
            } => {
                *input = self.entry_content(selected);
            }
            rofi_mode::Event::Complete { .. }
            | rofi_mode::Event::CustomInput { .. }
            | rofi_mode::Event::CustomCommand { .. }
            | rofi_mode::Event::DeleteEntry { .. } => {}
        }
        rofi_mode::Action::Reload
    }

    /// Return if an entry matches the search
    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        match self.entries.get(line) {
            Some(item) => matcher.matches(&item.name),
            None => false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    name: String,
    bin: String,
    args: Vec<String>
}

impl Entry {
    fn launch(&self) {
        Command::new(&self.bin).args(&self.args).exec();
    }
}

fn read_json_config(path: String) -> Vec<Entry> {
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(..) => panic!("Failed to open configuration file at: `{}`", &path),
    };
    let buf_reader = BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

fn get_config_path() -> String {
    let home_dir = match var("HOME") {
        Ok(dir) => dir,
        Err(..) => panic!("No $HOME environment variable!"),
    };
    format!("{}/.config/rofi/crun.json", home_dir)
}

use std::process::Command;
use std::os::unix::process::CommandExt;
use std::fs::File;
use std::io::BufReader;
use std::env::var;
use serde::{Deserialize, Serialize};
