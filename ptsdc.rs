use std::{env, fs};
use std::collections::HashMap;

fn main() {

    let arguments = env::args();
    let mut output = String::from("out.c");
    let mut input = String::new();
    let mut flag = String::new();

    for argument in arguments.skip(1) {

        if argument.ends_with(".ptsd") {

            input = argument;
        } else if argument.starts_with('-') {

            flag = argument;
        } else {

            if flag == String::from("-o") {

                output = argument; } else {

                eprintln!("ERROR: Invalid Input. Please suffix the file with .ptsd or use a valid flag.");
            }
        }
    }

    let file = fs::read_to_string(input).expect("Sorry, but I couldn't find the file. I really tried my best.");
    let words = to_words(file);

    let result = to_c(words);
    match result {

        Ok(converted) => {
            fs::write(output, converted).expect("ERROR: Can't write... What?");
        }
        Err((error, index)) => {eprintln!("ERROR: {error} at {index}");}
    }

}

fn to_words(sentence: String) -> Vec<String> {

    let mut word = String::new();
    let mut words = Vec::<String>::new();

    for character in sentence.chars() {
        match character {
            ' ' | '\n' | '\t' | '\r' => {if word.len() > 0 {words.push(word); word = String::new();}}
            '{' | '}' => {if word.len() > 0 {words.push(word); word = String::new();} words.push(String::from(character.to_string()));}
            _ => {word.push(character);}
        }
    }

    return words;
}

fn to_c(words: Vec<String>) -> Result<String, (String, usize)> {

    //The tuple represents value and type. Using hash maps because of a coin toss.
    let mut var_map = HashMap::<String, (i32, i32)>::new();
    let mut stack_map = HashMap::<i32, Vec<(i32, i32)>>::new();
    let mut stack_num = 29;
    let mut var_num = 2;
    let mut code = Vec::<(i32, i32)>::new();

    //Base operations.
    var_map.insert("EXIT".to_string(), (0, 0));
    var_map.insert("PUSH".to_string(), (1, 0));
    var_map.insert("POP".to_string(), (2, 0));
    var_map.insert("PEEK".to_string(), (3, 0));
    var_map.insert("RENAME".to_string(), (4, 0));
    var_map.insert("CLONE".to_string(), (5, 0));
    var_map.insert("FLUSH".to_string(), (6, 0));
    var_map.insert("ADD".to_string(), (7, 0));
    var_map.insert("SUB".to_string(), (8, 0));
    var_map.insert("MPY".to_string(), (9, 0));
    var_map.insert("DIV".to_string(), (10, 0));
    var_map.insert("IF".to_string(), (11, 0));
    var_map.insert("EQUAL".to_string(), (12, 0));
    var_map.insert("UNEQUAL".to_string(), (13, 0));
    var_map.insert("LESS".to_string(), (14, 0));
    var_map.insert("MORE".to_string(), (15, 0));
    var_map.insert("NOT".to_string(), (16, 0));
    var_map.insert("AND".to_string(), (17, 0));
    var_map.insert("OR".to_string(), (18, 0));
    var_map.insert("PRINT".to_string(), (19, 0));
    var_map.insert("SCAN".to_string(), (20, 0));
    var_map.insert("WRITE".to_string(), (21, 0));
    var_map.insert("READ".to_string(), (22, 0));
    var_map.insert("SIZE".to_string(), (23, 0));
    var_map.insert("INT".to_string(), (24, 0));
    var_map.insert("FIXED".to_string(), (25, 0));
    var_map.insert("CHAR".to_string(), (26, 0));
    var_map.insert("RUN".to_string(), (27, 0));
    var_map.insert("SYS".to_string(), (28, 0));

    //Their aliases.
    var_map.insert("exit".to_string(), (0, 0));
    var_map.insert("push".to_string(), (1, 0));
    var_map.insert("pop".to_string(), (2, 0));
    var_map.insert("peek".to_string(), (3, 0));
    var_map.insert("rename".to_string(), (4, 0));
    var_map.insert("clone".to_string(), (5, 0));
    var_map.insert("flush".to_string(), (6, 0));
    var_map.insert("add".to_string(), (7, 0));
    var_map.insert("+".to_string(), (7, 0));
    var_map.insert("sub".to_string(), (8, 0));
    var_map.insert("-".to_string(), (8, 0));
    var_map.insert("mpy".to_string(), (9, 0));
    var_map.insert("*".to_string(), (9, 0));
    var_map.insert("div".to_string(), (10, 0));
    var_map.insert("/".to_string(), (10, 0));
    var_map.insert("if".to_string(), (11, 0));
    var_map.insert("equal".to_string(), (12, 0));
    var_map.insert("=".to_string(), (12, 0));
    var_map.insert("unequal".to_string(), (13, 0));
    var_map.insert("!=".to_string(), (13, 0));
    var_map.insert("less".to_string(), (14, 0));
    var_map.insert("<".to_string(), (14, 0));
    var_map.insert("more".to_string(), (15, 0));
    var_map.insert(">".to_string(), (15, 0));
    var_map.insert("not".to_string(), (16, 0));
    var_map.insert("!".to_string(), (16, 0));
    var_map.insert("and".to_string(), (17, 0));
    var_map.insert("&".to_string(), (17, 0));
    var_map.insert("or".to_string(), (18, 0));
    var_map.insert("|".to_string(), (18, 0));
    var_map.insert("print".to_string(), (19, 0));
    var_map.insert("scan".to_string(), (20, 0));
    var_map.insert("write".to_string(), (21, 0));
    var_map.insert("read".to_string(), (22, 0));
    var_map.insert("size".to_string(), (23, 0));
    var_map.insert("int".to_string(), (24, 0));
    var_map.insert("fixed".to_string(), (25, 0));
    var_map.insert("char".to_string(), (26, 0));
    var_map.insert("run".to_string(), (27, 0));
    var_map.insert("sys".to_string(), (28, 0));
    var_map.insert("system".to_string(), (28, 0));
    var_map.insert("SYSTEM".to_string(), (28, 0));

    let mut skip = false;
    let mut comment = false;

    for (index, word) in words.iter().enumerate() {

        //parse comments
        if *word == String::from("//") {
            comment = !comment;
        }

        if comment || *word == String::from("//") {
            skip = true;
        }

        //skipping for space parsing...
        if skip {
            skip = false;
            continue;
        }

        //recognize normal (not space or escape) characters.
        if word.starts_with("'") && word.ends_with("'") && word.len() == 3 {
            code.push((word.chars().nth(1).unwrap() as i32, 3));
            continue;
        }

        //recognize space.
        if word == &String::from("'") {
            if words.get(index + 1).unwrap() == &String::from("'") {
                code.push((' ' as i32, 3));
                skip = true;
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

            code.push((esc as i32, 3));
            continue;
        }

        //recognize ints.
        if let Ok(int) = word.parse::<i32>() {
            code.push((int, 1));
            continue;
        }

        //recognize fixeds.
        if let Ok(fixed) = word.parse::<f32>() {
            code.push(((fixed * (1 << 16) as f32) as i32, 2));
            continue;
        }

        //recognize enclosures.
        if word == "{" {
            code.push((-1, -1));
            continue;
        }

        if word == "}" {
            //pop everything until a { is encountered and
            //replace the block with the designated stack number.
            //compared to the "main"
            let mut symbol = code.pop().expect("Some unclosed bracket somewhere.");
            let mut stack = Vec::<(i32, i32)>::new();
            let mut comment = false;
            while symbol != (-1, -1) {
                //parse comments
                if *word == String::from("//") {
                    comment = !comment;
                }

                if comment || *word == String::from("//") {
                    continue;
                }

                stack.push(symbol);
                symbol = code.pop().expect("Some unclosed bracket somewhere.");
            }

            stack_map.insert(stack_num, stack);
            code.push((stack_num, -2));
            stack_num += 1;
            continue;
        }

        //recognize variables.
        if !var_map.contains_key(word) {
            var_map.insert(word.clone(), (var_num, -1));
            var_num += 1;
        }
        code.push(*var_map.get(word).unwrap());
    }

    let mut rev_code = Vec::<(i32, i32)>::new();

    for symbol in code.into_iter().rev() {
        rev_code.push(symbol);
    }

    if !rev_code.contains(&(0, 0)) {
        panic!("You forgot to put in an exit statement!");
    }
            
    Ok(add_boilerplate(rev_code, stack_map, var_num))
}

fn add_boilerplate(code: Vec<(i32, i32)>, stack_map: HashMap<i32, Vec<(i32, i32)>>, var_num: i32) -> String {

    let mut c_code = String::from("
//Boilerplate. Cannot modify.
#include <stdio.h>
#include <stdlib.h>
#define VAR_COUNT ");
    c_code.push_str(&var_num.to_string());

    c_code.push_str("
//For now, stack (as a var) = -2 (parsed by compiler somehow), var = -1, inst = 0, int = 1, fixed = 2 (Unimplemented), char = 3.
typedef struct {
	long val;
	long type;
} Value;

typedef struct node {
	Value* val;
	struct node* next;
} Node;

typedef struct {
	long size;
	Node* top;
} Stack;

Stack* vars[VAR_COUNT];

void push(long stack, long val, long type) {

	Node* new_node = malloc(sizeof(Node));
	new_node->val = malloc(sizeof(Value));
	new_node->val->val = val;
	new_node->val->type = type;
	new_node->next = vars[stack]->top;
	vars[stack]->top = new_node;
	vars[stack]->size++;
}

Value pop(long stack) {

	if (vars[stack]->size == 0) {
		fprintf(stderr, \"Popped too hard!\\n\");
	}

	Node* top = vars[stack]->top;
	vars[stack]->top = vars[stack]->top->next;
	vars[stack]->size--;
	Value ret_val;
	ret_val.val = top->val->val;
	ret_val.type = top->val->type;
	free(top->val);
	free(top);

	return ret_val;
}

Value peek(long stack) {

	Value ret_val;
	ret_val.val = vars[stack]->top->val->val;
	ret_val.type = vars[stack]->top->val->type;
	return ret_val;
}

long size(long stack) {

	return vars[stack]->size;
}

void flush(long stack) {

	vars[stack]->size = 0;
	Node* top = vars[stack]->top;
	while (top != NULL) {
		Node* next = top->next;
		free(top->val);
		free(top);
		top = next;
	}
	vars[stack]->top = top;
}

void ren(long old, long new) {

	if (vars[new] != vars[old]) {
		Node* top = vars[new]->top;
		while (top != NULL) {
			Node* next = top->next;
			free(top->val);
			free(top);
			top = next;
		}

		vars[new] = vars[old];
 	   	Stack* newOld = malloc(sizeof(Stack));
		newOld->size = 0;
		newOld->top = NULL;
		vars[old] = newOld;
	}
}

void clone(long orig, long clon) {

	if (vars[orig] != vars[clon]) {
		Node* otop = vars[orig]->top;
		Node* ctop = vars[clon]->top;
		vars[clon]->size = vars[orig]->size;

		while (ctop != NULL) {
			Node* next = ctop->next;
			free(ctop->val);
			free(ctop);
			ctop = next;
		}

		vars[clon]->top = malloc(sizeof(Node));
		vars[clon]->top->val = malloc(sizeof(Value));
		ctop = vars[clon]->top;

		while (otop->next != NULL) {
			ctop->val->val = otop->val->val;
			ctop->val->type = otop->val->type;
			ctop->next = malloc(sizeof(Node));
			ctop->next->val = malloc(sizeof(Value));
			otop = otop->next;
			ctop = ctop->next;
		}

		ctop->val->val = otop->val->val;
		ctop->val->type = otop->val->type;
		ctop->next = NULL;
	}
}

long fixed2dec(long num) {

	long i = 500000;
	long ret_num = 0;

	for(short count = 15; count >= 0; count--) {
		if (num >= 1 << count) {
			num -= 1 << count;
			ret_num += i;
		}
		i /= 2;
	}

	return ret_num;
}

int main() {

	for (long i = 0; i < VAR_COUNT; i++) {
		vars[i] = malloc(sizeof(Stack));
		vars[i]->size = 0;
		vars[i]->top = NULL;
	}

	Value top;
	top.val = 0;
	top.type = 0;

	long ffront = 0;
	long fback = 0; 
        FILE* file;
        char* cbuf;
        Node* stop;

	//Start of global code.
	//Reminder that variables start at 2.
");
    
    //push main
    for word in code {
        let mut line_of_code = String::from("\tpush(0, ");
        line_of_code.push_str(&word.0.to_string());
        line_of_code.push_str(", ");
        line_of_code.push_str(&word.1.to_string());
        line_of_code.push_str(");\n");
        c_code.push_str(&line_of_code);
    }

    //switch hub
    c_code.push_str("
\t//End of global code.
	goto L0;

L0:
	//Switch hub.
	if (size(0) == 0) return 1;
	top = pop(0);
	switch (top.type) {
		case 0: switch (top.val) {
			case 0: return peek(1).val; break;
			case 1: goto L1; break;		//PUSH
			case 2: goto L2; break;		//POP
			case 3: goto L3; break;		//PEEK
			case 4: goto L4; break;		//RENAME
			case 5: goto L5; break;		//CLONE
			case 6: goto L6; break;		//FLUSH
			case 7: goto L7; break;		//ADD
			case 8: goto L8; break;		//SUB
			case 9: goto L9; break;		//MPY
			case 10: goto L10; break;	//DIV
			case 11: goto L11; break;	//IF
			case 12: goto L12; break;	//EQUAL
			case 13: goto L13; break;	//UNEQUAL
			case 14: goto L14; break;	//LESS THAN
			case 15: goto L15; break;	//MORE THAN
			case 16: goto L16; break;	//NOT
			case 17: goto L17; break;	//AND
			case 18: goto L18; break;	//OR
			case 19: goto L19; break;	//PRINT
			case 20: goto L20; break;	//SCAN
			case 21: goto L21; break;	//WRITE
			case 22: goto L22; break;	//READ
			case 23: goto L23; break;	//SIZE
			case 24: goto L24; break;	//INT
			case 25: goto L25; break;	//FIXED
			case 26: goto L26; break;	//CHAR
			case 27: goto L27; break;	//RUN
			case 28: goto L28; break;	//SYS
"); 

    //push switch
    for stack in &stack_map {
        let mut line_of_code = String::from("\t\t\tcase ");
        line_of_code.push_str(&stack.0.to_string());
        line_of_code.push_str(": goto L");
        line_of_code.push_str(&stack.0.to_string());
        line_of_code.push_str("; break;\n");
        c_code.push_str(&line_of_code);
    }

    c_code.push_str("			default: return 1;
		}

		default: push(1, top.val, top.type); goto L0;
	}
L1:	//push into var.
	if (peek(1).type == -1) return 5;
	push(pop(1).val, pop(1).val, peek(1).type);
	goto L0;

L2:	//pop from var.
	if (peek(1).type != -1) return 5;
	push(1, pop(pop(1).val).val, peek(peek(1).val).type);
	goto L0;

L3:	//peek var.
	if (peek(1).type != -1) return 5;
	push(1, peek(pop(1).val).val, peek(peek(1).val).type);
	goto L0;

L4:	//rename var.
	if (peek(1).type != -1) return 5;
	ren(pop(1).val, pop(1).val);
	goto L0;

L5:	//clone var.
	if (peek(1).type != -1) return 5;
	clone(pop(1).val, pop(1).val);
	goto L0;

L6:	//flush.
	if (peek(1).type != -1) return 5;
	flush(pop(1).val);
	goto L0;

L7:	//add.
	top = pop(1);
	if (top.type < 1) return 5;
	if (peek(1).type < 1) return 5;
	if (((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) || top.type == peek(1).type) {
		push(1, top.val + pop(1).val, top.type);
	} else if (top.type == 1 || top.type == 3) {
		push(1, (top.val << 16) + pop(1).val, top.type);
	} else {
		push(1, top.val + (pop(1).val << 16), top.type);
	}
	goto L0;

L8:	//sub.
	top = pop(1);
	if (top.type < 1) return 5;
	if (peek(1).type < 1) return 5;
	if (((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) || top.type == peek(1).type) {
		push(1, pop(1).val - top.val, top.type);
	} else if (top.type == 1 || top.type == 3) {
		push(1, pop(1).val - (top.val << 16), top.type);
	} else {
		push(1, (pop(1).val << 16) - top.val, top.type);
	}
	goto L0;
	
L9:	//mpy.
	top = pop(1);
	if (top.type < 1) return 5;
	if (peek(1).type < 1) return 5;
	if ((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) {
		push(1, pop(1).val * top.val, top.type);
	} else if ((top.type == 1 || top.type == 3) || (peek(1).type == 1 || peek(1).type == 3)) {
		push(1, pop(1).val * top.val, 2);
	} else {
		push(1, ((long long)(pop(1).val) * (long long)(top.val)) >> 16, 2); //still very rough.
	}
	goto L0;

L10:	//div. To be fixed.
	if (peek(1).type == -1) return 5;
	top = pop(1);
	if (peek(1).type == 1 || peek(1).type == 3) {
		push(1, pop(1).val / top.val, top.type);
	} else if ((top.type == 1 || top.type == 3)) {
		push(1, (long long)(pop(1).val) << 32 / top.val, 2);
	} else {
		push(1, (((long long)(pop(1).val) << 32) / top.val) >> 16, 2); //still very rough.
	}
	goto L0;

L11:	//if. no else.
	top = pop(1);
	if (top.type != -2) return 5;
	if (peek(1).type < 1) return 5;
	switch (pop(1).val) {
		case 0: break;
		case 1: push(0, top.val, 0); break;
		default: return 3;
	}
	goto L0;

L12:	//equal.
        top = pop(1);
	if (top.type == -1) return 5;
	if (peek(1).type == -1) return 5;
        if (top.type != 2 && peek(1).type == 2) top.val <<= 16;
        if (top.type == 2 && peek(1).type != 2) {if (pop(1).val << 16 == top.val) push(1, 1, 1); else push(1, 0, 1); goto L0;}
	if (top.val == pop(1).val) push(1, 1, 1); else push(1, 0, 1);
	goto L0;

L13:	//unequal.
        top = pop(1);
	if (top.type == -1) return 5;
	if (peek(1).type == -1) return 5;
        if (top.type != 2 && peek(1).type == 2) top.val <<= 16;
        if (top.type == 2 && peek(1).type != 2) {if (pop(1).val << 16 != top.val) push(1, 1, 1); else push(1, 0, 1); goto L0;}
	if (top.val != pop(1).val) push(1, 1, 1); else push(1, 0, 1);
	goto L0;

L14:	//less than. Reversed because of how C works...
        top = pop(1);
	if (top.type == -1) return 5;
	if (peek(1).type == -1) return 5;
        if (top.type != 2 && peek(1).type == 2) top.val <<= 16;
        if (top.type == 2 && peek(1).type != 2) {if (pop(1).val << 16 <= top.val) push(1, 0, 1); else push(1, 1, 1); goto L0;}
	if (top.val <= pop(1).val) push(1, 0, 1); else push(1, 1, 1);
	goto L0;

L15:	//more than.
        top = pop(1);
	if (top.type == -1) return 5;
	if (peek(1).type == -1) return 5;
        if (top.type != 2 && peek(1).type == 2) top.val <<= 16;
        if (top.type == 2 && peek(1).type != 2) {if (pop(1).val << 16 >= top.val) push(1, 0, 1); else push(1, 1, 1); goto L0;}
	if (top.val >= pop(1).val) push(1, 0, 1); else push(1, 1, 1);
	goto L0;

L16:	//not.
	if (peek(1).type != 1) return 5;
	switch (pop(1).val) {
		case 0: push(1, 1, 1); break;
		case 1: push(1, 0, 1); break;
		default: return 3;
	}
	goto L0;

L17:	//and.
        top = pop(1);
	if (top.type < 1) return 5;
	if (peek(1).type < 1) return 5;
	push(1, pop(1).val & top.val, 1);
	goto L0;

L18:	//or.
        top = pop(1);
	if (top.type < 1) return 5;
	if (peek(1).type < 1) return 5;
	push(1, pop(1).val | top.val, 1);
	goto L0;

L19:	//print.
	switch (peek(1).type) {
		case 1: printf(\"%ld\", pop(1).val); break;
		case 2:	ffront = peek(1).val >> 16;
			fback = fixed2dec(peek(1).val - (ffront << 16));
			top.val = fback;
			printf(\"%ld.\", pop(1).val >> 16);
			while (top.val < 100000 && top.val != 0) {
				top.val *= 10;
				fputc('0', stdout);
			}
			printf(\"%ld\", fback);
			break;
		case 3: fputc(pop(1).val, stdout); break;
		default: return 4;
	}
	goto L0;

L20:	//scan.
	push(1, fgetc(stdin), 3);
	goto L0;

L21:	//write.
	if (peek(1).type != -1) return 6;
        top = pop(1);
	if (peek(1).type != -1) return 6;
        cbuf = malloc(size(top.val) * sizeof(char));
        stop = vars[top.val]->top;
        for (int i = 0; i < size(top.val); i++) {
            cbuf[i] = stop->val->val;
            stop = stop->next;
        }
        file = fopen(cbuf, \"w\");
        stop = vars[pop(1).val]->top;
        while (stop != NULL) {
            fputc((char)stop->val->val, file);
            stop = stop->next;
        }
        fclose(file);
        goto L0;

L22:	//read.
	if (peek(1).type != -1) return 6;
        top = pop(1);
	if (peek(1).type != -1) return 6;
        cbuf = malloc(size(top.val) * sizeof(char));
        stop = vars[top.val]->top;
        for (int i = 0; i < size(top.val); i++) {
            cbuf[i] = stop->val->val;
            stop = stop->next;
        }
        file = fopen(cbuf, \"r\");
        top.val = fgetc(file);
        while (!feof(file)) {
            push(peek(1).val, top.val, 3);
            top.val = fgetc(file);
        }
        fclose(file);
        goto L0;


L23:	//size.
	if (peek(1).type == -1) push(1, size(pop(1).val), 1);
	else return 5;
	goto L0;
	
L24:	//cast to int.
	switch (peek(1).type) {
		case 1: push(1, pop(1).val, 1); break;
		case 2: push(1, pop(1).val >> 16, 1); break;
		case 3: push(1, pop(1).val, 1); break;
		default: return 5;
	}
	goto L0;
	
L25:	//cast to fixed.
	switch (peek(1).type) {
		case 1: push(1, pop(1).val << 16, 2); break;
		case 2: push(1, pop(1).val, 2); break;
		case 3: push(1, pop(1).val << 16, 2); break;
		default: return 5;
	}
	goto L0;
	
L26:	//cast to char.
	switch (peek(1).type) {
		case 1: push(1, (char)(pop(1).val), 3); break;
		case 2: push(1, (char)(pop(1).val >> 16), 3); break;
		case 3: push(1, (char)(pop(1).val), 3); break;
		default: return 5;
	}
	goto L0;

L27:	//run stack.
	switch (peek(1).type) {
		case -2: push(0, pop(1).val, 0); break;
		default: return 5;
	}
	goto L0;
	
L28:    //system.
        if (peek(1).type != -1) return 6;
        top = pop(1);
        cbuf = malloc(size(top.val) * sizeof(char));
        stop = vars[top.val]->top;
        for (int i = 0; i < size(top.val); i++) {
            cbuf[i] = stop->val->val;
            stop = stop->next;
        }
        system(cbuf);
        goto L0;
	//End of boilerplate code. Of course, the number of cases are increased by how many blocks there are, same for variables.
");

    //add user-defined stacks.
    for stack in stack_map {

        c_code.push_str("\nL");
        c_code.push_str(&stack.0.to_string());
        c_code.push_str(":\n");

        for word in stack.1 {
            let mut line_of_code = String::from("\tpush(0, ");
            line_of_code.push_str(&word.0.to_string());
            line_of_code.push_str(", ");
            line_of_code.push_str(&word.1.to_string());
            line_of_code.push_str(");\n");
            c_code.push_str(&line_of_code);
        }
        c_code.push_str("\tgoto L0;\n");
    }

    c_code.push_str("}");
    return c_code;
}
