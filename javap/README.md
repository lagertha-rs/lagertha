# javap

## Description

The `javap` tool is a disassembler for Java class files. It allows you to view the structure of compiled Java classes,
including their methods, fields, and bytecode instructions.

https://docs.oracle.com/en/java/javase/24/docs/specs/man/javap.html

This `javap` tool has exactly the same functionality as `javap -v -p`. The `-v` option enables verbose output, and the
`-p` option includes private members in the output.
The purpose of `javap` is to simplify the debugging, testing and analysis of Java class files by using the `jclass`
library under the hood.

## Usage

To use the `javap` tool, run the following command:

```bash
cargo run --bin javap -- <class file>
```

- `<class file>`: The path to the Java class file you want to inspect.

## Example

Assuming you have a Java class file named `HelloWorldMain.class`, with the source code:

```java
package com.example;

public class HelloWorldMain {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```

You can use the `javap` tool to inspect its contents:

```bash
cargo run --bin javap -- path/to/HelloWorldMain.class
```

This will output the detailed structure of the `HelloWorldMain` class, including its fields, methods, and bytecode
instructions. Like this:

```
public class com.example.HelloWorldMain 
  minor version: 0
  major version: 68
  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
  this_class: #21                      //com/example/HelloWorldMain
  super_class: #2                      //java/lang/Object
  interfaces: 0, fields: 0, methods: 2, attributes: 1
Constant pool:
   #1 = Methodref          #2.#3            // java/lang/Object."<init>":()V
   #2 = Class              #4               // java/lang/Object
   #3 = NameAndType        #5:#6            // "<init>":()V
   #4 = Utf8               java/lang/Object
   #5 = Utf8               <init>
   #6 = Utf8               ()V
   #7 = Fieldref           #8.#9            // java/lang/System.out:Ljava/io/PrintStream;
   #8 = Class              #10              // java/lang/System
   #9 = NameAndType        #11:#12          // out:Ljava/io/PrintStream;
  #10 = Utf8               java/lang/System
  #11 = Utf8               out
  #12 = Utf8               Ljava/io/PrintStream;
  #13 = String             #14              // Hello, World!
  #14 = Utf8               Hello, World!
  #15 = Methodref          #16.#17          // java/io/PrintStream.println:(Ljava/lang/String;)V
  #16 = Class              #18              // java/io/PrintStream
  #17 = NameAndType        #19:#20          // println:(Ljava/lang/String;)V
  #18 = Utf8               java/io/PrintStream
  #19 = Utf8               println
  #20 = Utf8               (Ljava/lang/String;)V
  #21 = Class              #22              // com/example/HelloWorldMain
  #22 = Utf8               com/example/HelloWorldMain
  #23 = Utf8               Code
  #24 = Utf8               LineNumberTable
  #25 = Utf8               main
  #26 = Utf8               ([Ljava/lang/String;)V
  #27 = Utf8               SourceFile
  #28 = Utf8               HelloWorldMain.java
{
  public com.example.HelloWorldMain();
    descriptor: ()V
    flags: (0x0001) ACC_PUBLIC
    Code: 
      stack=1, locals=1, args_size=1
         0: aload_0                 
         1: invokespecial #1                  // Method java/lang/Object."<init>":()V
         4: return                  
      LineNumberTable:
        line 3: 0
  
  public static void main(java.lang.String[]);
    descriptor: ([Ljava/lang/String;)V
    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    Code: 
      stack=2, locals=1, args_size=1
         0: getstatic     #7                  // Field java/lang/System.out:Ljava/io/PrintStream;
         3: ldc           #13                 // String Hello, World!
         5: invokevirtual #15                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
         8: return                  
      LineNumberTable:
        line 5: 0
        line 6: 8
}
SourceFile: "HelloWorldMain.java"
```
