; ModuleID = 'Joy IR'
source_filename = "Joy IR"

define i32 @add(i32 %a, i32 %b) {
entry:
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  %b2 = alloca i32, align 4
  store i32 %b, ptr %b2, align 4
  %a3 = load i32, ptr %a1, align 4
  %b4 = load i32, ptr %b2, align 4
  %Int_Add = add i32 %a3, %b4
  ret i32 %Int_Add
}

define i32 @check_range(i32 %val) {
entry:
  %val1 = alloca i32, align 4
  store i32 %val, ptr %val1, align 4
  br label %cond.0

cond.0:                                           ; preds = %entry, %entry
  %val2 = load i32, ptr %val1, align 4
  %Int_Lesser = icmp slt i32 %val2, 10
  br i1 %Int_Lesser, label %body.0, label %cond.1

body.0:                                           ; preds = %cond.0
  ret i32 1
  br label %end.bb

cond.1:                                           ; preds = %cond.0
  %val3 = load i32, ptr %val1, align 4
  %Int_Lesser4 = icmp slt i32 %val3, 50
  br i1 %Int_Lesser4, label %body.1, label %cond.2

body.1:                                           ; preds = %cond.1
  ret i32 2

cond.2:                                           ; preds = %cond.1
  br label %body.2

body.2:                                           ; preds = %cond.2
  ret i32 3
  br label %end.bb

end.bb:                                           ; preds = %body.2, %body.0
  ret i32 4
}

define i32 @factorial(i32 %n) {
entry:
  %n1 = alloca i32, align 4
  store i32 %n, ptr %n1, align 4
  br label %cond.0

cond.0:                                           ; No predecessors!
  %n2 = load i32, ptr %n1, align 4
  %Int_LesEq = icmp sle i32 %n2, 1
  br i1 %Int_LesEq, label %body.0, label %end.bb

body.0:                                           ; preds = %cond.0
  ret i32 1
  br label %end.bb

end.bb:                                           ; preds = %body.0, %cond.0
  %n3 = load i32, ptr %n1, align 4
  %"function factorial" = call i32 @factorial()
  %Int_Mul = mul i32 %n3, %"function factorial"
  ret i32 %Int_Mul
}

define void @main() {
entry:
  %"function add" = call i32 @add(i32 15)
  %x = alloca i32, align 4
  store i32 %"function add", ptr %x, align 4
  %"function check_range" = call i32 @check_range()
  %status = alloca i32, align 4
  store i32 %"function check_range", ptr %status, align 4
  %"function factorial" = call i32 @factorial()
  %fact = alloca i32, align 4
  store i32 %"function factorial", ptr %fact, align 4
  %total = alloca i32, align 4
  store i32 0, ptr %total, align 4
  %i = alloca i64, align 8
  store i64 1, ptr %i, align 4
  br label %for.cond

for.cond:                                         ; preds = %for.step, %entry
  %i1 = load i64, ptr %i, align 4
  %is_loop_end = icmp slt i64 %i1, 10
  br i1 %is_loop_end, label %for.body, label %for.end

for.body:                                         ; preds = %for.cond
  br label %for.end
  br label %for.step

for.step:                                         ; preds = %for.body
  %next_ident_number = add i64 %i1, 1
  store i64 %next_ident_number, ptr %i, align 4
  br label %for.cond

for.end:                                          ; preds = %for.body, %for.cond
  ret void
}
