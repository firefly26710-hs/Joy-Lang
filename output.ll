; ModuleID = 'joyc'
source_filename = "joyc"

define i32 @fib(i32 %n) {
entry:
  %n1 = alloca i32, align 4
  store i32 %n, ptr %n1, align 4
  br label %cond.0

cond.0:                                           ; preds = %entry
  %n2 = load i32, ptr %n1, align 4
  %INT_LESSER = icmp slt i32 %n2, 2
  br i1 %INT_LESSER, label %body.0, label %end.bb

body.0:                                           ; preds = %cond.0
  ret i32 1
  br label %end.bb

end.bb:                                           ; preds = %body.0, %cond.0
  %n3 = load i32, ptr %n1, align 4
  %INT_SUB = sub i32 %n3, 1
  %func_fib = call i32 @fib(i32 %INT_SUB)
  %n4 = load i32, ptr %n1, align 4
  %INT_SUB5 = sub i32 %n4, 2
  %func_fib6 = call i32 @fib(i32 %INT_SUB5)
  %INT_ADD = add i32 %func_fib, %func_fib6
  ret i32 %INT_ADD
}

define void @main() {
entry:
  %n = alloca i32, align 4
  store i32 5, ptr %n, align 4
  %n1 = load i32, ptr %n, align 4
  %func_fib = call i32 @fib(i32 %n1)
  %sum = alloca i32, align 4
  store i32 %func_fib, ptr %sum, align 4
  ret void
}
