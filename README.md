# PostStacked
### A language written by a madman for madmen.
I don't know how to write markdown so I'll just use Discord syntax.

___

## Key Points

* This language is designed around the concept of stacks.
* No local variables.
* The language is designed to contain as little parentheses/brackets/quotes as possible but I got lazy and gave up midway. Anything inspired by this is deeply appreciated.
* Reading the source code and the given example may be more helpful than the reference below because I'm *really* bad at explaining things.
* The compiler errors aren't very helpful.

## Usage

1. ` ./ptsdc input.ptsd [-o output] ` for the executable. It will come out as a C file.
2. ` bash ptsdc.sh input.ptsd output ` for the provided short shell script. It will come out an executable. (I don't understand std::process at all...)
3. Note that if this is compiled with clang it will throw a Segmentation Fault error. The memory management is bad.


## Reference

### Syntax

1. Uses postfix notation, meaning ` 1 + 2 * 3` would be written as ` 1 2 3 * + `.
2. Space-seperated, meaning ` 1 2 3*+ ` is not allowed. (Except for curly brackets)
3. Does not care for line breaks or indentations.

### Variables

1. All variables are stacks, able to contain one of four value types:
    * integers (INT)
    * fixed-point numbers (FIXED)
    * characters (CHAR)
    * stacks (surrounded by curly brackets)

2. Stacks are blocks of code that get appended on the instruction stack to be executed. They can be executed by running the RUN operation on them.
EX: A stack that locks your program in a never-ending loop.
```
loop{loop
peek run
}
push
```

### Operations
Note that they can be called in either UPPERLINE or underline form.

#### Stack-related operations.
1. PUSH Pushes a value into a variable. ` VAR value PUSH `
2. POP Pops a value from a variable. ` VAR POP `
3. PEEK Peeks at the top value of a variable. ` VAR PEEK `
4. RENAME Renames a variable, overriding the destination. ` OLD NEW RENAME `
5. CLONE Clones a variable, overriding the destination. ` ORIG CLON RENAME `
6. FLUSH Flushes a variable, leaving it at size 0 and without values. ` VAR FLUSH `

#### Maths-related operations.
7. ADD / +
8. SUB / -
9. MPY / *
10. DIV / /

#### Boolean-related operations.
11. IF ` boolean (0 or 1) stack IF `
12. EQUAL / =
13. UNEQUAL / !=
14. LESS / <
15. MORE / >
16. NOT / !
17. AND / &
18. OR / |

#### I/O, note that scan can only scan characters.
19. PRINT ` value PRINT `
20. SCAN ` SCAN `
21. WRITE Writes to file. ` (stack with file name) (stack with content) WRITE/READ `
22. READ Reads from file. (Note that the last character will be on top.)

23. SIZE ` VAR SIZE `

#### type casts.
24. INT
25. FIXED
26. CHAR

27. RUN ` stack RUN `


## Personal Remarks (Not Important, Extremely Egotistical.)

1. The suffix is intentional. Originally it was to be called "PostStack" but then I couldn't think of a good suffix, so I added an "ed".
2. This program was written in part because of my fascination in the C goto statement.
3. If I feel like it, I might extend this later.
