use std::env;
use std::path::Path;

use syntex_syntax as syntax;

use syntax::ast;
use syntax::ast::{Arg, Expr};
use syntax::ast::{ExprKind, ItemKind, LitKind, PatKind, StmtKind};
use syntax::codemap::FilePathMapping;
use syntax::parse::ParseSess;
use syntax::ptr::P;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rua <file.rua>");
        return;
    }
    let path = Path::new(&args[1]);

    let sess = ParseSess::new(FilePathMapping::empty());
    let krate = match syntax::parse::parse_crate_from_file(path, &sess) {
        Ok(_) if sess.span_diagnostic.has_errors() => panic!("errors but recovered"),
        Ok(krate) => krate,
        Err(_e) => panic!("errors while parsing"),
    };

    let mut generator = Generator::new();
    generator.module(&krate.module);
    eprintln!("-------------------- GENERATED ------------------------");
    println!("{}", generator.buf);
}

struct Generator {
    buf: String,
}

impl Generator {
    fn new() -> Self {
        Self { buf: String::new() }
    }

    fn push_str(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    fn module(&mut self, module: &ast::Mod) {
        for item in &module.items {
            self.item(&item);
        }
    }

    fn literal(&mut self, lit: &ast::Lit) {
        match lit.node {
            LitKind::Str(s, _) => self.push_str(&format!("'{}'", s)),
            LitKind::Int(n, _) => self.push_str(&format!("{}", n)),
            _ => panic!("unsupported literal kind: {:?}", lit.node),
        }
    }

    fn ident(&mut self, ident: &ast::Ident) {
        self.push_str(&format!("{}", ident.name));
    }

    fn tuple(&mut self, args: &Vec<P<Expr>>) {
        self.push_str("(");
        for (i, arg) in args.iter().enumerate() {
            self.expr(arg);
            if i + 1 == args.len() {
                self.push_str(")");
            } else {
                self.push_str(", ");
            }
        }
    }

    fn args(&mut self, args: &Vec<Arg>) {
        self.push_str("(");
        for (i, arg) in args.iter().enumerate() {
            self.pat(&arg.pat);
            if i + 1 == args.len() {
                self.push_str(")");
            } else {
                self.push_str(", ");
            }
        }
    }

    fn expr(&mut self, expr: &ast::Expr) {
        match &expr.node {
            ExprKind::Lit(literal) => self.literal(literal),
            ExprKind::Path(_, path) => self.path(path),
            ExprKind::Call(expr, args) => {
                self.expr(expr);
                self.tuple(args);
            }
            ExprKind::Binary(op, lhs, rhs) => {
                self.expr(lhs);
                self.push_str(" ");
                self.push_str(op.node.to_string());
                self.push_str(" ");
                self.expr(rhs);
            }
            ExprKind::Ret(val) => {
                if let Some(ret) = val {
                    self.push_str("return ");
                    self.expr(ret);
                } else {
                    self.push_str("return");
                }
            }

            _ => panic!("unsupported expr: {:?}", expr.node),
        }
    }

    fn path(&mut self, path: &ast::Path) {
        assert!(path.segments.len() == 1, "no support for paths like a::b");
        let ident = path.segments.last().unwrap().identifier;
        self.ident(&ident);
    }

    fn pat(&mut self, pat: &ast::Pat) {
        // lua is a virgin without pattern matching so we assume pat is an ident.
        // (we could add pattern matching if we're smart though)
        match &pat.node {
            PatKind::Ident(_, ident, _) => self.ident(&ident.node),
            PatKind::Path(_, path) => self.path(path),
            _ => panic!("unsupported pat: {:?}", pat),
        }
    }

    fn stmt(&mut self, stmt: &ast::Stmt) {
        match &stmt.node {
            StmtKind::Item(item) => self.item(&item),
            StmtKind::Expr(expr) => self.expr(&expr),
            StmtKind::Semi(expr) => {
                // just an expr with a trailing semicolon
                self.expr(&expr);
            }
            StmtKind::Local(local) => {
                // let <pat>:<ty> = <expr>
                self.push_str("local ");
                self.pat(&local.pat);
                if let Some(init) = &local.init {
                    self.push_str(" = ");
                    self.expr(init);
                }
            }

            // macros seem complicated from the docs so I'll just use functions.
            // eventually it would be cute to compile
            // println!("foo {}", 5) -> print(("foo %d"):format(5))
            _ => panic!("unsupported stmt: {:?}", stmt),
        }
    }

    fn item(&mut self, item: &ast::Item) {
        match &item.node {
            ItemKind::Fn(decl, _, _, _, _, block) => {
                self.push_str(&format!("function {}", item.ident.name));
                self.args(&decl.inputs);
                self.push_str("\n");
                for stmt in &block.stmts {
                    self.stmt(stmt);
                    self.push_str("\n");
                }
                self.push_str("end\n\n");
            }

            _ => panic!("unsupported: {:?}", &item.node),
        }
    }
}
