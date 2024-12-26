
//Boilerplate. Cannot modify.
#include <stdio.h>
#include <stdlib.h>
#define VAR_COUNT 7
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
		fprintf(stderr, "Popped too hard!\n");
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

	push(0, 0, 0);
	push(0, 0, 1);
	//Start of global code.
	//Reminder that variables start at 2.
	push(0, 0, 0);
	push(0, -1, 1);
	push(0, 19, 0);
	push(0, 9, 0);
	push(0, 28318, 2);
	push(0, 808714, 2);
	push(0, 19, 0);
	push(0, 10, 3);
	push(0, 27, 0);
	push(0, 3, 0);
	push(0, 4, -1);
	push(0, 5, 0);
	push(0, 3, -1);
	push(0, 6, -1);
	push(0, 19, 0);
	push(0, 32, 3);
	push(0, 27, 0);
	push(0, 3, 0);
	push(0, 2, -1);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 87, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 111, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 114, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 108, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 100, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 33, 3);
	push(0, 6, -1);
	push(0, 1, 0);
	push(0, 72, 3);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 101, 3);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 108, 3);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 108, 3);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 111, 3);
	push(0, 5, -1);
	push(0, 1, 0);
	push(0, 29, -2);
	push(0, 4, -1);
	push(0, 1, 0);
	push(0, 28, -2);
	push(0, 2, -1);

	//End of global code.
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
			case 28: goto L28; break;
			case 29: goto L29; break;
			default: return 1;
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
	if (peek(1).type == -1) return 5;
	top = pop(1);
	if (((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) || top.type == peek(1).type) {
		push(1, top.val + pop(1).val, top.type);
	} else if (top.type == 1 || top.type == 3) {
		push(1, (top.val << 16) + pop(1).val, top.type);
	} else {
		push(1, top.val + (pop(1).val << 16), top.type);
	}
	goto L0;

L8:	//sub.
	if (peek(1).type == -1) return 5;
	top = pop(1);
	if (((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) || top.type == peek(1).type) {
		push(1, pop(1).val - top.val, top.type);
	} else if (top.type == 1 || top.type == 3) {
		push(1, pop(1).val - (top.val << 16), top.type);
	} else {
		push(1, (pop(1).val << 16) - top.val, top.type);
	}
	goto L0;
	
L9:	//mpy.
	if (peek(1).type == -1) return 5;
	top = pop(1);
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
	if ((top.type == 1 || top.type == 3) && (peek(1).type == 1 || peek(1).type == 3)) {
		push(1, pop(1).val / top.val, top.type);
	} else if ((top.type == 1 || top.type == 3) || (peek(1).type == 1 || peek(1).type == 3)) {
		push(1, pop(1).val / top.val, 2);
	} else {
		push(1, (((long long)(pop(1).val) << 32) / top.val) >> 16, 2); //still very rough.
	}
	goto L0;

L11:	//if. no else.
	top = pop(1);
	if (top.type != -2) return 3;
	if (peek(1).type == -1) return 5;
	switch (pop(1).val) {
		case 0: break;
		case 1: push(0, top.val, 0); break;
		default: return 3;
	}
	goto L0;

L12:	//equal.
	if (peek(1).type == -1) return 5;
	if (pop(1).val == pop(1).val) push(1, 1, 1); else push(1, 0, 1);
	goto L0;

L13:	//unequal.
	if (peek(1).type == -1) return 5;
	if (pop(1).val != pop(1).val) push(1, 1, 1); else push(1, 0, 1);
	goto L0;

L14:	//less than. Reversed because of how C works...
	if (peek(1).type == -1) return 5;
	if (pop(1).val <= pop(1).val) push(1, 0, 1); else push(1, 1, 1);
	goto L0;

L15:	//more than.
	if (peek(1).type == -1) return 5;
	if (pop(1).val >= pop(1).val) push(1, 0, 1); else push(1, 1, 1);
	goto L0;

L16:	//not.
	if (peek(1).type == -1) return 5;
	switch (pop(1).val) {
		case 0: push(1, 1, 1); break;
		case 1: push(1, 0, 1); break;
		default: return 3;
	}
	goto L0;

L17:	//and.
	push(1, pop(1).val & pop(1).val, 1);
	goto L0;

L18:	//or.
	push(1, pop(1).val | pop(1).val, 1);
	goto L0;

L19:	//print.
	switch (peek(1).type) {
		case 1: printf("%ld", pop(1).val); break;
		case 2:	ffront = peek(1).val >> 16;
			fback = fixed2dec(peek(1).val - (ffront << 16));
			top.val = fback;
			printf("%ld.", pop(1).val >> 16);
			while (top.val < 100000 && top.val != 0) {
				top.val *= 10;
				fputc('0', stdout);
			}
			printf("%ld", fback);
			break;
		case 3: fputc(pop(1).val, stdout); break;
		default: return 4;
	}
	goto L0;

L20:	//scan.
	push(1, fgetc(stdin), 3);
	goto L0;

L21:	//write.
	if (peek(1).type != -1) return 5;
        top = pop(1);
	if (peek(1).type != -1) return 5;
        cbuf = malloc(size(top.val) * sizeof(char));
        stop = vars[top.val]->top;
        for (int i = 0; i < size(top.val); i++) {
            cbuf[i] = stop->val->val;
            stop = stop->next;
        }
        file = fopen(cbuf, "w");
        stop = vars[pop(1).val]->top;
        while (stop != NULL) {
            fputc((char)stop->val->val, file);
            stop = stop->next;
        }
        fclose(file);
        goto L0;

L22:	//read.
	if (peek(1).type != -1) return 5;
        top = pop(1);
	if (peek(1).type != -1) return 5;
        cbuf = malloc(size(top.val) * sizeof(char));
        stop = vars[top.val]->top;
        for (int i = 0; i < size(top.val); i++) {
            cbuf[i] = stop->val->val;
            stop = stop->next;
        }
        file = fopen(cbuf, "r");
        top.val = fgetc(file);
        while (top.val != 0) {
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
	
	//End of boilerplate code. Of course, the number of cases are increased by how many blocks there are, same for variables.

L28:
	push(0, 27, 0);
	push(0, 3, 0);
	push(0, 4, -1);
	push(0, 4, 0);
	push(0, 3, -1);
	goto L0;

L29:
	push(0, 11, 0);
	push(0, 3, 0);
	push(0, 4, -1);
	push(0, 15, 0);
	push(0, 0, 1);
	push(0, 23, 0);
	push(0, 3, -1);
	push(0, 19, 0);
	push(0, 2, 0);
	push(0, 3, -1);
	goto L0;
}