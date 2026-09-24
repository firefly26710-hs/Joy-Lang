; ModuleID = 'joyc'
source_filename = "joyc"

define i32 @add(i32 %a, i32 %b) {
entry:
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  %b2 = alloca i32, align 4
  store i32 %b, ptr %b2, align 4
  %a3 = load i32, ptr %a1, align 4
  %b4 = load i32, ptr %b2, align 4
  %INT_ADD = add i32 %a3, %b4
  ret i32 %INT_ADD
}

define void @main() {
entry:
  %func_add = call i32 @add(i32 6, i32 7)
  %x = alloca i32, align 4
  store i32 %func_add, ptr %x, align 4
  ret void
}
