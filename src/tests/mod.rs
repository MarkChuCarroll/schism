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

use crate::ast::*;
use crate::twist::{Twist, Twistable};
use crate::{lex, parser::DefinitionParser, parser::SectParser};

fn assert_token_is(result: Option<lex::ScannerResult>, expected: lex::Token) {
    assert!(result.is_some());
    let (_, t, _) = result.unwrap().unwrap();
    assert_eq!(expected, t)
}

#[test]
pub fn test_scan_symbols_and_idents() {
    let mut lex = lex::Scanner::new("foo bar/baz + 23\nbli");

    assert_token_is(
        lex.scan_token(),
        lex::Token::LName(LowerName("foo".to_string())),
    );
    assert_token_is(
        lex.scan_token(),
        lex::Token::LName(LowerName("bar/baz".to_string())),
    );
    assert_token_is(
        lex.scan_token(),
        lex::Token::LName(LowerName("+".to_string())),
    );
    assert_token_is(lex.scan_token(), lex::Token::IntLit(23));
    assert_token_is(
        lex.scan_token(),
        lex::Token::LName(LowerName("bli".to_string())),
    )
}

#[test]
pub fn test_scan_literals() {
    let mut lex = lex::Scanner::new("\"this is a string\" 27 13.2 -4.0e5 'a'\"");

    assert_token_is(
        lex.scan_token(),
        lex::Token::StringLit("this is a string".to_string()),
    );
    assert_token_is(lex.scan_token(), lex::Token::IntLit(27));
    assert_token_is(lex.scan_token(), lex::Token::FloatLit("13.2".to_string()));
    assert_token_is(lex.scan_token(), lex::Token::FloatLit("-4.0e5".to_string()));
    assert_token_is(lex.scan_token(), lex::Token::CharLit('a'));
}

#[test]
pub fn test_parse_fun() {
    let fun_str = "
    fun foo ( Int Str -- Float ) do
        dup * /
    end @fun
    ";
    let parsed = DefinitionParser::new().parse(lex::Scanner::new(fun_str));

    let def = parsed.expect("Should have succeeded");
    let t = def.twist();
    let rendered = t.to_string();

    let expected = mk_expected_fun().to_string();

    assert_eq!(expected, rendered)
}

#[test]
pub fn parse_object() {
    let obj_str = "
    use .thatsect.overthere.yeahthatone.{ a , b , c }

    obj Consish[`a] composes B , C ( String Int ) is
        slot s1 : Int ( Int ) init set end
        slot s2 : String ( String ) init set end

        meth print_n_times ( String Int -- ) do
        { ( String -- String ) dup print } doNtimes
        end @meth
        do
    end@obj
    ";
    let parsed = SectParser::new()
        .parse(lex::Scanner::new(obj_str))
        .unwrap()
        .twist();

    let parsed_str = parsed.to_string();
    let expected = mk_expected_obj().twist().to_string();
    assert_eq!(expected, parsed_str)
}

#[test]
pub fn parse_complicated_stack_effect_type() {
    let fun_str = "
    fun meta[`X] ( $A `X y:( $A Int -- $B `X ) --  $B ) do
        twiddle swap apply
    end
    ";
    let parsed = DefinitionParser::new().parse(lex::Scanner::new(fun_str));
    if parsed.is_err() {
        println!("{:?}", parsed);
    }

    let twist = parsed.unwrap().twist();
    let expected_str = mk_expected_fun_with_complicated_type().to_string();
    println!("Actual:\n{}\n-------------------", twist.to_string());
    println!("Expected:\n{}\n-------------------", expected_str);
    assert_eq!(expected_str, twist.to_string())
}

#[test]
pub fn test_parse_lots_of_stuff() {
    let stuff = "
    use .lib.blob.{ that , +, ^squid^ }
    use .squirt.squat.squit.{ foo }

    obj Squortle[`A, `B] ( That ) is
        slot foo: List[`A] ( `A `A `A ) init
             eat eat eat
        end

        meth m ( Int -- Str) do
           + - /
           cond
             {( -- Int) something } do   aoeuaoeu /* test a comment */ end
           else
                 \"abc\" print
           end@cond
        end@meth
        do
    end@obj

    var q: Squortle[Int, Str] ( Str ) init
       31 ua set!
    end

    fun meta ( $A Int ($A Int -- $B) --  $B) do
        twiddle swap apply
    end
    ";

    let parsed = SectParser::new().parse(lex::Scanner::new(stuff));
    let twisted = parsed.unwrap().twist();
    assert_eq!(mk_expected_lots_of_stuff().to_string(), twisted.to_string());
}

fn mk_expected_fun() -> Twist {
    Twist::obj(
        "Function",
        vec![
            Twist::attr("name", "foo".to_string()),
            Twist::arr("type_params", vec![]),
            Twist::val(
                "effect",
                Twist::obj(
                    "StackEffect",
                    vec![
                        Twist::arr("effect_domains", vec![]),
                        Twist::val(
                            "before",
                            Twist::obj(
                                "StackImage",
                                vec![Twist::arr(
                                    "Stack",
                                    vec![
                                        Twist::obj(
                                            "SimpleType",
                                            vec![Twist::attr("id", "Int".to_string())],
                                        ),
                                        Twist::obj(
                                            "SimpleType",
                                            vec![Twist::attr("id", "Str".to_string())],
                                        ),
                                    ],
                                )],
                            ),
                        ),
                        Twist::val(
                            "after",
                            Twist::obj(
                                "StackImage",
                                vec![Twist::arr(
                                    "Stack",
                                    vec![Twist::obj(
                                        "SimpleType",
                                        vec![Twist::attr("id", "Float".to_string())],
                                    )],
                                )],
                            ),
                        ),
                    ],
                ),
            ),
            Twist::arr(
                "body",
                vec![
                    Twist::attr("InvokeName", "dup".to_string()),
                    Twist::attr("InvokeName", "*".to_string()),
                    Twist::attr("InvokeName", "/".to_string()),
                ],
            ),
        ],
    )
}

fn mk_expected_obj() -> Twist {
    Twist::obj("sect",
      vec![
         Twist::arr("uses",
            vec![
               Twist::obj("UseDecl",
                  vec![
                     Twist::arr("functions",
                        vec![
                           Twist::leaf("a"),
                           Twist::leaf("b"),
                           Twist::leaf("c")
                        ])
                  ])
            ]),
         Twist::arr("defs",
            vec![
               Twist::obj("ObjectDef",
                  vec![
                     Twist::attr("name", "Consish".to_string()),
                     Twist::arr("type_params",
                        vec![
                           Twist::obj("TypeParam",
                              vec![
                                 Twist::attr("name", "`a".to_string())
                              ])
                        ]),
                     Twist::arr("composes",
                        vec![
                           Twist::obj("SimpleType",
                              vec![
                                 Twist::attr("id", "B".to_string())
                              ]),
                           Twist::obj("SimpleType",
                              vec![
                                 Twist::attr("id", "C".to_string())
                              ])
                        ]),
                     Twist::val("inputs",
                        Twist::obj("StackImage",
                           vec![
                              Twist::arr("Stack",
                                 vec![
                                    Twist::obj("SimpleType",
                                       vec![
                                          Twist::attr("id", "String".to_string())
                                       ]),
                                    Twist::obj("SimpleType",
                                       vec![
                                          Twist::attr("id", "Int".to_string())
                                       ])
                                 ])
                           ])
                     ),
                     Twist::arr("members",
                        vec![
                           Twist::obj("Slot",
                              vec![
                                 Twist::attr("name", "s1".to_string()),
                                 Twist::val("value_type",
                                    Twist::obj("SimpleType",
                                       vec![
                                          Twist::attr("id", "Int".to_string())
                                       ])
                                 ),
                                 Twist::val("inputs",
                                    Twist::obj("StackImage",
                                       vec![
                                          Twist::arr("Stack",
                                             vec![
                                                Twist::obj("SimpleType",
                                                   vec![
                                                      Twist::attr("id", "Int".to_string())
                                                   ])
                                             ])
                                       ])
                                 ),
                                 Twist::arr("body",
                                    vec![
                                       Twist::attr("InvokeName", "set".to_string())
                                    ])
                              ]),
                           Twist::obj("Slot",
                              vec![
                                 Twist::attr("name", "s2".to_string()),
                                 Twist::val("value_type",
                                    Twist::obj("SimpleType",
                                       vec![
                                          Twist::attr("id", "String".to_string())
                                       ])
                                 ),
                                 Twist::val("inputs",
                                    Twist::obj("StackImage",
                                       vec![
                                          Twist::arr("Stack",
                                             vec![
                                                Twist::obj("SimpleType",
                                                   vec![
                                                      Twist::attr("id", "String".to_string())
                                                   ])
                                             ])
                                       ])
                                 ),
                                 Twist::arr("body",
                                    vec![
                                       Twist::attr("InvokeName", "set".to_string())
                                    ])
                              ]),
                           Twist::obj("Method",
                              vec![
                                 Twist::attr("name", "print_n_times".to_string()),
                                 Twist::val("effect",
                                    Twist::obj("StackEffect",
                                       vec![
                                          Twist::val("before",
                                             Twist::obj("StackImage",
                                                vec![
                                                   Twist::arr("Stack",
                                                      vec![
                                                         Twist::obj("SimpleType",
                                                            vec![
                                                               Twist::attr("id", "String".to_string())
                                                            ]),
                                                         Twist::obj("SimpleType",
                                                            vec![
                                                               Twist::attr("id", "Int".to_string())
                                                            ])
                                                      ])
                                                ])
                                          ),
                                          Twist::val("after",
                                             Twist::obj("StackImage",
                                                vec![

                                                ])
                                          )
                                       ])
                                 ),
                                 Twist::arr("body",
                                    vec![
                                       Twist::obj("Block",
                                          vec![
                                             Twist::val("effect",
                                                Twist::obj("StackEffect",
                                                   vec![
                                                      Twist::val("before",
                                                         Twist::obj("StackImage",
                                                            vec![
                                                               Twist::arr("Stack",
                                                                  vec![
                                                                     Twist::obj("SimpleType",
                                                                        vec![
                                                                           Twist::attr("id", "String".to_string())
                                                                        ])
                                                                  ])
                                                            ])
                                                      ),
                                                      Twist::val("after",
                                                         Twist::obj("StackImage",
                                                            vec![
                                                               Twist::arr("Stack",
                                                                  vec![
                                                                     Twist::obj("SimpleType",
                                                                        vec![
                                                                           Twist::attr("id", "String".to_string())
                                                                        ])
                                                                  ])
                                                            ])
                                                      )
                                                   ])
                                             ),
                                             Twist::arr("body",
                                                vec![
                                                   Twist::attr("InvokeName", "dup".to_string()),
                                                   Twist::attr("InvokeName", "print".to_string())
                                                ])
                                          ]),
                                       Twist::attr("InvokeName", "doNtimes".to_string())
                                    ])
                              ])
                        ])
                  ])
            ])
      ])
}
fn mk_expected_fun_with_complicated_type() -> Twist {
    Twist::obj(
        "Function",
        vec![
            Twist::attr("name", "meta".to_string()),
            Twist::arr(
                "type_params",
                vec![Twist::obj(
                    "TypeParam",
                    vec![Twist::attr("name", "`X".to_string())],
                )],
            ),
            Twist::val(
                "effect",
                Twist::obj(
                    "StackEffect",
                    vec![
                        Twist::val(
                            "before",
                            Twist::obj(
                                "StackImage",
                                vec![
                                    Twist::val("context", Twist::leaf("$A")),
                                    Twist::arr(
                                        "Stack",
                                        vec![
                                            Twist::obj(
                                                "TypeVar",
                                                vec![Twist::attr("id", "`X".to_string())],
                                            ),
                                            Twist::val(
                                                "y",
                                                Twist::val(
                                                    "FunctionType",
                                                    Twist::obj(
                                                        "StackEffect",
                                                        vec![
                                                            Twist::val(
                                                                "before",
                                                                Twist::obj(
                                                                    "StackImage",
                                                                    vec![
                                                                        Twist::val(
                                                                            "context",
                                                                            Twist::leaf("$A"),
                                                                        ),
                                                                        Twist::arr(
                                                                            "Stack",
                                                                            vec![Twist::obj(
                                                                                "SimpleType",
                                                                                vec![Twist::attr(
                                                                                    "id",
                                                                                    "Int"
                                                                                        .to_string(
                                                                                        ),
                                                                                )],
                                                                            )],
                                                                        ),
                                                                    ],
                                                                ),
                                                            ),
                                                            Twist::val(
                                                                "after",
                                                                Twist::obj(
                                                                    "StackImage",
                                                                    vec![
                                                                        Twist::val(
                                                                            "context",
                                                                            Twist::leaf("$B"),
                                                                        ),
                                                                        Twist::arr(
                                                                            "Stack",
                                                                            vec![Twist::obj(
                                                                                "TypeVar",
                                                                                vec![Twist::attr(
                                                                                    "id",
                                                                                    "`X".to_string(
                                                                                    ),
                                                                                )],
                                                                            )],
                                                                        ),
                                                                    ],
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                ],
                            ),
                        ),
                        Twist::val(
                            "after",
                            Twist::obj(
                                "StackImage",
                                vec![Twist::val("context", Twist::leaf("$B"))],
                            ),
                        ),
                    ],
                ),
            ),
            Twist::arr(
                "body",
                vec![
                    Twist::attr("InvokeName", "twiddle".to_string()),
                    Twist::attr("InvokeName", "swap".to_string()),
                    Twist::attr("InvokeName", "apply".to_string()),
                ],
            ),
        ],
    )
}

fn mk_expected_lots_of_stuff() -> Twist {
    Twist::obj("sect",
               vec![
                   Twist::arr("uses",
                              vec![
                                  Twist::obj("UseDecl",
                                             vec![
                                                 Twist::arr("functions",
                                                            vec![
                                                                Twist::leaf("that"),
                                                                Twist::leaf("+"),
                                                                Twist::leaf("^squid^")
                                                            ])
                                             ]),
                                  Twist::obj("UseDecl",
                                             vec![
                                                 Twist::arr("functions",
                                                            vec![
                                                                Twist::leaf("foo")
                                                            ])
                                             ])
                              ]),
                   Twist::arr("defs",
                              vec![
                                  Twist::obj("ObjectDef",
                                             vec![
                                                 Twist::attr("name", "Squortle".to_string()),
                                                 Twist::arr("type_params",
                                                            vec![
                                                                Twist::obj("TypeParam",
                                                                           vec![
                                                                               Twist::attr("name", "`A".to_string())
                                                                           ]),
                                                                Twist::obj("TypeParam",
                                                                           vec![
                                                                               Twist::attr("name", "`B".to_string())
                                                                           ])
                                                            ]),
                                                 Twist::val("inputs",
                                                            Twist::obj("StackImage",
                                                                       vec![
                                                                           Twist::arr("Stack",
                                                                                      vec![
                                                                                          Twist::obj("SimpleType",
                                                                                                     vec![
                                                                                                         Twist::attr("id", "That".to_string())
                                                                                                     ])
                                                                                      ])
                                                                       ])
                                                 ),
                                                 Twist::arr("members",
                                                            vec![
                                                                Twist::obj("Slot",
                                                                           vec![
                                                                               Twist::attr("name", "foo".to_string()),
                                                                               Twist::val("value_type",
                                                                                          Twist::obj("ParametricType",
                                                                                                     vec![
                                                                                                         Twist::attr("base_type", "List".to_string()),
                                                                                                         Twist::arr("parameters",
                                                                                                                    vec![
                                                                                                                        Twist::obj("TypeVar",
                                                                                                                                   vec![
                                                                                                                                       Twist::attr("id", "`A".to_string())
                                                                                                                                   ])
                                                                                                                    ])
                                                                                                     ])
                                                                               ),
                                                                               Twist::val("inputs",
                                                                                          Twist::obj("StackImage",
                                                                                                     vec![
                                                                                                         Twist::arr("Stack",
                                                                                                                    vec![
                                                                                                                        Twist::obj("TypeVar",
                                                                                                                                   vec![
                                                                                                                                       Twist::attr("id", "`A".to_string())
                                                                                                                                   ]),
                                                                                                                        Twist::obj("TypeVar",
                                                                                                                                   vec![
                                                                                                                                       Twist::attr("id", "`A".to_string())
                                                                                                                                   ]),
                                                                                                                        Twist::obj("TypeVar",
                                                                                                                                   vec![
                                                                                                                                       Twist::attr("id", "`A".to_string())
                                                                                                                                   ])
                                                                                                                    ])
                                                                                                     ])
                                                                               ),
                                                                               Twist::arr("body",
                                                                                          vec![
                                                                                              Twist::attr("InvokeName", "eat".to_string()),
                                                                                              Twist::attr("InvokeName", "eat".to_string()),
                                                                                              Twist::attr("InvokeName", "eat".to_string())
                                                                                          ])
                                                                           ]),
                                                                Twist::obj("Method",
                                                                           vec![
                                                                               Twist::attr("name", "m".to_string()),
                                                                               Twist::val("effect",
                                                                                          Twist::obj("StackEffect",
                                                                                                     vec![
                                                                                                         Twist::val("before",
                                                                                                                    Twist::obj("StackImage",
                                                                                                                               vec![
                                                                                                                                   Twist::arr("Stack",
                                                                                                                                              vec![
                                                                                                                                                  Twist::obj("SimpleType",
                                                                                                                                                             vec![
                                                                                                                                                                 Twist::attr("id", "Int".to_string())
                                                                                                                                                             ])
                                                                                                                                              ])
                                                                                                                               ])
                                                                                                         ),
                                                                                                         Twist::val("after",
                                                                                                                    Twist::obj("StackImage",
                                                                                                                               vec![
                                                                                                                                   Twist::arr("Stack",
                                                                                                                                              vec![
                                                                                                                                                  Twist::obj("SimpleType",
                                                                                                                                                             vec![
                                                                                                                                                                 Twist::attr("id", "Str".to_string())
                                                                                                                                                             ])
                                                                                                                                              ])
                                                                                                                               ])
                                                                                                         )
                                                                                                     ])
                                                                               ),
                                                                               Twist::arr("body",
                                                                                          vec![
                                                                                              Twist::attr("InvokeName", "+".to_string()),
                                                                                              Twist::attr("InvokeName", "-".to_string()),
                                                                                              Twist::attr("InvokeName", "/".to_string()),
                                                                                              Twist::obj("Cond",
                                                                                                         vec![
                                                                                                             Twist::arr("cases",
                                                                                                                        vec![
                                                                                                                            Twist::obj("CondClause",
                                                                                                                                       vec![
                                                                                                                                           Twist::val("condition",
                                                                                                                                                      Twist::obj("Block",
                                                                                                                                                                 vec![
                                                                                                                                                                     Twist::val("effect",
                                                                                                                                                                                Twist::obj("StackEffect",
                                                                                                                                                                                           vec![
                                                                                                                                                                                               Twist::val("before",
                                                                                                                                                                                                          Twist::obj("StackImage",
                                                                                                                                                                                                                     vec![])
                                                                                                                                                                                               ),
                                                                                                                                                                                               Twist::val("after",
                                                                                                                                                                                                          Twist::obj("StackImage",
                                                                                                                                                                                                                     vec![
                                                                                                                                                                                                                         Twist::arr("Stack",
                                                                                                                                                                                                                                    vec![
                                                                                                                                                                                                                                        Twist::obj("SimpleType",
                                                                                                                                                                                                                                                   vec![
                                                                                                                                                                                                                                                       Twist::attr("id", "Int".to_string())
                                                                                                                                                                                                                                                   ])
                                                                                                                                                                                                                                    ])
                                                                                                                                                                                                                     ])
                                                                                                                                                                                               )
                                                                                                                                                                                           ])
                                                                                                                                                                     ),
                                                                                                                                                                     Twist::arr("body",
                                                                                                                                                                                vec![
                                                                                                                                                                                    Twist::attr("InvokeName", "something".to_string())
                                                                                                                                                                                ])
                                                                                                                                                                 ])
                                                                                                                                           ),
                                                                                                                                           Twist::arr("body",
                                                                                                                                                      vec![
                                                                                                                                                          Twist::attr("InvokeName", "aoeuaoeu".to_string())
                                                                                                                                                      ])
                                                                                                                                       ])
                                                                                                                        ]),
                                                                                                             Twist::arr("else",
                                                                                                                        vec![
                                                                                                                            Twist::attr("StringLit", "abc".to_string()),
                                                                                                                            Twist::attr("InvokeName", "print".to_string())
                                                                                                                        ])
                                                                                                         ])
                                                                                          ])
                                                                           ])
                                                            ])
                                             ]),
                                  Twist::obj("Variable",
                                             vec![
                                                 Twist::attr("name", "q".to_string()),
                                                 Twist::val("value_type",
                                                            Twist::obj("ParametricType",
                                                                       vec![
                                                                           Twist::attr("base_type", "Squortle".to_string()),
                                                                           Twist::arr("parameters",
                                                                                      vec![
                                                                                          Twist::obj("SimpleType",
                                                                                                     vec![
                                                                                                         Twist::attr("id", "Int".to_string())
                                                                                                     ]),
                                                                                          Twist::obj("SimpleType",
                                                                                                     vec![
                                                                                                         Twist::attr("id", "Str".to_string())
                                                                                                     ])
                                                                                      ])
                                                                       ])
                                                 ),
                                                 Twist::val("inputs",
                                                            Twist::obj("StackImage",
                                                                       vec![
                                                                           Twist::arr("Stack",
                                                                                      vec![
                                                                                          Twist::obj("SimpleType",
                                                                                                     vec![
                                                                                                         Twist::attr("id", "Str".to_string())
                                                                                                     ])
                                                                                      ])
                                                                       ])
                                                 ),
                                                 Twist::arr("body",
                                                            vec![
                                                                Twist::attr("IntLit", "31".to_string()),
                                                                Twist::attr("InvokeName", "ua".to_string()),
                                                                Twist::attr("InvokeName", "set!".to_string())
                                                            ])
                                             ]),
                                  Twist::obj("Function",
                                             vec![
                                                 Twist::attr("name", "meta".to_string()),
                                                 Twist::val("effect",
                                                            Twist::obj("StackEffect",
                                                                       vec![
                                                                           Twist::val("before",
                                                                                      Twist::obj("StackImage",
                                                                                                 vec![
                                                                                                     Twist::val("context",
                                                                                                                Twist::leaf("$A")
                                                                                                     ),
                                                                                                     Twist::arr("Stack",
                                                                                                                vec![
                                                                                                                    Twist::obj("SimpleType",
                                                                                                                               vec![
                                                                                                                                   Twist::attr("id", "Int".to_string())
                                                                                                                               ]),
                                                                                                                    Twist::val("FunctionType",
                                                                                                                               Twist::obj("StackEffect",
                                                                                                                                          vec![
                                                                                                                                              Twist::val("before",
                                                                                                                                                         Twist::obj("StackImage",
                                                                                                                                                                    vec![
                                                                                                                                                                        Twist::val("context",
                                                                                                                                                                                   Twist::leaf("$A")
                                                                                                                                                                        ),
                                                                                                                                                                        Twist::arr("Stack",
                                                                                                                                                                                   vec![
                                                                                                                                                                                       Twist::obj("SimpleType",
                                                                                                                                                                                                  vec![
                                                                                                                                                                                                      Twist::attr("id", "Int".to_string())
                                                                                                                                                                                                  ])
                                                                                                                                                                                   ])
                                                                                                                                                                    ])
                                                                                                                                              ),
                                                                                                                                              Twist::val("after",
                                                                                                                                                         Twist::obj("StackImage",
                                                                                                                                                                    vec![
                                                                                                                                                                        Twist::val("context",
                                                                                                                                                                                   Twist::leaf("$B")
                                                                                                                                                                        )
                                                                                                                                                                    ])
                                                                                                                                              )
                                                                                                                                          ])
                                                                                                                    )
                                                                                                                ])
                                                                                                 ])
                                                                           ),
                                                                           Twist::val("after",
                                                                                      Twist::obj("StackImage",
                                                                                                 vec![
                                                                                                     Twist::val("context",
                                                                                                                Twist::leaf("$B")
                                                                                                     )
                                                                                                 ])
                                                                           )
                                                                       ])
                                                 ),
                                                 Twist::arr("body",
                                                            vec![
                                                                Twist::attr("InvokeName", "twiddle".to_string()),
                                                                Twist::attr("InvokeName", "swap".to_string()),
                                                                Twist::attr("InvokeName", "apply".to_string())
                                                            ])
                                             ])
                              ])
               ])
}
