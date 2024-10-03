use crate::petri_net::{Arrow, PetriNet, Place, Transition};
use crate::zblob::Zblob;
use crate::{Model, Vector};
use base64::Engine;
use std::io::Write;
use std::sync::{Arc, Mutex};

// Define the ImageBuilder trait
pub trait ImageBuilder {
    fn new_svg_image(&self, width: Option<i32>, height: Option<i32>);
    fn write_defs(&self, buffer: &mut Vec<u8>);
    fn rect(&self, x: i32, y: i32, width: i32, height: i32, extra: &str);
    fn circle(&self, x: i32, y: i32, radius: i32, extra: &str);
    fn text(&self, x: i32, y: i32, text: &str, extra: &str);
    fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32, extra: &str);
    fn group(&self);
    fn gend(&self);
    fn write_element(&self, element: String);
    fn render(&self, initial_vectors: Vec<Vector>);
    fn place(&self, label: String, place: &Place);
    fn arc(&self, net: &PetriNet, arc: &Arrow);
    fn transition(&self, label: String, transition: &Transition);
    fn end(&self);
}

// Define the ImageOutput trait
pub trait ImageOutput {
    fn encode_url_component(component: &str) -> String;
    fn to_base64_url(&self) -> String;
    fn to_data_url(&self) -> String;
    fn to_img_tag(&self) -> String;
    fn to_zblob(&self) -> Zblob;
    fn to_html(&self) -> String;
}

pub struct Display {
    buffer: Arc<Mutex<Vec<u8>>>,
    model: Model,
}

impl Display {
    fn new(model: Model) -> Self {
        Self {
            model,
            buffer: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

// Implement the ImageBuilder trait for Display
impl ImageBuilder for Display {
    fn new_svg_image(&self, width: Option<i32>, height: Option<i32>) {
        let w = width.unwrap_or(400);
        let h = height.unwrap_or(400);
        let mut buffer = self.buffer.lock().unwrap();
        write!(
            buffer,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\">"
        )
        .unwrap();
        self.write_defs(&mut buffer);
    }

    fn write_defs(&self, buffer: &mut Vec<u8>) {
        write!(
            buffer,
            "<defs><marker id=\"markerArrow1\" markerWidth=\"23\" markerHeight=\"13\" refX=\"31\" refY=\"6\" orient=\"auto\"><rect width=\"28\" height=\"3\" fill=\"white\" stroke=\"white\" x=\"3\" y=\"5\"/><path d=\"M2,2 L2,11 L10,6 L2,2\"/></marker><marker id=\"markerInhibit1\" markerWidth=\"23\" markerHeight=\"13\" refX=\"31\" refY=\"6\" orient=\"auto\"><rect width=\"28\" height=\"3\" fill=\"white\" stroke=\"white\" x=\"3\" y=\"5\"/><circle cx=\"5\" cy=\"6.5\" r=\"4\"/></marker></defs>"
        ).unwrap();
    }

    fn rect(&self, x: i32, y: i32, width: i32, height: i32, extra: &str) {
        self.write_element(format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" {} />",
            x, y, width, height, extra
        ));
    }

    fn circle(&self, x: i32, y: i32, radius: i32, extra: &str) {
        self.write_element(format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" {} />",
            x, y, radius, extra
        ));
    }

    fn text(&self, x: i32, y: i32, text: &str, extra: &str) {
        self.write_element(format!(
            "<text x=\"{}\" y=\"{}\" {}>{}</text>",
            x, y, extra, text
        ));
    }

    fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32, extra: &str) {
        self.write_element(format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" {} />",
            x1, y1, x2, y2, extra
        ));
    }

    fn group(&self) {
        self.write_element("<g>".to_string());
    }

    fn gend(&self) {
        self.write_element("</g>".to_string());
    }

    fn write_element(&self, element: String) {
        let mut buffer = self.buffer.lock().unwrap();
        write!(buffer, "{}", element).unwrap();
    }

    fn render(&self, _initial_vectors: Vec<Vector>) {
        let net = &self.model.net;
        for arc in &net.arcs {
            self.arc(&net, arc);
        }
        for (label, place) in &net.places {
            self.place(label.to_string(), place);
        }
        for (label, transition) in &net.transitions {
            self.transition(label.to_string(), transition);
        }
        self.end();
    }

    fn place(&self, label: String, place: &Place) {
        self.group();
        self.circle(place.x, place.y, 16, "stroke-width=\"1.5\" fill=\"#ffffff\" stroke=\"#000000\" orient=\"0\" shapeRendering=\"auto\"");
        self.text(place.x - 18, place.y - 20, &label, "font-size=\"small\"");
        self.gend();
    }

    fn arc(&self, net: &PetriNet, arc: &Arrow) {
        self.group();
        let marker = if arc.inhibit.unwrap_or(false) {
            "url(#markerInhibit1)"
        } else {
            "url(#markerArrow1)"
        };
        let extra = format!(
            "stroke=\"#000000\" fill=\"#000000\" marker-end=\"{}\"",
            marker
        );

        let place = net.places.get(&arc.source);
        let transition = net.transitions.get(&arc.target);
        if place.is_none() {
            let p = net.places.get(&arc.target).expect("Place not found");
            let t = net
                .transitions
                .get(&arc.source)
                .expect("Transition not found");
            self.line(t.x, t.y, p.x, p.y, &extra);
        } else {
            let p = place.expect("Place not found");
            let t = transition.expect("Transition not found");
            self.line(p.x, p.y, t.x, t.y, &extra);
        }
        self.gend();
    }

    fn transition(&self, label: String, transition: &Transition) {
        self.group();
        let x = transition.x - 17;
        let y = transition.y - 17;
        self.rect(x, y, 30, 30, "stroke=\"#000000\" fill=\"#ffffff\" rx=\"4\"");
        self.text(x, y - 8, &label, "font-size=\"small\"");
        self.gend();
    }

    fn end(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        write!(buffer, "</svg>").unwrap();
    }
}

// Implement the ImageOutput trait for Display
impl ImageOutput for Display {
    fn encode_url_component(component: &str) -> String {
        component
            .chars()
            .map(|c| match c {
                ' ' => "%20".to_string(),
                '!' => "%21".to_string(),
                '"' => "%22".to_string(),
                '#' => "%23".to_string(),
                '$' => "%24".to_string(),
                '%' => "%25".to_string(),
                '&' => "%26".to_string(),
                '\'' => "%27".to_string(),
                '(' => "%28".to_string(),
                ')' => "%29".to_string(),
                '*' => "%2A".to_string(),
                '+' => "%2B".to_string(),
                ',' => "%2C".to_string(),
                '/' => "%2F".to_string(),
                ':' => "%3A".to_string(),
                ';' => "%3B".to_string(),
                '<' => "%3C".to_string(),
                '=' => "%3D".to_string(),
                '>' => "%3E".to_string(),
                '?' => "%3F".to_string(),
                '@' => "%40".to_string(),
                '[' => "%5B".to_string(),
                '\\' => "%5C".to_string(),
                ']' => "%5D".to_string(),
                '^' => "%5E".to_string(),
                '`' => "%60".to_string(),
                '{' => "%7B".to_string(),
                '|' => "%7C".to_string(),
                '}' => "%7D".to_string(),
                '~' => "%7E".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }

    fn to_base64_url(&self) -> String {
        let buffer = self.buffer.lock().unwrap().clone();
        let base64_svg = base64::engine::general_purpose::STANDARD.encode(&buffer);
        format!("data:image/svg+xml;base64,{}", base64_svg)
    }

    fn to_data_url(&self) -> String {
        let buffer = self.buffer.lock().unwrap();
        let encoded = Self::encode_url_component(&String::from_utf8_lossy(&buffer));
        format!("data:image/svg+xml,{}", encoded)
    }

    fn to_img_tag(&self) -> String {
        format!("<img src=\"{}\" />", self.to_data_url())
    }

    fn to_zblob(&self) -> Zblob {
        self.model.net.to_zblob()
    }

    fn to_html(&self) -> String {
        let zblob = self.to_zblob();
        let cid = zblob.ipfs_cid;
        let zipped_data = zblob.base64_zipped;
        let image = self.to_img_tag();
        let pretty_json = serde_json::to_string_pretty(&self.model.net.to_json().unwrap()).unwrap();
        let model_type = self.model.net.model_type.clone();

        format!(
            r#"<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width,initial-scale=1"/>
                <title>pflow.xyz | metamodel viewer</title>
                <script>
                    sessionStorage.cid = "{cid}";
                    sessionStorage.data = "{zipped_data}";
                </script>
            </head>
            <body>
                <h5>{model_type}:{cid}</h5>
                <a href="https://pflow.dev/?z={zipped_data}">{image}</a>
                <br/>
                <textarea id="svg" style="position: absolute; bottom: 0; height: 60%; width: 98%; padding: 0 5px;">
                    {pretty_json}
                </textarea>
            </body>
        </html>"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_html() {
        let model = Model::new(|p| {
            p.model_type("petriNet");
            p.cell("place0", Option::from(0), Option::from(3), 100, 180);
            p.func("txn0", "default", 20, 100);
            p.func("txn1", "default", 180, 100);
            p.func("txn2", "default", 20, 260);
            p.func("txn3", "default", 180, 260);
            p.arrow("txn0", "place0", 1);
            p.arrow("place0", "txn1", 3);
            p.guard("txn2", "place0", 3);
            p.guard("place0", "txn3", 1);
        });

        let svg = Display::new(model);
        svg.new_svg_image(None, None);
        svg.render(Vec::new());
        println!("{}", svg.to_html());

        if std::env::var("WRITE_TO_FILE").is_ok() {
            let mut file = std::fs::File::create("/tmp/test.html").unwrap();
            file.write_all(svg.to_html().as_bytes()).unwrap();
        }
    }
}
