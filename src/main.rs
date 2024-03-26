use std::{process::{Command, self}, io::{Cursor, Write}, borrow::Cow, fs::File};
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

    #[arg(long, short)]
    volume: Option<String>,

    #[arg(long, short)]
    container: Option<String>,

    #[arg(long, default_value_t=false)]
    standalone: bool,
}

fn main() {
    let args = Cli::parse();
    println!("The first argument is {:?}", args);

    let tmp_path = format!("/tmp/{}.xml", Uuid::new_v4());

    let paths = match args.volume {
        None => vec![args.path.to_string(), args.path.to_string()],
        Some(v) => {
            let paths: Vec<_> = v.split(":").collect();
            if paths.len() < 2 {
                panic!("Unable to parse volume")
            }
            vec![paths[0].to_owned(), paths[1].to_owned()]
        },
    };

    let file = args.path.replace(&paths[0], &paths[1]);

    let container = match args.container {
        Some(c) => c,
        None => "php".to_string(),
    };

    let mut binding = Command::new("docker");
    binding.args([
        if args.standalone { "" } else {"compose"},
        "exec",
        &container,
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
    println!("{:?}", binding.get_args());

    binding.output().expect("Failed");

    Command::new("docker").args([
        if args.standalone { "" } else {"compose"},
        "cp",
        &format!("{}:{}", &container, &tmp_path),
        &args.log_junit,
    ]).output().expect("Failed");

    let buffer = match fs::read_to_string(&args.log_junit) {
        Ok(b) => b,
        Err(_) => {
            eprintln!("Unable to find the local junit file");
            process::exit(1)
        }
    };

    let mut xml_reader = Reader::from_str(&buffer);

    let mut xml_wtitter = Writer::new(Cursor::new(Vec::new()));

    loop {
        match xml_reader.read_event() {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                if let Ok(Some(attr)) = e.try_get_attribute("file") {
                    let elem = replace_file_attr(&e, &attr, &paths);

                    if e.name().as_ref() == b"testcase" {
                        assert!(xml_wtitter.write_event(Event::Empty(elem)).is_ok());
                    } else {
                        assert!(xml_wtitter.write_event(Event::Start(elem)).is_ok());
                    }

                    continue;
                }

                assert!(xml_wtitter.write_event(Event::Start(e)).is_ok());
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

    process::exit(0)
}

fn replace_file_attr(e: &BytesStart, attr: &Attribute, paths: &Vec<String>) -> BytesStart<'static> {
    let a_path = String::from_utf8(attr.value.to_vec()).unwrap()
        .replace(&paths[1], &paths[0]); // Replace in the reverse order

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
