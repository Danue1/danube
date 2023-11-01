use danubec_lex::Lex;
use danubec_parse::Parse;

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
    let ast = Parse::parse(tokens.as_slice());
    dbg!(ast);

    Ok(())
}
