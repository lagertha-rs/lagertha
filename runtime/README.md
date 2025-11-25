# VM Runtime

## Notes

- It is planned to support mutli-threading in the future, that's why there are `Arc` usages,
  even though currently the runtime is single-threaded. I use it where I'm more or less sure that
  the data will be shared between threads in the future and this part won't need to be completely rewritten.

## Loading and Running a Program

[Chapter 5. Loading, Linking, and Initializing](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html)
