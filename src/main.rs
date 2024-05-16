use std::{env, fs};

const TEMPLATE: &'static str = include_str!("template.asm");

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

fn to_asm(loops: &mut u32, tokens: &Vec<Token>) -> String {
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
                *loops += 2;
                let current = *loops;

                format!(
                    "jmp .L{}\n.L{}:\n{}.L{}:\n    loop_check .L{}",
                    current,
                    current + 1,
                    to_asm(loops, tokens),
                    current,
                    current + 1
                )
            }
        })
        .map(|line| format!("    {line}\n"))
        .collect()
}

fn compile(input_path: &str, output_path: &str) {
    let source = filter_chars(&fs::read_to_string(input_path).unwrap());
    let asm = to_asm(&mut 0, &parse(&source));

    if let Err(e) = fs::write(output_path, TEMPLATE.replace(";   code", &asm)) {
        eprintln!("error writing file: {e}");
    }
}

fn main() {
    compile(
        &env::args().nth(1).expect("no path given"),
        &env::args().nth(2).unwrap_or("./output.asm".to_owned()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    fn run(name: &str, args: &[&str]) -> String {
        let output = Command::new(name)
            .args(args)
            .output()
            .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

        if !output.status.success() {
            panic!(
                "{name} failed: {}",
                core::str::from_utf8(&output.stderr).unwrap()
            );
        }

        String::from_utf8(output.stdout).unwrap()
    }

    fn verify(name: &str) {
        let path = format!("./src/tests/{name}");

        compile(
            &format!("{path}/main.bf"),
            &format!("./src/tests/output/{name}.asm"),
        );
        run(
            "nasm",
            &[
                "-f",
                "elf64",
                &format!("./src/tests/output/{name}.asm"),
                "-o",
                &format!("./src/tests/output/{name}.o"),
            ],
        );
        run(
            "ld",
            &[
                &format!("./src/tests/output/{name}.o"),
                "-o",
                &format!("./src/tests/output/{name}"),
            ],
        );

        assert_eq!(
            run(&format!("./src/tests/output/{name}"), &[]),
            fs::read_to_string(path + "/output.txt").unwrap()
        )
    }

    #[test]
    fn hello() {
        verify("hello");
    }

    #[test]
    fn mandlebrot() {
        verify("mandlebrot");
    }

    #[test]
    fn squares() {
        verify("squares");
    }
}
