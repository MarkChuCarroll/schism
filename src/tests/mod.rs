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

fn assert_token_is<'input>(result: Option<lex::ScannerResult<'input>>, expected: lex::Token) {
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
    assert_token_is(lex.scan_token(), lex::Token::INTLIT(23));
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
        lex::Token::STRINGLIT("this is a string".to_string()),
    );
    assert_token_is(lex.scan_token(), lex::Token::INTLIT(27));
    assert_token_is(lex.scan_token(), lex::Token::FLOATLIT("13.2".to_string()));
    assert_token_is(lex.scan_token(), lex::Token::FLOATLIT("-4.0e5".to_string()));
    assert_token_is(lex.scan_token(), lex::Token::CHARLIT('a'));
}

#[test]
pub fn test_parse_fun() {
    let funstr = "
    fun foo ( Int Str -- Float ) do
        dup * /
    end @fun
    ";
    let parsed = DefinitionParser::new().parse(lex::Scanner::new(funstr));

    let def = parsed.expect("Should have succeeded");
    let t = def.twist();
    let rendered = t.to_string();

    let expected = mk_expected_fun().to_string();

    assert_eq!(expected, rendered)
}

#[test]
pub fn parse_object() {
    let structstr = "
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
        .parse(lex::Scanner::new(structstr))
        .unwrap()
        .twist();

    let parsed_str = parsed.to_string();
    let expected = make_expected_obj().twist().to_string();
    assert_eq!(expected, parsed_str)
}

/*
#[test]
pub fn test_parse_harder_fun() {
    ast::StackImage::reset_index();
    let funstr = "
    fun meta ( @A int (@A int -- @B) --  @B) is
        [int]twiddle swap apply
    end
    ";

    // This is annoyingly laborious - but how else to praperly test a
    // parser than to ensure that it generates the right AST?
    let se = ast::StackEffect {
        before: ast::StackImage {
            stack_var: ast::Symbol("@A".to_string()),
            stack: vec![
                ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("int".to_string()))),
                ast::SType::Function(ast::StackEffect {
                    before: ast::StackImage {
                        stack_var: ast::Symbol("@A".to_string()),
                        stack: vec![ast::SType::Simple(ast::Identifier::Simple(ast::Symbol(
                            "int".to_string(),
                        )))],
                    },
                    after: ast::StackImage {
                        stack_var: ast::Symbol("@B".to_string()),
                        stack: vec![],
                    },
                }),
            ],
        },
        after: ast::StackImage {
            stack_var: ast::Symbol("@B".to_string()),
            stack: vec![],
        },
    };

    let body: Vec<ast::Expr> = vec![
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("twiddle".to_string())),
            type_args: Some(vec![ast::SType::Simple(ast::Identifier::Simple(
                ast::Symbol("int".to_string()),
            ))]),
        }),
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("swap".to_string())),
            type_args: None,
        }),
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("apply".to_string())),
            type_args: None,
        }),
    ];
    let expected_fun = ast::FunctionDecl {
        name: ast::Symbol("meta".to_string()),
        type_params: None,
        signature: se,
        body,
    };

    let parsed = schism_parser::FunctionDeclParser::new()
        .parse(lex::Scanner::new("foo".to_string(), funstr));

    let parsed_str = parsed.unwrap().to_string();
    let expected_str = expected_fun.to_string();
    println!("parsed='''\n{}\n'''", parsed_str);
    println!("expected='''\n{}\n'''", expected_str);

    assert_eq!(expected_str, parsed_str);
}

#[test]
pub fn test_parse_lots_of_stuff() {
    ast::StackImage::reset_index();
    let funstr = "
    use lib::blob{that, +, ^squid^}
    use squirt::squat::squit

    struct [`a, `b] Squortle ( that ) is
        slot foo: [int, `a]List

        meth m ( int -- str) is
           + - /
           if
               aoeuaoeu /* test a comment */
           else
               [[ ( -- )  \"abc\" print]]
           end
        end
    end

    var q: [int, str]Squortle init
       31 ua set!
    end

    fun meta ( @A int (@A int -- @B) --  @B) is
        [int]twiddle swap apply
    end
    ";

    let expected = "   sect
      use lib::blob{that, +, ^squid^}
      use squirt::squat::squit
      struct [`a, `b]Squortle
         supers that
         slot foo: [int, `a]List
         meth m (@_0 int -- @_0 str) do
            +
            -
            /
            if
               aoeuaoeu
            else
               [[
                  (@_1  -- @_1 )
                  \"abc\"
                  print
               ]]
            end
         end
      end
      var q: [int, str]Squortle{
         31
         ua
         set!
      }
      fun meta(@A int (@A int -- @B ) -- @B ) is
         [int]twiddle
         swap
         apply
      end
   end
";

    let parsed =
        schism_parser::SectParser::new().parse(lex::Scanner::new("foo".to_string(), funstr));

    let parsed_str = parsed.unwrap().to_string();
    assert_eq!(expected, parsed_str);
}
*/

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

fn make_expected_obj() -> Twist {
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
