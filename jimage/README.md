# JImage

JImage is a file format introduced in Java 9 to store Java class files and resources in a more efficient way than
traditional JAR files. It is designed to improve the performance of Java applications by reducing startup time and
memory footprint.

The JImage is internal format used to package the Java runtime itself, and it is not intended for general use by
developers. However, it is necessary to read the JImage files, to access the classes and resources contained within the
Java runtime. The JImage files are typically located in the `${JAVA_HOME}/lib` directory of the Java installation.

## Progress

As of now, there is no official documentation available for the JImage file format. However, with resources available
online and AI tools, it is possible to reverse-engineer the format and create a reader for it.

Right now it is possible to read all classes from the `java.base` module, but other modules are not yet supported.
