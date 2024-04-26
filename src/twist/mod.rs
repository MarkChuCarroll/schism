// Copyright 2024 Mark C. Chu-Carroll
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        Self::ObjNode(name.to_string(), children)
    }

    pub fn arr(name: &str, children: Vec<Twist>) -> Self {
        Self::ArrayNode(name.to_string(), children)
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
        Self::AttrNode(name.to_string(), value)
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
                    let mut cr = String::new();
                    c.render(&mut cr, indent + 1);
                    if cr.chars().any(|ch| !ch.is_whitespace()) {
                        rendered.push_str(&cr)
                    }
                }
            }
            Self::ArrayNode(name, children) => {
                if children.len() > 0 {
                    Self::indent(rendered, indent);
                    rendered.push_str("arr ");
                    rendered.push_str(name);
                    rendered.push_str(":\n");
                    for c in children {
                        c.render(rendered, indent + 1);
                    }
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
        s
    }

    pub fn to_code(&self) -> String {
        let mut s = String::new();
        self.code(&mut s, 1);
        s
    }

    pub fn code(&self, rendered: &mut String, ind: usize) {
        match self {
            Self::ObjNode(name, children) => {
                Self::indent(rendered, ind);
                rendered.push_str("Twist::obj(\"");
                rendered.push_str(name);
                rendered.push_str("\",\n");
                Self::indent(rendered, ind + 1);
                rendered.push_str("vec![\n");
                rendered.push_str(
                    &children
                        .iter()
                        .map(|c| {
                            let mut cstr = String::new();
                            c.code(&mut cstr, ind + 2);
                            cstr
                        })
                        .filter(|c| c.chars().any(|c| !c.is_whitespace()))
                        .collect::<Vec<String>>()
                        .join(",\n"),
                );
                rendered.push_str("\n");
                Self::indent(rendered, ind + 1);
                rendered.push_str("])");
            }
            Self::ArrayNode(name, children) => {
                if children.len() > 0 {
                    Self::indent(rendered, ind);
                    rendered.push_str("Twist::arr(\"");
                    rendered.push_str(name);
                    rendered.push_str("\",\n");
                    Self::indent(rendered, ind + 1);
                    rendered.push_str("vec![\n");
                    rendered.push_str(
                        &children
                            .iter()
                            .map(|c| {
                                let mut cstr = String::new();
                                c.code(&mut cstr, ind + 2);
                                cstr
                            })
                            .filter(|l| l.len() > 0)
                            .collect::<Vec<String>>()
                            .join(",\n"),
                    );
                    rendered.push_str("\n");
                    Self::indent(rendered, ind + 1);
                    rendered.push_str("])");
                }
            }
            Self::AttrNode(name, value) => {
                Self::indent(rendered, ind);
                rendered.push_str("Twist::attr(\"");
                rendered.push_str(name);
                rendered.push_str("\", \"");
                rendered.push_str(value);
                rendered.push_str("\".to_string())")
            }
            Self::ValueNode(name, value) => match value {
                Some(v) => {
                    Self::indent(rendered, ind);
                    rendered.push_str("Twist::val(\"");
                    rendered.push_str(name);
                    rendered.push_str("\",\n");
                    v.code(rendered, ind + 1);
                    rendered.push_str("\n");
                    Self::indent(rendered, ind);
                    rendered.push_str(")");
                }
                None => (),
            },
            Self::LeafNode(name) => {
                Self::indent(rendered, ind);
                rendered.push_str("Twist::leaf(\"");
                rendered.push_str(name);
                rendered.push_str("\")")
            }
        }
    }
}

impl Twistable for Twist {
    fn twist(&self) -> Twist {
        self.clone()
    }
}
