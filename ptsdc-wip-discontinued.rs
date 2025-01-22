use std::{env, fs};
use std::collections::HashMap;

enum Backend {
    LinuxX86_64fasm,
    C,
}

fn main() {

    let arguments = env::args();
    let mut output = String::from("out.s");
    let mut input = String::new();
    let mut flag = String::new();
    let mut backend = Backend::LinuxX86_64fasm;

    for argument in arguments.skip(1) {

        if argument.ends_with(".ptsd") {

            input = argument;
        } else if argument.starts_with('-') {

            flag = argument;
        } else {

            if flag == String::from("-o") {

                output = argument;
                continue;
            } else if flag == String::from("-b") {

                match argument.as_str() {

                    "fasm-linux-x86_64" => {backend = Backend::LinuxX86_64fasm},
                    "c" => {backend = Backend::C},
                    _ => {panic!("Backend not supported yet!");},
                }
                continue;
            } else {

                eprintln!("\x1B[1;31mERROR:\x1B[0m Invalid Input. Please suffix the file with .ptsd or use a valid flag.");
            }
        }
    }

    let file = fs::read_to_string(input).expect("Sorry, but I couldn't find the file. I really tried my best.");
    let words = to_words(file);

    //debug print
    println!("{words:?}");

    let trees = parse(words);
    //debug print
    println!("{trees:?}");

    let result = match trees {

        Ok(trees) => match backend {
            Backend::LinuxX86_64fasm => to_fasm(trees),
            Backend::C => to_c(trees),
        },
        Err((error, index)) => panic!("\x1B[1;31mERROR:\x1B[0m \"{}\" at {}.", error, index),
    };

    //debug print
    println!("{result:?}");

    match result {

        Ok(converted) => {
            fs::write(output, converted).expect("\x1B[1;31mERROR:\x1B[0m Can't write... What?");
        }
        Err((error, index)) => {eprintln!("\x1B[1;31mERROR:\x1B[0m \"{}\" at {}.", error, index);}
    }

}

fn to_words(sentence: String) -> Vec<String> {

    let mut word = String::new();
    let mut words = Vec::<String>::new();
    let mut comment = false;

    for character in sentence.chars() {

        //parse comments
        if word.ends_with("//") {
            word.pop();
            word.pop(); //pops out the comment part.
            if word.len() > 0 && !comment {

                words.push(word);
            }

            word = String::new();
            comment = !comment;
        }

        if comment {
            word.push(character);
            continue;
        }

        match character {
            ' ' | '\n' | '\t' | '\r' => {if word.len() > 0 {words.push(word); word = String::new();}}
            '{' | '}' => {if word.len() > 0 {words.push(word); word = String::new();} words.push(String::from(character.to_string()));}
            _ => {word.push(character);}
        }
    }

    return words;
}

#[derive(Debug, Clone, PartialEq)]
enum Word {
    Ins(i32),
    Int(i32),
    Fix(i64),
    Cha(char),
    Stk(i32),
    Var(i32),
}

#[derive(Debug, Clone, PartialEq)]
enum Vartype {
    Int,
    Fix,
    Cha,
    Stk,
    Idk,
}

#[derive(Debug)]
struct Tree {
    cur: Option<Word>,
    sub: Vec<Tree>,
}
fn parse(words: Vec<String>) -> Result<Vec<Vec<Tree>>, (String, usize)> {

    //The tuple represents value and type. Using hash maps because of a coin toss.
    let mut var_map = HashMap::<String, Word>::new();
    let mut stack_map = HashMap::<i32, Vec<Word>>::new();
    let mut stack_num = 30;
    let mut var_num = 2;
    let mut code = Vec::<Word>::new();

    //Base operations.
    var_map.insert("EXIT".to_string(), Word::Ins(0));
    var_map.insert("PUSH".to_string(), Word::Ins(1));
    var_map.insert("POP".to_string(), Word::Ins(2));
    var_map.insert("PEEK".to_string(), Word::Ins(3));
    var_map.insert("RENAME".to_string(), Word::Ins(4));
    var_map.insert("CLONE".to_string(), Word::Ins(5));
    var_map.insert("FLUSH".to_string(), Word::Ins(6));
    var_map.insert("ADD".to_string(), Word::Ins(7));
    var_map.insert("SUB".to_string(), Word::Ins(8));
    var_map.insert("MPY".to_string(), Word::Ins(9));
    var_map.insert("DIV".to_string(), Word::Ins(10));
    var_map.insert("IF".to_string(), Word::Ins(11));
    var_map.insert("EQUAL".to_string(), Word::Ins(12));
    var_map.insert("UNEQUAL".to_string(), Word::Ins(13));
    var_map.insert("LESS".to_string(), Word::Ins(14));
    var_map.insert("MORE".to_string(), Word::Ins(15));
    var_map.insert("NOT".to_string(), Word::Ins(16));
    var_map.insert("AND".to_string(), Word::Ins(17));
    var_map.insert("OR".to_string(), Word::Ins(18));
    var_map.insert("PRINT".to_string(), Word::Ins(19));
    var_map.insert("SCAN".to_string(), Word::Ins(20));
    var_map.insert("WRITE".to_string(), Word::Ins(21));
    var_map.insert("READ".to_string(), Word::Ins(22));
    var_map.insert("SIZE".to_string(), Word::Ins(23));
    var_map.insert("INT".to_string(), Word::Ins(24));
    var_map.insert("FIXED".to_string(), Word::Ins(25));
    var_map.insert("CHAR".to_string(), Word::Ins(26));
    var_map.insert("RUN".to_string(), Word::Ins(27));
    var_map.insert("SYS".to_string(), Word::Ins(28));
    var_map.insert("SKIP".to_string(), Word::Ins(29));

    //Their aliases.
    var_map.insert("exit".to_string(), Word::Ins(0));
    var_map.insert("push".to_string(), Word::Ins(1));
    var_map.insert("pop".to_string(), Word::Ins(2));
    var_map.insert("peek".to_string(), Word::Ins(3));
    var_map.insert("rename".to_string(), Word::Ins(4));
    var_map.insert("clone".to_string(), Word::Ins(5));
    var_map.insert("flush".to_string(), Word::Ins(6));
    var_map.insert("add".to_string(), Word::Ins(7));
    var_map.insert("+".to_string(), Word::Ins(7));
    var_map.insert("sub".to_string(), Word::Ins(8));
    var_map.insert("-".to_string(), Word::Ins(8));
    var_map.insert("mpy".to_string(), Word::Ins(9));
    var_map.insert("*".to_string(), Word::Ins(9));
    var_map.insert("div".to_string(), Word::Ins(10));
    var_map.insert("/".to_string(), Word::Ins(10));
    var_map.insert("if".to_string(), Word::Ins(11));
    var_map.insert("equal".to_string(), Word::Ins(12));
    var_map.insert("=".to_string(), Word::Ins(12));
    var_map.insert("unequal".to_string(), Word::Ins(13));
    var_map.insert("!=".to_string(), Word::Ins(13));
    var_map.insert("less".to_string(), Word::Ins(14));
    var_map.insert("<".to_string(), Word::Ins(14));
    var_map.insert("more".to_string(), Word::Ins(15));
    var_map.insert(">".to_string(), Word::Ins(15));
    var_map.insert("not".to_string(), Word::Ins(16));
    var_map.insert("!".to_string(), Word::Ins(16));
    var_map.insert("and".to_string(), Word::Ins(17));
    var_map.insert("&".to_string(), Word::Ins(17));
    var_map.insert("or".to_string(), Word::Ins(18));
    var_map.insert("|".to_string(), Word::Ins(18));
    var_map.insert("print".to_string(), Word::Ins(19));
    var_map.insert("scan".to_string(), Word::Ins(20));
    var_map.insert("write".to_string(), Word::Ins(21));
    var_map.insert("read".to_string(), Word::Ins(22));
    var_map.insert("size".to_string(), Word::Ins(23));
    var_map.insert("int".to_string(), Word::Ins(24));
    var_map.insert("fixed".to_string(), Word::Ins(25));
    var_map.insert("char".to_string(), Word::Ins(26));
    var_map.insert("run".to_string(), Word::Ins(27));
    var_map.insert("sys".to_string(), Word::Ins(28));
    var_map.insert("system".to_string(), Word::Ins(28));
    var_map.insert("SYSTEM".to_string(), Word::Ins(28));
    var_map.insert("skip".to_string(), Word::Ins(29));

    let mut skip = 0;

    for (index, word) in words.iter().enumerate() {

        //skipping for space parsing...
        if skip > 0 {
            skip -= 1; continue;
        }

        //recognize normal (not space or escape) characters.
        if word.starts_with("'") && word.ends_with("'") && word.len() == 3 {
            code.push(Word::Cha(word.chars().nth(1).unwrap()));
            continue;
        }

        //recognize space.
        if word == &String::from("'") {
            if words.get(index + 1).unwrap() == &String::from("'") { code.push(Word::Cha(' '));
                skip = 1;
                continue;
            } else if words.get(index + 1).unwrap() == &String::from("{") && words.get(index + 2).unwrap() == &String::from("'") {
                code.push(Word::Cha('{'));
                skip = 2;
                continue;
            } else if words.get(index + 1).unwrap() == &String::from("}") && words.get(index + 2).unwrap() == &String::from("'") {
                code.push(Word::Cha('}'));
                skip = 2;
                continue;
            } else {
                return Err(("Stray single-quote".to_string(), index));
            }
        }

        //recognize escape.
        if word.starts_with("'") && word.ends_with("'") && word.len() == 4 && word.chars().nth(1).unwrap() == '\\' {
            let esc = match word.chars().nth(2).unwrap() {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\0' => '\0',
                _ => {return Err(("Invalid escape character".to_string(), index));}
            };

            code.push(Word::Cha(esc));
            continue;
        }

        //recognize ints.
        if let Ok(int) = word.parse::<i32>() {
            code.push(Word::Int(int));
            continue;
        }

        //recognize fixeds.
        if let Ok(fixed) = word.parse::<f64>() {
            code.push(Word::Fix((fixed * ((1 as i64) << 32) as f64) as i64));
            continue;
        }
        //recognize stacks.
        if word == "{" {
            code.push(Word::Var(-1));
            continue;
        }

        if word == "}" {
            //pop everything until a { is encountered and
            //replace the block with the designated stack number.
            //compared to the "main"
            let mut symbol = code.pop().expect("Some unclosed bracket somewhere.");
            let mut stack = Vec::<Word>::new();
            while match symbol {
                Word::Var(vnum) => {if vnum == -1 { false } else { true } }
                _ => true
            } {

                stack.push(symbol);
                symbol = code.pop().expect("Some unclosed bracket somewhere.");
            }

            stack.reverse();
            stack_map.insert(stack_num, stack);
            code.push(Word::Stk(stack_num));
            stack_num += 1;
            continue;
        }

        //recognize variables.
        if !var_map.contains_key(word) {
            var_map.insert(word.clone(), Word::Var(var_num));
            var_num += 1;
            //debug print
            println!("{word}");

        }
        code.push(var_map.get(word).unwrap().clone());
    }

//    let mut rev_code = Vec::<Word>::new();
//
//    for symbol in code.into_iter().rev() {
//        rev_code.push(symbol);
//    }
//
    if !code.last().expect("Empty code.").eq(&Word::Ins(0)) {
        code.push(Word::Ins(0));
    };

    //debug print
    println!("{code:?}");
    let mut trees = Vec::<Vec<Tree>>::new();
    trees.push(Vec::<Tree>::new());

    let mut vars = Vec::<Vartype>::new();
    while var_num > 0 {
        var_num -= 1;
        vars.push(Vartype::Idk);
    }
    while code.len() > 0 {
        trees[0].push(match to_tree(&mut code, stack_map.clone(), &mut vars) {
            Ok(tree) => tree,
            Err(tuple) => {return Err(tuple);}
        });
    }

    for (index, mut stack) in stack_map.clone().into_iter().enumerate() {

        trees.push(Vec::<Tree>::new());
        println!("Parsing stack {}...", stack.0 - 29);
        while stack.1.len() > 0 {
            trees[index].push(match to_tree(&mut stack.1, stack_map.clone(), &mut vars) {
                Ok(tree) => tree,
                Err(tuple) => {return Err(tuple);}
            });
        }
    }

    return Ok(trees);
}

fn to_tree(code: &mut Vec<Word>, stack_map: HashMap<i32, Vec<Word>>, vars: &mut Vec<Vartype>) -> Result<Tree, (String, usize)> {

    let mut tree = Tree { cur: code.pop(), sub: Vec::<Tree>::new() };
    let index = code.len();

    //Recursion here.
    match tree.cur {

        Some(ref word) => match word {
            Word::Ins(inst) => {
                match inst {
                    20 => {},
                    0 | 2 | 3 | 6 | 16 | 19 | 22 | 23 | 24 | 25 | 26 | 27 | 28 => {

                        let subtree = match to_tree(code, stack_map, vars) {
                            Ok(subtree) => subtree,
                            Err(error) => { return Err(error); }
                        };

                        tree.sub.push(subtree);
                    },
                    1 | 4 | 5 | 7 ..= 15 | 17 | 18 | 21 | 29 => {
                        let subtree = match to_tree(code, stack_map.clone(), vars) {
                            Ok(subtree) => subtree,
                            Err(error) => { return Err(error); }
                        };

                        tree.sub.push(subtree);

                        let subtree = match to_tree(code, stack_map, vars) {
                            Ok(subtree) => subtree,
                            Err(error) => { return Err(error); }
                        };

                        tree.sub.push(subtree);
                    },
                    _ => { return Err(("Illegal Instruction".to_string(), index)); },
                };
    
            },
            _ => {},
        },
        None => {return Ok(tree);},
    }

    //Check if the inputs are correct.
    match check_bugs(&tree, vars) {
        
        Some(message) => match message {
            
            Ok(warning) => { eprintln!("\x1B[1;33mWARNING:\x1B[0m \"{warning}\" at {index}"); },
            Err(error) => { return Err((error, index)); }
        },
        None => {}
    };

    return Ok(tree);
}

//Extremely messy. Surely there's a better way to do this.
fn check_bugs(tree: &Tree, vars: &mut Vec<Vartype>) -> Option<Result<String, String>> {
    //Ignore the branch entirely if it's got a RUN or IF in it because screw that. (for variables)
    //Shallow check because tree.cur should tell us the type.
    match tree.cur {
        Some(Word::Ins(inst)) => {
            match inst {

                0 | 29 => { //Requires one whole number. (exit & skip)

                    match tree.sub[0].cur {
                        Some(Word::Int(_)) => { return None; },
                        Some(Word::Cha(_)) => { return None; },
                        Some(Word::Ins(ins)) => match ins {
                            2 | 3 => { //POP & PEEK. Check subtree for vartype.
                                match &tree.sub[0].sub[0].cur {
                                    Some(word) => {
                                        match word {
                                            Word::Var(var_num) => {
                                                match vars[*var_num as usize] {
                                                    Vartype::Stk => { return Some(Err("Operation input type mismatch. Should be either an integer or a character.".to_string())); },
                                                    Vartype::Idk => { return Some(Ok("Don't know what the variable type here is, might cause weird things.".to_string())); },
                                                    _ => { return None; }
                                                };
                                            },
                                            _ => { return Some(Err("Operation input type mismatch. Should be a literal.".to_string())); },
                                        };
                                    },
                                    None => { return Some(Err("Apparently there was a POP/PEEK without an input.".to_string())); },
                                };
                            },
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            7 ..= 10 | 12 ..= 18 | 20 | 24 | 26 => { return None }, //Numeric & Booleans & Casts & Scan.
                            _ => { return Some(Err("Operation input type mismatch. Should either be an integer or a character.".to_string())); }
                        },
                        _ => { return Some(Err("Operation input type mismatch. Should either be an integer or a character.".to_string())); }
                    };
                },
                1 => { //lit var

                    let mut var_num = -1;
                    match tree.sub[0].cur {
                        Some(Word::Var(varnum)) => {
                            var_num = varnum;
                        },
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            _ => { return Some(Err("Pushing into a non-variable is not allowed.".to_string())); }
                        }
                        _ => { return Some(Err("Pushing into a non-variable is not allowed.".to_string())); }
                    };
                    if var_num == -1 {
                        //The warning (or error) was already sent.
                        return None;
                    }
                    match tree.sub[1].cur {
                        //check the lit and label the variable.
                        Some(Word::Int(_)) => {
                            if vars[var_num as usize] == Vartype::Int {
                                return None;
                            } else if vars[var_num as usize] == Vartype::Idk {
                                vars[var_num as usize] = Vartype::Int;
                                return None;
                            } else {
                                return Some(Err("Variable PUSHED Type mismatch.".to_string()));
                            }
                        },
                        Some(Word::Fix(_)) => {
                            if vars[var_num as usize] == Vartype::Fix {
                                return None;
                            } else if vars[var_num as usize] == Vartype::Idk {
                                vars[var_num as usize] = Vartype::Fix;
                                return None;
                            } else {
                                return Some(Err("Variable PUSHED Type mismatch.".to_string()));
                            }
                        },
                        Some(Word::Cha(_)) => {
                            if vars[var_num as usize] == Vartype::Cha {
                                return None;
                            } else if vars[var_num as usize] == Vartype::Idk {
                                vars[var_num as usize] = Vartype::Cha;
                                return None;
                            } else {
                                return Some(Err("Variable PUSHED Type mismatch.".to_string()));
                            }
                        },
                        Some(Word::Stk(_)) => {
                            if vars[var_num as usize] == Vartype::Stk {
                                return None;
                            } else if vars[var_num as usize] == Vartype::Idk {
                                vars[var_num as usize] = Vartype::Stk;
                                return None;
                            } else {
                                return Some(Err("Variable PUSHED Type mismatch.".to_string()));
                            }
                        },
                        Some(Word::Ins(_)) => {
                                return Some(Ok("I can't help you check the type of the variable being pushed here. Might cause some weird stuff.".to_string()));
                        },
                        _ => { return Some(Err("Unimplemented.".to_string())); }
                    };
                },
                2 | 3 | 6 | 23 | 28 => { //var

                    match tree.sub[0].cur {
                        Some(Word::Var(_)) => { return None; },
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            _ => { return Some(Err("Operations that only accept one 'variable' argument don't allow this input.".to_string())); }
                        }
                        _ => { return Some(Ok("I'm letting this pass, but know that this stack needs input, so if this is main, fix it.".to_string())); }
                    };
                },
                4 | 5 | 21 | 22 => { //var var

                    match tree.sub[0].cur {
                        Some(Word::Var(_)) => {},
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            _ => { return Some(Err("Operations that only accept two 'variable's argument don't allow this input.".to_string())); }
                        }
                        _ => { return Some(Ok("I'm letting this pass, but know that this stack needs input, so if this is main, fix it.".to_string())); }
                    };
                    match tree.sub[1].cur {
                        Some(Word::Var(_)) => { return None; },
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            1 | 4 ..= 6 | 19 | 21 | 22 => { return Some(Ok("I'm letting this pass, but know that this stack needs input, so if this is main, fix it.".to_string())); } //Operations that don't yield anything.
                            _ => { return Some(Err("Operations that only accept two 'variable's argument don't allow this input.".to_string())); }
                        }
                        _ => { return Some(Ok("I'm letting this pass, but know that this stack needs input, so if this is main, fix it.".to_string())); }
                    };
                },
                7 ..= 10 | 12 ..=18 => { //lit lit

                    return Some(Err("Unimplemented".to_string()));
                },
                11 => { //lit stk

                    return Some(Err("Unimplemented".to_string()));
                },
                19 | 24 ..= 26 => { //lit

                    match tree.sub[0].cur {
                        Some(Word::Int(_)) | Some(Word::Fix(_)) | Some(Word::Cha(_)) => { return None; },
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            2 | 3 => { //POP & PEEK. Check subtree for vartype.
                                match &tree.sub[0].sub[0].cur {
                                    Some(word) => {
                                        match word {
                                            Word::Var(var_num) => {
                                                match vars[*var_num as usize] {
                                                    Vartype::Stk => { return Some(Err("Operation input type mismatch. Should be a 'literal'".to_string())); },
                                                    Vartype::Idk => { return Some(Ok("Don't know what the variable type here is, might cause weird things.".to_string())); },
                                                    _ => { return None; }
                                                };
                                            },
                                            Word::Ins(ins) => match ins {
                                                11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                                                _ => {return Some(Err("Operation input type mismatch. Should be a stack.".to_string())); }
                                            }
                                            _ => { return Some(Err("Operation input type mismatch. Should be a literal.".to_string())); },
                                        };
                                    },
                                    None => { return Some(Err("Apparently there was a POP/PEEK without an input.".to_string())); },
                                };
                            },
                            _ => { return Some(Err("Operations that only accept one 'literal' argument don't allow this input.".to_string())); }
                        },
                        _ => { return Some(Err("Operations that only accept one 'literal' argument don't allow this input.".to_string())); }
                    };
                },
                20 => { //None

                    return None;
                },
                27 => { //stk

                    match tree.sub[0].cur {
                        Some(Word::Stk(_)) => { return None; },
                        Some(Word::Ins(ins)) => match ins {
                            11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                            2 | 3 => { //POP & PEEK. Check subtree for vartype.
                                match &tree.sub[0].sub[0].cur {
                                    Some(word) => {
                                        match word {
                                            Word::Var(var_num) => {
                                                match vars[*var_num as usize] {
                                                    Vartype::Stk => { return None; },
                                                    Vartype::Idk => { return Some(Ok("Don't know what the variable type here is, might cause weird things.".to_string())); },
                                                    _ => { return Some(Err("Operation input type mismatch. Should be a 'stack'".to_string())); }
                                                };
                                            },
                                            Word::Ins(ins) => match ins {
                                                11 | 27 | 28 | 29 => { return Some(Ok("I'm not going to bother checking for error here (for now). If there's a segmentation fault, better check here.".to_string())); }, //IF & RUN & SYS. Emit warning for now.
                                                _ => {return Some(Err("Operation input type mismatch. Should be a stack.".to_string())); }
                                            }
                                            _ => { return Some(Err("Operation input type mismatch. Should be a stack.".to_string())); },
                                        };
                                    },
                                    None => { return Some(Err("Apparently there was a POP/PEEK without an input.".to_string())); },
                                };
                            },
                            _ => { return Some(Err("Operations that only accept one 'stack' argument don't allow this input.".to_string())); }
                        }
                        _ => { return Some(Err("Operations that only accept one 'stack' argument don't allow this input.".to_string())); }
                    };
                },

                _ => { return Some(Err("Illegal Instruction.".to_string())); }
            };
        },
        _ => { return None; } //They aren't instructions.
    }
}

fn to_fasm(trees: Vec<Vec<Tree>>) -> Result<String, (String, usize)> {
    todo!();
}

fn to_c(code: Vec<Vec<Tree>>) -> Result<String, (String, usize)> {
    todo!();
}

fn test_program(inputs: String) {
    todo!();
}
