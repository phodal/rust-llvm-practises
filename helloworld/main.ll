; ModuleID = 'repl'
source_filename = "repl"

@hello = internal global [14 x i8] c"hello, world!\0A"

define i64 @main(i64 %0, i64 %1, i64 %2) {
entrypoint:
  %3 = call i32 (i8*, ...) @puts(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @hello, i32 0, i32 0))
  ret i32 0
}

declare i32 @puts(i8*, ...)
