use anyhow::Result;
use std::io::BufReader;
use xml::reader::XmlEvent;
use xml::EventReader;

enum ParseState {
    Start,
    InTitle,
    InDescription,
}

pub(crate) struct Podcast {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) audio_file: Option<String>,
}

impl Podcast {
    fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            audio_file: None,
        }
    }

    pub(crate) fn to_html(&self) -> String {
        format!(
            r#"
            <html>
                <head>
                    <title>My Podcast: {}</title>
                </head>
                <body>
                    <h1>{}</h1>
                    <p>{}</p>
                    <audio controls src="{}"></audio>
                </body>
            </html>
        "#,
            self.title,
            self.title,
            self.description,
            match self.audio_file {
                Some(ref file) => file,
                None => "No audio available",
            }
        )
    }
}

pub(crate) async fn read_podcasts_from_xml(url: &str) -> Result<Vec<Podcast>> {
    let mut results = Vec::new();
    let data = reqwest::get(url).await?.text().await?;
    let parser = EventReader::new(BufReader::new(data.as_bytes()));

    let mut podcast = Podcast::new();
    let mut state = ParseState::Start;

    for event in parser {
        match event {
            // ".." used below is a "rest pattern". It allows to ignore the rest of the fields in the struct
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => match name.local_name.as_str() {
                "title" => state = ParseState::InTitle,
                "description" => state = ParseState::InDescription,
                "enclosure" => {
                    podcast.audio_file = attributes.into_iter().find_map(|attr| {
                        if attr.name.local_name == "url" {
                            Some(attr.value)
                        } else {
                            None
                        }
                    });
                }
                _ => {}
            },
            Ok(XmlEvent::CData(content)) => match state {
                ParseState::InTitle => {
                    podcast.title = content;
                    state = ParseState::Start;
                }
                ParseState::InDescription => {
                    podcast.description = content;
                    state = ParseState::Start;
                }
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "item" {
                    results.push(podcast);
                    podcast = Podcast::new();
                    state = ParseState::Start;
                }
            }
            _ => {}
        }
    }
    
    Ok(results)
}