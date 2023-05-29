pub trait Twistable {
    fn twist(&self) -> Twist;
}

#[derive(Clone, Debug)]
pub enum Twist {
    ObjNode(String, Vec<Twist>),
    ArrayNode(String, Vec<Twist>),
    AttrNode(String, String),
    ValueNode(String, Option<Box<Twist>>),
}

impl Twist {
    pub fn obj(name: &str, children: Vec<Twist>) -> Self {
        return Self::ObjNode(name.to_string(), children);
    }

    pub fn arr(name: &str, children: Vec<Twist>) -> Self {
        return Self::ArrayNode(name.to_string(), children);
    }

    pub fn attr(name: &str, value: String) -> Self {
        return Self::AttrNode(name.to_string(), value);
    }

    pub fn val(name: &str, value: Option<Twist>) -> Self {
        return match value {
            Some(v) => Self::ValueNode(name.to_string(), Some(Box::new(v.twist()))),
            None => Self::ValueNode(name.to_string(), None),
        };
    }

    fn indent(s: &mut String, i: usize) {
        s.push_str(&"   ".repeat(i))
    }

    pub fn render(&self, rendered: &mut String, indent: usize) {
        match self {
            Self::ObjNode(name, children) => {
                Self::indent(rendered, indent);
                rendered.push_str("obj ");
                rendered.push_str(name);
                rendered.push_str(":\n");
                for c in children {
                    c.render(rendered, indent + 1)
                }
            }
            Self::ArrayNode(name, children) => {
                Self::indent(rendered, indent);
                rendered.push_str("arr ");
                rendered.push_str(name);
                rendered.push_str(":\n");
                for c in children {
                    c.render(rendered, indent + 1);
                }
            }
            Self::AttrNode(name, value) => {
                Self::indent(rendered, indent);
                rendered.push_str("attr ");
                rendered.push_str(name);
                rendered.push_str("=");
                rendered.push_str(value);
                rendered.push_str("\n");
            }
            Self::ValueNode(name, value) => match value {
                Some(v) => {
                    Self::indent(rendered, indent);
                    rendered.push_str("value ");
                    rendered.push_str(name);
                    rendered.push_str(":\n");
                    v.render(rendered, indent + 1)
                }
                None => (),
            },
        }
    }
}

impl Twistable for Twist {
    fn twist(&self) -> Twist {
        self.clone()
    }
}
