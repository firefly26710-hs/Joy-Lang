; ModuleID = 'joyc'
source_filename = "joyc"

define void @main() {
entry:
  %x = alloca i32, align 4
  store i32 5, ptr %x, align 4
  %x1 = load i32, ptr %x, align 4
  %y = alloca i32, align 4
  store i32 %x1, ptr %y, align 4
  ret void
}
