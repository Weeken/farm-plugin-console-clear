use farmfe_core::swc_common::DUMMY_SP;
use farmfe_core::swc_ecma_ast::*;
use farmfe_toolkit::swc_ecma_visit::{noop_fold_type, Fold, FoldWith};

pub struct RemoveConsole {}

impl RemoveConsole {
  fn is_console(&self, ident: &Ident) -> bool {
    &ident.sym == "console"
  }
  fn should_remove_call(&mut self, n: &CallExpr) -> bool {
    let callee = &n.callee;
    let member_expr = match callee {
      Callee::Expr(e) => match &**e {
        Expr::Member(m) => m,
        _ => return false,
      },
      _ => return false,
    };

    match &*member_expr.obj {
      Expr::Ident(i) if self.is_console(i) => {}
      _ => return false,
    }
    true
  }
}

impl Fold for RemoveConsole {
  noop_fold_type!();

  fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
    if let Stmt::Expr(e) = &stmt {
      if let Expr::Call(c) = &*e.expr {
        if self.should_remove_call(c) {
          return Stmt::Empty(EmptyStmt { span: DUMMY_SP });
        }
      }
    }
    stmt.fold_children_with(self)
  }
}

pub fn remove_console() -> impl Fold {
  RemoveConsole {}
}
