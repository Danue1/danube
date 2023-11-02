use danubec_lex::Lex;
use danubec_parse::Parse;
use danubec_syntax_node::node::AstNode;

// TODO: 의존성 패키지 스캔
// TODO: 파일 일기
// TODO: 파일 파싱
// TODO: AST 조립
// TODO: 시맨틱 검증
// TODO: 코드 생성
// TODO: 파일 쓰기
pub fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let source = include_str!("../fixtures/main.dnb");
    let tokens = Lex::lex(source);
    let node = Parse::parse(tokens);
    let ast = AstNode::lower(node);
    dbg!(&ast);

    Ok(())
}
