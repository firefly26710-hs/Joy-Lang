; ModuleID = 'joyc'
source_filename = "joyc"

define void @main() {
entry:
  br label %while.cond
}

define void @add() {
while.cond:
  br i1 false, label %while.body, label %while.end

while.body:                                       ; preds = %while.cond
  %x = alloca i32, align 4
  store i32 5, ptr %x, align 4
  br label %while.cond

while.end:                                        ; preds = %while.cond
  %y = alloca i32, align 4
  store i32 7, ptr %y, align 4

entry:                                            ; No predecessors!
}
