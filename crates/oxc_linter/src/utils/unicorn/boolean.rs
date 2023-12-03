use oxc_ast::{
    ast::{CallExpression, Expression},
    AstKind,
};
use oxc_semantic::AstNode;
use oxc_syntax::operator::UnaryOperator;

use crate::{ast_util::outermost_paren_parent, LintContext};

use super::is_logical_expression;
pub fn is_logic_not(node: &AstKind) -> bool {
    matches!(node, AstKind::UnaryExpression(unary_expr) if unary_expr.operator == UnaryOperator::LogicalNot)
}
pub fn is_logic_not_argument(node: &AstNode, ctx: &LintContext) -> bool {
    let parent = ctx.nodes().parent_kind(node.id());
    matches!(parent, Some(parent) if is_logic_not(&parent))
}
pub fn is_boolean_call(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::CallExpression(CallExpression {
            callee: Expression::Identifier(ident),
            arguments,
            ..
        }) if ident.name == "Boolean" && arguments.len() == 1
    )
}
pub fn is_boolean_call_argument<'a, 'b>(node: &'b AstNode<'a>, ctx: &'b LintContext<'a>) -> bool {
    let parent = ctx.nodes().parent_kind(node.id());
    matches!(parent, Some(parent) if is_boolean_call(&parent))
}

pub fn is_boolean_node<'a, 'b>(node: &'b AstNode<'a>, ctx: &'b LintContext<'a>) -> bool {
    let kind = node.kind();

    if is_logic_not(&kind)
        || is_logic_not_argument(node, ctx)
        || is_boolean_call(&kind)
        || is_boolean_call_argument(node, ctx)
    {
        return true;
    }

    let Some(parent) = outermost_paren_parent(node, ctx) else { return false };

    if matches!(
        parent.kind(),
        AstKind::IfStatement(_)
            | AstKind::ConditionalExpression(_)
            | AstKind::WhileStatement(_)
            | AstKind::DoWhileStatement(_)
            | AstKind::ForStatement(_)
    ) {
        return true;
    }

    if is_logical_expression(parent) {
        return is_boolean_node(parent, ctx);
    }

    false
}

pub fn get_boolean_ancestor<'a, 'b>(
    node: &'b AstNode<'a>,
    ctx: &'b LintContext<'a>,
    // (node, is_negative)
) -> (&'b AstNode<'a>, bool) {
    let mut is_negative = false;
    let mut cur = node;
    loop {
        if is_logic_not_argument(cur, ctx) {
            is_negative = !is_negative;
            cur = ctx.nodes().parent_node(cur.id()).unwrap();
        } else if is_boolean_call_argument(cur, ctx) {
            cur = ctx.nodes().parent_node(cur.id()).unwrap();
        } else {
            break;
        }
    }
    (cur, is_negative)
}
