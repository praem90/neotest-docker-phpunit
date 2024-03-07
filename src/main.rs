use std::{process::{Command, exit}, io::{Cursor, Write}, borrow::Cow, fs::File};
use quick_xml::{Reader, Writer, events::{Event, BytesStart, attributes::Attribute}};
use clap::Parser;
use uuid::Uuid;
use std::fs;

#[derive(Parser, Debug)] // requires `derive` feature
struct Cli {
    #[arg()]
    path: String,

    #[arg(long)]
    log_junit: String,

    #[arg(long)]
    filter: Option<String>,
}

fn main() {
    let args = Cli::parse();
    println!("The first argument is {:?}", args);

    let tmp_path = format!("/tmp/{}.xml", Uuid::new_v4());
    let file = args.path.replace("/Users/praem90/projects/track-payments", "/app");

    println!("tmp_path : {}", tmp_path);

    let mut binding = Command::new("docker");
    binding.args([
        "compose",
        "exec",
        "php",
        "./vendor/bin/phpunit",
        "--no-coverage",
        "--log-junit",
        &tmp_path,
    ]);

    if let Some(filter) = args.filter {
        binding.arg("--filter");
        binding.arg(&filter);
    }

    binding.arg(&file);

    binding.output().expect("Failed");

    Command::new("docker").args([
                                "compose",
                                "cp",
                                &format!("php:{}", &tmp_path),
                                &args.log_junit,
    ]).output().expect("Failed");

    let buffer = fs::read_to_string(&args.log_junit).unwrap();
    let mut xml_reader = Reader::from_str(&buffer);

    let mut xml_wtitter = Writer::new(Cursor::new(Vec::new()));

    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(e)) => {
                if let Ok(Some(attr)) = e.try_get_attribute("file") {
                    let elem = replace_file_attr(&e, &attr);
                    assert!(xml_wtitter.write_event(Event::Start(elem)).is_ok());
                } else {
                    assert!(xml_wtitter.write_event(Event::Start(e)).is_ok());
                }
            },
            Ok(Event::Empty(e)) => {
                if let Ok(Some(attr)) = e.try_get_attribute("file") {
                    let elem = replace_file_attr(&e, &attr);
                    assert!(xml_wtitter.write_event(Event::Empty(elem)).is_ok());
                } else {
                    assert!(xml_wtitter.write_event(Event::Start(e)).is_ok());
                }
            },
            Ok(Event::End(e)) => {
                assert!(xml_wtitter.write_event(Event::End(e)).is_ok());
            },
            Ok(Event::Eof) => break,
            Ok(e) => {
                assert!(xml_wtitter.write_event(e).is_ok())
            },
            Err(_) => panic!("Unable to read"),
        }
    }

    let result = xml_wtitter.into_inner().into_inner();

    fs::remove_file(&args.log_junit).unwrap();
    let mut new_file = File::create(&args.log_junit).unwrap();

    new_file.write_all(&result).unwrap();

    println!("{}", String::from_utf8(result).unwrap());

    exit(0)
}

fn replace_file_attr(e: &BytesStart, attr: &Attribute) -> BytesStart<'static> {
    let a_path = String::from_utf8(attr.value.to_vec()).unwrap()
        .replace("/app", "/Users/praem90/projects/track-payments");

    let mut elem = match e.name().as_ref() {
        b"testsuite" => BytesStart::new("testsuite"),
        b"testcase" => BytesStart::new("testcase"),
        _ => BytesStart::new("Unknown"),
    };

    elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()).filter(|a| {
        a.key.as_ref() != b"file".as_ref()
    }));

    elem.push_attribute(Attribute{
        key: attr.key,
        value: Cow::Borrowed(&a_path.as_bytes())
    });

    elem
}
