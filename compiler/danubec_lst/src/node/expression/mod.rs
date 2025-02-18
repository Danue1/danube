pub mod array_expression;
pub mod assignment_expression;
pub mod await_expression;
pub mod binary_expression;
pub mod block_expression;
pub mod break_expression;
pub mod continue_expression;
pub mod field_expression;
pub mod for_expression;
pub mod function_call_expression;
pub mod if_expression;
pub mod index_expression;
pub mod let_expression;
pub mod literal_expression;
pub mod loop_expression;
pub mod match_expression;
pub mod method_call_expression;
pub mod path_element;
pub mod range_expression;
pub mod return_expression;
pub mod struct_expression;
pub mod try_expression;
pub mod unary_expression;
pub mod while_expression;
pub mod yield_expression;

pub use array_expression::*;
pub use assignment_expression::*;
pub use await_expression::*;
pub use binary_expression::*;
pub use block_expression::*;
pub use break_expression::*;
pub use continue_expression::*;
pub use field_expression::*;
pub use for_expression::*;
pub use function_call_expression::*;
pub use if_expression::*;
pub use index_expression::*;
pub use let_expression::*;
pub use literal_expression::*;
pub use loop_expression::*;
pub use match_expression::*;
pub use method_call_expression::*;
pub use path_element::*;
pub use range_expression::*;
pub use return_expression::*;
pub use struct_expression::*;
pub use try_expression::*;
pub use unary_expression::*;
pub use while_expression::*;
pub use yield_expression::*;

ast_node! {
    /// ```ebnf
    /// Expression =
    /// | AssignmentKind
    /// ```
    struct Expression;

    node kind -> ExpressionKind;
}

ast_node! {
    /// ```ebnf
    /// ExpressionKind =
    /// | LetExpression
    /// | IfExpression
    /// | MatchExpression
    /// | LoopExpression
    /// | WhileExpression
    /// | ForExpression
    /// | ReturnExpression
    /// | BreakExpression
    /// | ContinueExpression
    ///
    /// | ArrayExpression
    /// | BlockExpression
    /// | LiteralExpression
    /// | UnaryExpression
    /// | PathExpression
    ///
    /// | AssignmentExpression
    /// | BinaryExpression
    /// | RangeExpression
    /// | StructExpression
    /// | CallExpression
    /// | IndexExpression
    /// | FieldExpression
    /// | AwaitExpression
    /// | YieldExpression
    /// ```
    enum ExpressionKind {
        Break(BreakExpression),
        Continue(ContinueExpression),
        For(ForExpression),
        If(IfExpression),
        Let(LetExpression),
        Loop(LoopExpression),
        Match(MatchExpression),
        Return(ReturnExpression),
        While(WhileExpression),

        Array(ArrayExpression),
        Block(BlockExpression),
        Literal(LiteralExpression),
        Path(PathExpression),
        Unary(UnaryExpression),

        Assignment(AssignmentExpression),
        Binary(BinaryExpression),
        Await(AwaitExpression),
        Call(FunctionCallExpression),
        Field(FieldExpression),
        Index(IndexExpression),
        Range(RangeExpression),
        Struct(StructExpression),
        Try(TryExpression),
        Yield(YieldExpression),
    }
}
