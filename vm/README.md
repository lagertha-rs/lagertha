# vm

## Description

It is responsible for launching the `runtime` crate, which implements the Java Virtual Machine. It handles command-line
arguments,
sets up the classpath, and initializes the runtime environment.

## Usage

To run the VM, use the following command:

```bash
cargo run --bin vm -- [options] <class>
```

- `<class>`: Right now, it only supports running a single class file. The class name should be only the file name,
  without package.
- `[options]`: Options for the VM. Currently, it supports:
    - `-cp <path>` or `--classpath <path>`: Specifies the classpath to search for class files.

## Example

Assuming the tree structure is as follows:

```
.
├── com
│   └── example
│       └── HelloWorldMain.class
|       └── OtherClass.class
└── vm
```

To run the `HelloWorldMain` class, use the following command:

```bash
cargo run --bin vm -- -cp . com/example/HelloWorldMain.class
```

