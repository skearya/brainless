use std::fs;

#[derive(Debug)]
enum Token {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    GetChar,
    PutChar,
    Loop(Vec<Token>),
}

fn filter_chars(input: &str) -> String {
    input
        .chars()
        .filter(|c| ['>', '<', '+', '-', ',', '.', '[', ']'].contains(c))
        .collect()
}

fn recursive_len(tokens: &Vec<Token>) -> usize {
    tokens
        .iter()
        .map(|token| match token {
            Token::Loop(tokens) => recursive_len(tokens) + 2,
            _ => 1,
        })
        .sum()
}

fn parse(source: &str) -> Vec<Token> {
    let mut char_indices = source.char_indices();
    let mut tokens: Vec<Token> = vec![];

    while let Some((i, ch)) = char_indices.next() {
        tokens.push(match ch {
            '>' => Token::IncPtr,
            '<' => Token::DecPtr,
            '+' => Token::IncVal,
            '-' => Token::DecVal,
            ',' => Token::GetChar,
            '.' => Token::PutChar,
            '[' => {
                let tokens = parse(&source[i + 1..]);

                for _ in 0..(recursive_len(&tokens) + 1) {
                    let _ = char_indices.next();
                }

                Token::Loop(tokens)
            }
            ']' => break,
            unexpected => panic!("unexpected character: {unexpected}"),
        });
    }

    tokens
}

fn to_asm(mut loops: u32, tokens: &Vec<Token>) -> String {
    tokens
        .iter()
        .map(|token| match token {
            Token::IncPtr => "inc r8".into(),
            Token::DecPtr => "dec r8".into(),
            Token::IncVal => "inc byte [buffer + r8]".into(),
            Token::DecVal => "dec byte [buffer + r8]".into(),
            Token::GetChar => "get".into(),
            Token::PutChar => "put".into(),
            Token::Loop(tokens) => {
                loops += 5;

                format!(
                    "jmp .L{}\n.L{}:\n{}.L{}:\nloop_check .L{}",
                    loops,
                    loops + 1,
                    to_asm(loops + 1, tokens),
                    loops,
                    loops + 1
                )
            }
        })
        .map(|line| line + "\n")
        .collect()
}

fn main() {
    let template = include_str!("template.asm");
    let source: String = filter_chars(&fs::read_to_string("./src/source.bf").unwrap());
    let asm = to_asm(0, &parse(&source));

    match fs::write("output.asm", template.replace(";   code", &asm)) {
        Ok(_) => println!("wrote asm"),
        Err(e) => eprintln!("error writing file: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reverse_map(tokens: &Vec<Token>) -> String {
        tokens
            .iter()
            .map(|token| match token {
                Token::IncPtr => ">".into(),
                Token::DecPtr => "<".into(),
                Token::IncVal => "+".into(),
                Token::DecVal => "-".into(),
                Token::GetChar => ",".into(),
                Token::PutChar => ".".into(),
                Token::Loop(tokens) => format!("[{}]", reverse_map(tokens)),
            })
            .collect()
    }

    #[test]
    fn parse_squares() {
        assert_eq!(
            reverse_map(&parse(&filter_chars(include_str!("tests/squares.bf")))),
            filter_chars(include_str!("tests/squares.bf"))
        );
    }

    #[test]
    fn parse_hello_world() {
        assert_eq!(
            reverse_map(&parse(&filter_chars(include_str!("tests/hello.bf")))),
            filter_chars(include_str!("tests/hello.bf"))
        );
    }
}
