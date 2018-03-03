use svgdom::{
    ElementId as EId,
    AttributeId as AId,
    NodeType,
    Node,
};

use font::FontMetrics;

pub trait Adaptor {
    fn append_rect(&mut self, x: u32, y: u32, w: u32, h: u32) -> Node;
    fn append_text(&mut self, text: &str, x: u32, y: u32, fm: &FontMetrics) -> Node;
    fn append_hline(&mut self, x: u32, y: u32, w: u32) -> Node;
    fn append_vline(&mut self, x: u32, y: u32, h: u32) -> Node;
}

impl Adaptor for Node {
    fn append_rect(&mut self, x: u32, y: u32, w: u32, h: u32) -> Node {
        let mut rect = self.document().create_element(EId::Rect);
        self.append(&rect);

        rect.set_attribute((AId::X, x as f64));
        rect.set_attribute((AId::Y, y as f64));
        rect.set_attribute((AId::Width, w as f64));
        rect.set_attribute((AId::Height, h as f64));
        rect.set_attribute((AId::Fill, "none"));
        rect.clone()
    }

    fn append_text(&mut self, text: &str, x: u32, y: u32, fm: &FontMetrics) -> Node {
        let mut doc = self.document();
        let mut text_elem = doc.create_element(EId::Text);
        self.append(&text_elem);

        let text_node = doc.create_node(NodeType::Text, text);
        text_elem.append(&text_node);

        text_elem.set_attribute((AId::X, x as f64));
        text_elem.set_attribute((AId::Y, y as f64));
        text_elem.set_attribute((AId::FontFamily, fm.family()));
        text_elem.set_attribute((AId::FontSize, fm.height() as f64));
        text_elem.clone()
    }

    fn append_vline(&mut self, x: u32, y: u32, h: u32) -> Node {
        let mut rect = self.document().create_element(EId::Rect);
        self.append(&rect);

        rect.set_attribute((AId::X, x as f64));
        rect.set_attribute((AId::Y, y as f64));
        rect.set_attribute((AId::Width, 1));
        rect.set_attribute((AId::Height, h as f64));
        rect.clone()
    }

    fn append_hline(&mut self, x: u32, y: u32, w: u32) -> Node {
        let mut rect = self.document().create_element(EId::Rect);
        self.append(&rect);

        rect.set_attribute((AId::X, x as f64));
        rect.set_attribute((AId::Y, y as f64));
        rect.set_attribute((AId::Width, w as f64));
        rect.set_attribute((AId::Height, 1));
        rect.clone()
    }
}

// fn append_script(parent: &Node) {
//     let doc = parent.document();
//     let text = doc.create_element(EId::Script);

//     parent.append(&text);

//     let text = "
//         var t = document.getElementById('text');
//         var b = t.getBBox();
//         var e_bbox = document.getElementById('bbox');
//         e_bbox.setAttribute('width', b.width);
//         e_bbox.setAttribute('height', b.height);
//         e_bbox.setAttribute('x', b.x);
//         e_bbox.setAttribute('y', b.y);
//         console.log(b)
//     ";

//     let text_node1 = doc.create_node(NodeType::Text, text);
//     text.append(&text_node1);
// }
