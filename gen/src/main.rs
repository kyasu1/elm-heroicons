use base64;
use glob::glob;
use inflector::cases::camelcase::to_camel_case;
use inflector::Inflector;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;
use svg::node::element::tag::{Path, Type, SVG};
use svg::parser::Event;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match generate(&args[1]) {
        Ok(()) => println!("Conversion success {}", &args[1]),
        Err(err) => println!("Error {:?}", err),
    }

    //    generate("solid");
}

fn generate(category: &str) -> std::io::Result<()> {
    let dir = format!("./heroicons/src/{}/*.svg", category);
    let files: Vec<std::path::PathBuf> = glob(&dir)
        .expect("Could not find svg files")
        .filter_map(Result::ok)
        .collect();

    if files.len() == 0 {
        panic!("No SVG file unser the specified directory");
    }
    let elm_module_name = category.to_title_case();

    let mut output =
        File::create(format!("{}.elm", elm_module_name)).expect("Could not create output file");
    output.write_all(header(elm_module_name, &files).as_bytes())?;

    for file in files {
        let mut svg: String = String::new();
        let mut children: Vec<String> = Vec::new();

        let mut f = File::open(file.clone())?;
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;

        // 読みこんだSVG形式のデータをbase64エンコードしてimgタグで表示できるようにする
        let encoded_svg = base64::encode(&buffer);

        // SVG形式のデータをパースしてElmのコードに変換する
        for event in svg::read(&buffer[..])? {
            match event {
                Event::Tag(SVG, Type::Start, attributes) => {
                    let attrs = attributes
                        .iter()
                        .map(|(key, val)| {
                            if key == "width" {
                                "".to_string()
                            } else if key == "height" {
                                "".to_string()
                            } else if key == "xmlns" {
                                "".to_string()
                            } else {
                                format!("A.{} \"{}\"", key.to_camel_case(), val)
                            }
                        })
                        .filter(|item| item != "")
                        .collect::<Vec<String>>()
                        .join(", ");
                    svg = format!("{}", attrs);
                }
                // Event::Tag(SVG, Type::End, _) => {
                //     println!("]  ");
                // }
                Event::Tag(Path, _, attributes) => {
                    let attrs = attributes
                        .iter()
                        .map(|(key, val)| {
                            if key == "stroke" {
                                String::from("A.stroke \"currentColor\"")
                            } else {
                                format!("A.{} \"{}\"", key.to_camel_case(), val)
                            }
                        })
                        .filter(|item| item != "")
                        .collect::<Vec<String>>()
                        .join("\n            , ");
                    children.push(format!(
                        "path\n            [ {} \n            ]\n            [] ",
                        attrs
                    ));
                }
                _ => {}
            }
        }

        let name = file
            .file_stem()
            .and_then(|file_name| file_name.to_str())
            .map(to_camel_case)
            .expect("Invalid filename");
        write_font(
            &mut output,
            name,
            encoded_svg,
            svg,
            children.join("\n        , "),
        )?;
    }
    Ok(())
}
fn header(category: String, files: &std::vec::Vec<std::path::PathBuf>) -> String {
    let mut names: Vec<String> = Vec::new();

    for file in files {
        names.push(format!(
            "{}",
            file.file_stem()
                .and_then(|stem| stem.to_str())
                .map(|name| name.to_camel_case())
                .unwrap()
        ));
    }

    let header = format!(
        r#"module Heroicons.{} exposing ({})

import Svg exposing (Svg, path, svg, Attribute)
import Svg.Attributes as A

"#,
        category.to_title_case(),
        names.join(", ")
    );

    header
}

fn write_font(
    file: &mut File,
    name: String,
    encoded_svg: String,
    parent: String,
    children: String,
) -> std::io::Result<()> {
    let buf = format!(
        r#"
{{-| {name}

![image](data:image/svg+xml;base64,{encoded_svg})

-}}
{name} : List (Attribute msg) -> Svg msg
{name} attrs =
    svg ( [ {parent} ] ++ attrs )
        [ {children}
        ]

"#,
        encoded_svg = encoded_svg,
        parent = parent,
        children = children,
        name = name
    );

    file.write_all(buf.as_bytes())?;
    Ok(())
}
