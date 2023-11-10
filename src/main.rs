

use std::env;

use miette::{IntoDiagnostic, Context,miette};

use quick_xml::{Reader, events::Event}; 

#[derive(PartialEq, Eq, Debug)]
enum Tag {
    Source,
    Translation,
    Other
}




fn main() -> miette::Result<()> {

    let xml = load_file( env::args().skip(1).next()
        .ok_or_else(||miette!("Argument 1 should be filename!"))?.as_str() )?;

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut tag = Tag::Other;
    let mut had_translation = false;

    loop {
        match reader.read_event().into_diagnostic()? {
            Event::Eof => break,
            Event::Start(e) => { 
                    tag = match e.name().as_ref() {
                        b"TS" => {
                            println!("{}",String::from_utf8_lossy(e.attributes_raw()));
                            continue;
                        }
                        b"message" => { println!();had_translation=false; continue }
                        b"source" => Tag::Source,
                        b"translation" => Tag::Translation,
                        _ => Tag::Other
                    };
            },
            Event::Text(t) => { 
                if tag != Tag::Other {
                    let text = t.unescape().into_diagnostic()?;
                    let mut html = Reader::from_str(text.as_ref());
                    html.trim_text(true);
                    if tag == Tag::Source {
                        println!("- {:?}:",&tag);
                    } else { 
                        println!("  {:?}:",&tag);
                    }
                    loop {
                        match html.read_event() {
                            Ok(Event::Start(e)) => {
                                if e.name().as_ref() == b"style" {
                                    let _ = html.read_text(e.name());
                                }
                            }
                            Ok(Event::Text(t)) =>{
                                for s in t.unescape().into_diagnostic()?.as_ref().split('\n') {
                                    println!("    {s}");
                                    if tag == Tag::Translation {had_translation=true;}
                                }
                            } 
                            Ok(Event::Eof) => break,
                            _html_other => (),//{dbg!(html_other);}
                        }
                    }
                } 
            },
            Event::End(e) => {
                if e.name().as_ref() == b"message" && !had_translation {
                    println!("  Translation:");
                }
            }
            _other => (),//{dbg!(other);} 
        };
    };

    return  Ok(());
}

fn load_file(path:&str) -> Result<String, miette::ErrReport> {
    std::fs::read_to_string(path)
        .into_diagnostic().with_context(||format!("Error in loading {path}!"))
}
