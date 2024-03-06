use std::{process::{Command, exit}, io::{Cursor, Write}, borrow::Cow, fs::File};
use std::fs;
use clap::Parser;
use quick_xml::{Reader, Writer, events::{Event, BytesStart}};
use uuid::Uuid;

#[derive(Parser, Debug)] // requires `derive` feature
struct Cli {
    #[arg()]
    path: String,

    #[arg(long)]
    log_junit: String,

    #[arg(long)]
    filter: String,
}

fn main() {
    let args = Cli::parse();
    println!("The first argument is {:?}", args);

    let tmp_path = format!("/tmp/{}.xml", Uuid::new_v4());
    let file = args.path.replace("/Users/praem90/projects/track-payments", "/app");

    println!("tmp_path : {}", tmp_path);

    Command::new("docker").args([
        "compose",
        "exec",
        "php",
        "./vendor/bin/phpunit",
        "--no-coverage",
        "--log-junit",
        &tmp_path,
        "--filter",
        &args.filter,
        &file,
    ]).output().expect("Failed");

    Command::new("docker").args([
        "compose",
        "cp",
        &format!("php:{}", &tmp_path),
        &args.log_junit,
    ]).output().expect("Failed");

    let result = {
        let buffer = fs::read_to_string(&args.log_junit).unwrap();
        let mut xml_reader = Reader::from_str(&buffer);

        let mut xml_wtitter = Writer::new(Cursor::new(Vec::new()));

        loop {
            match xml_reader.read_event() {
                Ok(Event::Start(e)) => {
                    if let Ok(Some(mut attr)) = e.try_get_attribute("file") {
                        let a_path = String::from_utf8(attr.value.to_vec()).unwrap()
                            .replace("/app", "/Users/praem90/projects/track-payments");

                        attr.value = Cow::Borrowed(a_path.as_bytes());

                        let mut elem = match e.name().as_ref() {
                            b"testsuite" => BytesStart::new("testsuite"),
                            b"testcase" => BytesStart::new("testcase"),
                            _ => BytesStart::new("Unknown"),
                        };


                        // collect existing attributes
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()).filter(|a| {
                            a.key.as_ref() != b"file".as_ref()
                        }));

                        // copy existing attributes, adds a new my-key="some value" attribute
                        elem.push_attribute(attr);
                        // writes the event to the writer
                        assert!(xml_wtitter.write_event(Event::Start(elem)).is_ok());
                    } else {
                        assert!(xml_wtitter.write_event(Event::Start(e)).is_ok());
                    }
                },
                Ok(Event::Empty(e)) => {
                    if let Ok(Some(mut attr)) = e.try_get_attribute("file") {
                        let a_path = String::from_utf8(attr.value.to_vec()).unwrap()
                            .replace("/app", "/Users/praem90/projects/track-payments");

                        attr.value = Cow::Borrowed(a_path.as_bytes());

                        let mut elem = match e.name().as_ref() {
                            b"testsuite" => BytesStart::new("testsuite"),
                            b"testcase" => BytesStart::new("testcase"),
                            _ => BytesStart::new("Unknown"),
                        };

                        // collect existing attributes
                        elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()).filter(|a| {
                            a.key.as_ref() != b"file".as_ref()
                        }));

                        // copy existing attributes, adds a new my-key="some value" attribute
                        elem.push_attribute(attr);
                        assert!(xml_wtitter.write_event(Event::Empty(elem)).is_ok())
                    } else {
                        assert!(xml_wtitter.write_event(Event::Empty(e)).is_ok())
                    }
                },
                Ok(Event::End(e)) if e.name().as_ref() == b"testsuite" => {
                    assert!(xml_wtitter.write_event(Event::End(e)).is_ok());
                },
                Ok(Event::Eof) => break,
                Ok(e) => {
                    println!("{:?}", e);
                    assert!(xml_wtitter.write_event(e).is_ok())
                },
                Err(_) => panic!("Unable to read"),
            }
        }

        let result = xml_wtitter.into_inner().into_inner();
        result
    };

    fs::remove_file(&args.log_junit).unwrap();
    let mut new_file = File::create(&args.log_junit).unwrap();

    new_file.write_all(&result).unwrap();

    exit(0)
}
