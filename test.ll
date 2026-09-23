; ModuleID = 'joyc'
source_filename = "joyc"

define void @main() {
entry:
  br label %while.cond

while.cond:                                       ; preds = %while.body, %entry
  br i1 true, label %while.body, label %while.end

while.body:                                       ; preds = %while.cond
  br label %while.end
  br label %while.cond

while.end:                                        ; preds = %while.body, %while.cond
  ret void
}
