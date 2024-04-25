pub trait Twistable {
    fn twist(&self) -> Twist;
}

#[derive(Clone, Debug)]
pub enum Twist {
    ObjNode(String, Vec<Twist>),
    ArrayNode(String, Vec<Twist>),
    AttrNode(String, String),
    ValueNode(String, Option<Box<Twist>>),
    LeafNode(String),
}

impl Twist {
    pub fn obj(name: &str, children: Vec<Twist>) -> Self {
        return Self::ObjNode(name.to_string(), children);
    }

    pub fn arr(name: &str, children: Vec<Twist>) -> Self {
        return Self::ArrayNode(name.to_string(), children);
    }

    pub fn twist_arr(name: &str, children: &Vec<impl Twistable>) -> Self {
        Self::arr(
            name,
            children.iter().map(|t| t.twist()).collect::<Vec<Twist>>(),
        )
    }

    pub fn leaf(name: &str) -> Self {
        Self::LeafNode(name.to_string())
    }

    pub fn attr(name: &str, value: String) -> Self {
        return Self::AttrNode(name.to_string(), value);
    }

    pub fn opt_val(name: &str, value: Option<Twist>) -> Self {
        match value {
            Some(v) => Self::ValueNode(name.to_string(), Some(Box::new(v.twist()))),
            None => Self::ValueNode(name.to_string(), None),
        }
    }

    pub fn twist_opt_val(name: &str, value: Option<impl Twistable>) -> Self {
        Self::opt_val(name, value.map(|it| it.twist()))
    }

    pub fn val(name: &str, value: Twist) -> Self {
        Self::ValueNode(name.to_string(), Some(Box::new(value)))
    }

    pub fn twist_val(name: &str, value: &impl Twistable) -> Self {
        Self::val(name, value.twist())
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
                rendered.push_str("='");
                rendered.push_str(value);
                rendered.push_str("'\n");
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
            Self::LeafNode(name) => {
                Self::indent(rendered, indent);
                rendered.push_str(name);
                rendered.push_str("\n")
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        self.render(&mut s, 1);
        return s;
    }

    pub fn code(&self, rendered: &mut String) {
        match self {
            Self::ObjNode(name, children) => {
                rendered.push_str("Twist::obj(\"");
                rendered.push_str(name);
                rendered.push_str("\", vec![");
                rendered.push_str(
                    &children
                        .iter()
                        .map(|c| {
                            let mut cstr = String::new();
                            c.code(&mut cstr);
                            cstr
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                rendered.push_str("])");
            }
            Self::ArrayNode(name, children) => {
                rendered.push_str("Twist::arr(\"");
                rendered.push_str(name);
                rendered.push_str("\", vec![");
                rendered.push_str(
                    &children
                        .iter()
                        .map(|c| {
                            let mut cstr = String::new();
                            c.code(&mut cstr);
                            cstr
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                );

                rendered.push_str("])");
            }
            Self::AttrNode(name, value) => {
                rendered.push_str("Twist::attr(\"");
                rendered.push_str(name);
                rendered.push_str("\", \"");
                rendered.push_str(value);
                rendered.push_str("\".to_string())")
            }
            Self::ValueNode(name, value) => match value {
                Some(v) => {
                    rendered.push_str("Twist::val(\"");
                    rendered.push_str(name);
                    rendered.push_str("\", ");
                    v.code(rendered);
                    rendered.push_str(")");
                }
                None => (),
            },
            Self::LeafNode(name) => {
                rendered.push_str("Twist::leaf(\"");
                rendered.push_str(name);
                rendered.push_str("\");n")
            }
        }
    }
}

impl Twistable for Twist {
    fn twist(&self) -> Twist {
        self.clone()
    }
}
