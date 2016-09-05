extern crate serde_json;
extern crate svgdom;
extern crate barh;

use std::env;
use std::f64;
use std::fs::File;
use std::io::Write;

use svgdom::{
    Document,
    ElementId as EId,
    AttributeId as AId,
    WriteBuffer,
    NodeType,
};

use barh::font::{FontData, FontMetrics};
use barh::config::Config;
use barh::layouts::{MainLayout, CalcLayout, DrawLayout};
use barh::load_file;

macro_rules! main_try {
    ($expr:expr) => (
        match $expr {
            Ok(c) => c,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }
    )
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:\n\tbarh config.json out.svg");
        return;
    }

    // detect system font
    let system_font = main_try!(FontData::system_font());

    // load config file
    let data = main_try!(load_file(&args[1]));
    // parse json
    let value: serde_json::Value = main_try!(serde_json::from_slice(&data));
    // generate config from json
    let conf = main_try!(Config::from_value(&value, &system_font));

    // load font from config
    let fm = main_try!(FontMetrics::from_font(&conf.items_font.family, conf.items_font.size));

    // init layout
    let mut lay = MainLayout::new(&conf);
    // set debug mode
    lay.set_enable_debug(conf.debug);
    // calculate layout
    lay.calc_layout(&fm);

    // init SVG DOM
    let doc = Document::new();
    doc.append_new_node(NodeType::Declaration,
        "version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"");
    doc.append_new_node(NodeType::Comment,
        " Generated with https://github.com/RazrFalcon/barh ");

    // create root node
    let svg = doc.append_new_element(EId::Svg);
    svg.set_attribute(AId::Xmlns, "http://www.w3.org/2000/svg");
    // useful option since we only draw objects with right angles
    // may be unsupported by user agent
    svg.set_attribute(AId::ShapeRendering, "crispEdges");

    // draw layout to SVG DOM
    lay.draw_layout(&fm, 0, 0, &svg);

    // set sizes
    svg.set_attribute(AId::Width, lay.width() as f64 + 1.0);
    svg.set_attribute(AId::Height, lay.height() as f64 + 1.0);

    // write SVG to file
    let mut ouput_data = Vec::new();
    doc.write_buf(&mut ouput_data);
    let mut out_file = main_try!(File::create(&args[2]));
    main_try!(out_file.write_all(&ouput_data));
}

