# Classp

The `javap` tool is a disassembler for Java class files. It allows you to view the structure of compiled Java classes,
including their methods, fields, and bytecode instructions.

https://docs.oracle.com/en/java/javase/24/docs/specs/man/javap.html

The `classp` tool has exactly the same functionality as `javap -v -p`. The `-v` option enables verbose output, and the
`-p` option includes private members in the output.
The purpose of `classp` is to simplify the debugging, testing and analysis of Java class files by using the `class_file`
library under the hood.
