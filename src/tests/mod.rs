use crate::twist::{Twist, Twistable};
use crate::{ast::LowerName, lex, parser::DefinitionParser};

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
    end
    ";
    let parsed = DefinitionParser::new().parse(lex::Scanner::new(funstr));
    assert!(parsed.is_ok());
    let def = parsed.expect("Should have succeeded");
    let t = def.twist();
    let mut rendered = String::new();
    t.render(&mut rendered, 1);

    let mut code = String::new();
    let expected = mk_expected_fun();
    let mut expected_str = String::new();
    expected.render(&mut expected_str, 1);

    assert_eq!(expected_str, rendered)
}

/*
    // This is annoyingly laborious - but how else to praperly test a
    // parser than to ensure that it generates the right AST?
    let se = ast::StackEffect {
        before: ast::StackImage {
            stack_var: ast::Symbol("@_0".to_string()),
            stack: vec![
                ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("int".to_string()))),
                ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("str".to_string()))),
            ],
        },
        after: ast::StackImage {
            stack_var: ast::Symbol("@_0".to_string()),
            stack: vec![ast::SType::Simple(ast::Identifier::Simple(ast::Symbol(
                "float".to_string(),
            )))],
        },
    };
    let body: Vec<ast::Expr> = vec![
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("dup".to_string())),
            type_args: None,
        }),
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("*".to_string())),
            type_args: None,
        }),
        ast::Expr::FunCall(ast::FunCallExpr {
            id: ast::Identifier::Simple(ast::Symbol("/".to_string())),
            type_args: None,
        }),
    ];
    let expected_fun = ast::FunctionDecl {
        name: ast::Symbol("foo".to_string()),
        type_params: None,
        signature: se,
        body,
    };

    let parsed = schism_parser::FunctionDeclParser::new()
        .parse(lex::Scanner::new("foo".to_string(), funstr));

    let parsed_str = parsed.unwrap().to_string();
    let expected_str = expected_fun.to_string();
    assert_eq!(expected_str, parsed_str);
}

#[test]
pub fn parse_struct() {
    ast::StackImage::reset_index();
    let structstr = "
    use thatsect::overthere::yeahthatone { a, b, c }

    struct [`a] Consish is
        slot s1: Int
        slot s2: str
        meth print_n_times ( str int -- ) is
            [[(@B str -- @B str) dup print ]] f::doNtimes
        end
        meth initialize ( @A int str -- @A ) is
            set-s2!
            set-s1!
        end
    end
    ";


    let block = ast::BlockExpr {
        effect: ast::StackEffect {
            before: ast::StackImage {
                stack_var: ast::Symbol("@B".to_string()),
                stack: vec![ast::SType::Simple(ast::Identifier::Simple(ast::Symbol(
                    "str".to_string(),
                )))],
            },
            after: ast::StackImage {
                stack_var: ast::Symbol("@B".to_string()),
                stack: vec![ast::SType::Simple(ast::Identifier::Simple(ast::Symbol(
                    "str".to_string(),
                )))],
            },
        },
        body: vec![
            ast::Expr::FunCall(ast::FunCallExpr {
                id: ast::Identifier::Simple(ast::Symbol("dup".to_string())),
                type_args: None,
            }),
            ast::Expr::FunCall(ast::FunCallExpr {
                id: ast::Identifier::Simple(ast::Symbol("print".to_string())),
                type_args: None,
            }),
        ],
    };

    let use_decl = ast::UseDecl {
        sect: ast::Identifier::Qualified(vec![
            ast::Symbol("thatsect".to_string()),
            ast::Symbol("overthere".to_string()),
            ast::Symbol("yeahthatone".to_string()),
        ]),
        names: Some(vec![
            ast::Symbol("a".to_string()),
            ast::Symbol("b".to_string()),
            ast::Symbol("c".to_string()),
        ]),
    };

    let m_print = ast::MethodDecl {
        name: ast::Symbol("print_n_times".to_string()),
        effect: ast::StackEffect {
            before: ast::StackImage {
                stack_var: ast::Symbol("@_0".to_string()),
                stack: vec![
                    ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("str".to_string()))),
                    ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("int".to_string()))),
                ],
            },
            after: ast::StackImage {
                stack_var: ast::Symbol("@_0".to_string()),
                stack: vec![],
            },
        },
        body: vec![
            ast::Expr::Block(block),
            ast::Expr::FunCall(ast::FunCallExpr {
                id: ast::Identifier::Qualified(vec![
                    ast::Symbol("f".to_string()),
                    ast::Symbol("doNtimes".to_string()),
                ]),
                type_args: None,
            }),
        ],
    };

    let init_meth = ast::MethodDecl {
        name: ast::Symbol("initialize".to_string()),
        effect: ast::StackEffect {
            before: ast::StackImage {
                stack_var: ast::Symbol("@A".to_string()),
                stack: vec![
                    ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("int".to_string()))),
                    ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("str".to_string()))),
                ],
            },
            after: ast::StackImage {
                stack_var: ast::Symbol("@A".to_string()),
                stack: vec![],
            },
        },
        body: vec![
            ast::Expr::FunCall(ast::FunCallExpr {
                id: ast::Identifier::Simple(ast::Symbol("set-s2!".to_string())),
                type_args: None,
            }),
            ast::Expr::FunCall(ast::FunCallExpr {
                id: ast::Identifier::Simple(ast::Symbol("set-s1!".to_string())),
                type_args: None,
            }),
        ],
    };

    let slots = vec![
        ast::TypedIdentifier {
            name: ast::Symbol("s1".to_string()),
            s_type: ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("Int".to_string()))),
        },
        ast::TypedIdentifier {
            name: ast::Symbol("s2".to_string()),
            s_type: ast::SType::Simple(ast::Identifier::Simple(ast::Symbol("str".to_string()))),
        },
    ];

    let consish = ast::StructDecl {
        name: ast::Symbol("Consish".to_string()),
        supers: None,
        type_params: Some(vec![ast::TypeParam {
            name: ast::Symbol("`a".to_string()),
            constraint: None,
        }]),
        fields: slots,
        methods: vec![m_print, init_meth],
    };

    let sect = ast::Sect {
        uses: vec![use_decl],
        decls: vec![ast::Decl::Struct(consish)],
    };

    let expected_str = sect.to_string();

    let parsed: Result<
        crate::ast::Sect,
        lalrpop_util::ParseError<usize, lex::Token, crate::error::Error>,
    > = schism_parser::SectParser::new().parse(lex::Scanner::new("foo".to_string(), structstr));

    let parsed_str = parsed.unwrap().to_string();

    assert_eq!(expected_str, parsed_str)
}

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
