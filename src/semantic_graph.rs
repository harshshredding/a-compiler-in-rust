use std::borrow::Cow;
use std::io::Write;
use std::process::Command;

pub type Nd = String;
pub type Ed = (String, String);

pub struct Edges(pub Vec<Ed>);

pub fn render_to<W: Write>(output: &mut W, edges: Edges) {
    dot::render(&edges, output).unwrap();
    Command::new("python").arg("viz.py").spawn().expect("graph command failed");
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Edges {
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("{}", n)).unwrap()
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for Edges {
    fn nodes(&self) -> dot::Nodes<'a, Nd> {
        // (assumes that |N| \approxeq |E|)
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());
        for (s, t) in v {
            nodes.push(s.clone());
            nodes.push(t.clone());
        }
        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Ed> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed) -> Nd { e.0.clone() }

    fn target(&self, e: &Ed) -> Nd { e.1.clone() }
}