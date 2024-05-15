use std::fs;

enum Token {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    GetChar,
    PutChar,
    StartLoop,
    EndLoop,
}

fn parse(source: &str) -> Vec<Token> {
    source
        .chars()
        .filter(|c| ['>', '<', '+', '-', ',', '.', '[', ']'].contains(c))
        .map(|c| match c {
            '>' => Token::IncPtr,
            '<' => Token::DecPtr,
            '+' => Token::IncVal,
            '-' => Token::DecVal,
            ',' => Token::GetChar,
            '.' => Token::PutChar,
            '[' => Token::StartLoop,
            ']' => Token::EndLoop,
            unexpected => panic!("unexpected character: {unexpected}"),
        })
        .collect()
}

fn main() {
    let template = include_str!("template.asm");
    let source = include_str!("source.bf");

    let asm = parse(source).into_iter().map(|token| match token {
        Token::IncPtr => "inc r8",
        Token::DecPtr => "dec r8",
        Token::IncVal => "inc byte [buffer + r8]",
        Token::DecVal => "dec byte [buffer + r8]",
        Token::GetChar => "get",
        Token::PutChar => "put",
        _ => todo!(),
    });

    let asm_str: String = asm.map(|line| "    ".to_owned() + line + "\n").collect();

    match fs::write("output.asm", template.replace(";   code", &asm_str)) {
        Ok(_) => println!("wrote asm"),
        Err(e) => eprintln!("error writing file: {e}"),
    }
}
