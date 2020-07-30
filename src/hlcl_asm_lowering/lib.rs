use crate::token::McToken;
use std::iter::FromIterator;
use std::path::Path;
use std::collections::HashMap;

pub mod token;
pub mod translate;

#[derive(Debug)]
pub struct FunctionWriter {
    blocks: Vec<(String, String)>
}

impl FunctionWriter {
    pub fn root_block(&self) -> &str {
        self.blocks[0].1.as_str()
    }
}

impl<'asm> FromIterator<McToken<'asm>> for FunctionWriter {
    fn from_iter<T: IntoIterator<Item=McToken<'asm>>>(iter: T) -> Self {
        let mut blocks = vec![(String::new(), String::with_capacity(128))];
        let mut current_file = &mut blocks[0].1;
        let mut parents = vec![0];
        let mut is_new_line = true;
        let mut current_index = 0;

        for token in iter {
            match token {
                McToken::BeginBlock(id) => {
                    let new_index = blocks.len();
                    parents.push(current_index);
                    current_index = new_index;

                    blocks.push((id.to_string(), String::with_capacity(128)));
                    current_file = &mut blocks.last_mut().unwrap().1;

                    is_new_line = true;
                }
                McToken::EndBlock => {
                    current_index = parents[current_index];
                    current_file = &mut blocks[current_index].1;

                    current_file.push('\n');
                    is_new_line = true;
                }
                McToken::EndLine => {
                    current_file.push('\n');
                    is_new_line = true;
                }
                _ => {
                    if is_new_line {
                        is_new_line = !is_new_line;
                    } else {
                        current_file.push(' ');
                    }

                    current_file.push_str(token.to_str().as_ref());
                }
            }
        }

        FunctionWriter { blocks }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::McToken;
    use hlcl_asm::selector::Selector;
    use crate::FunctionWriter;

    #[test]
    fn works() {
        let data = (Selector::executor(), "name");

        let tokens = {
            use McToken::*;
            vec![
                Execute,
                As,
                Selector(&data.0),
                Run,
                EndLine,
                Execute,
                BeginBlock(&data.1),
                Execute,
                As,
                EndBlock,
                Execute,
            ]
        };

        let writer: FunctionWriter = tokens.into_iter().collect();

        //println!("{:?}", writer);
        assert_eq!("execute as @s run\nexecute\nexecute", writer.root_block());
        assert_eq!("execute as", writer.blocks[1].1);
    }
}