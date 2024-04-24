#![allow(dead_code)]
use std::io::Error;

use chrono::Local;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use slug::slugify;
use std::fs::File;
use std::io::prelude::*;

const TEMPLATE: &str = r#"---
title:  "{title}"
description: ""
date: {date}
template: "{template}.html"
draft: true
taxonomies:
  {taxonomies}
---
"#;

#[derive(Debug)]
enum ContentType {
    Note,
    Post,
}

#[derive(Debug)]
struct Content {
    content_type: ContentType,
    title: String,
}

impl Content {
    fn filename(&self) -> String {
        let mut filename = String::from("content/");
        match &self.content_type {
            ContentType::Note => filename += "notes/",
            ContentType::Post => filename += "posts/",
        };
        filename + &self.slugify() + ".md"
    }

    fn slugify(&self) -> String {
        slugify(&self.title)
    }

    /// Takes the [`TEMPLATE`] and fill it in according to the fields
    /// of the [`Content`].
    fn contents(&self) -> String {
        let current_date = Local::now().to_rfc3339();
        TEMPLATE
            .replace("{title}", &self.title)
            .replace("{date}", &current_date)
            .replace(
                "{template}",
                match self.content_type {
                    ContentType::Note => "note",
                    ContentType::Post => "post",
                },
            )
            .replace(
                "{taxonomies}",
                match self.content_type {
                    ContentType::Note => "tags: []",
                    ContentType::Post => "categories: []",
                },
            )
    }

    /// Writes the generated template to disk.
    fn write_template(&self) -> Result<(), Error> {
        let path = self.filename();
        let mut file = File::create(path)?;
        file.write_all(self.contents().as_bytes())?;
        Ok(())
    }
}

fn main() {
    let options = vec!["Note", "Post"];
    let note_or_post = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want a note or a post")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    let content_type = match note_or_post {
        0 => ContentType::Note,
        1 => ContentType::Post,
        _ => unreachable!(),
    };

    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the title?")
        .interact_text()
        .unwrap();

    let content = Content {
        content_type,
        title,
    };

    content
        .write_template()
        .expect("could not write the file, gosh darn it!");

    print!(
        "done kind sir, wrote the file for you here: {}",
        content.filename()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_filename_returns_expected_value() {
        let content = Content {
            content_type: ContentType::Note,
            title: String::from("Test Title"),
        };

        assert_eq!(content.filename(), "content/notes/test-title.md");
    }

    #[test]
    fn content_slugify_returns_expected_value() {
        let content = Content {
            content_type: ContentType::Note,
            title: String::from("Test Title"),
        };

        assert_eq!(content.slugify(), "test-title");
    }

    #[test]
    fn content_contents_returns_expected_value() {
        let content = Content {
            content_type: ContentType::Note,
            title: String::from("Test Title"),
        };
        let current_date = Local::now().to_rfc3339();

        let contents = content.contents();
        assert!(contents.contains(&format!("date: {}", current_date)));
        assert!(contents.contains("title:  \"Test Title\""));
        assert!(contents.contains("template: \"note.html\""));
        assert!(contents.contains("tags: []"));
    }
}
