; ModuleID = 'joyc'
source_filename = "joyc"

define i32 @fib(i32 %n) {
entry:
  %n1 = alloca i32, align 4
  store i32 %n, ptr %n1, align 4
}

define void @main() {
entry:
}
