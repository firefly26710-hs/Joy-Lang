; ModuleID = 'Joy IR'
source_filename = "Joy IR"

define i32 @add(i32 %a, i32 %b) {
entry:
  %a1 = alloca i32, align 4
  store i32 %a, ptr %a1, align 4
  %b2 = alloca i32, align 4
  store i32 %b, ptr %b2, align 4
  br label %cond.0

cond.0:                                           ; preds = %entry
  %a3 = load i32, ptr %a1, align 4
  %b4 = load i32, ptr %b2, align 4
  %Int_Greater = icmp sgt i32 %a3, %b4
  br i1 %Int_Greater, label %body.0, label %cond.1

body.0:                                           ; preds = %cond.0
  %a5 = load i32, ptr %a1, align 4
  %b6 = load i32, ptr %b2, align 4
  %Int_Add = add i32 %a5, %b6
  ret i32 %Int_Add
  br label %end.bb

cond.1:                                           ; preds = %cond.0
  %a7 = load i32, ptr %a1, align 4
  %b8 = load i32, ptr %b2, align 4
  %Int_EqEq = icmp eq i32 %a7, %b8
  br i1 %Int_EqEq, label %body.1, label %cond.2

body.1:                                           ; preds = %cond.1
  %a9 = load i32, ptr %a1, align 4
  %b10 = load i32, ptr %b2, align 4
  %Int_Sub = sub i32 %a9, %b10
  ret i32 %Int_Sub

cond.2:                                           ; preds = %cond.1
  br label %body.2

body.2:                                           ; preds = %cond.2
  %a11 = load i32, ptr %a1, align 4
  %b12 = load i32, ptr %b2, align 4
  %Int_Mul = mul i32 %a11, %b12
  ret i32 %Int_Mul
  br label %end.bb

end.bb:                                           ; preds = %body.2, %body.0
  %a13 = load i32, ptr %a1, align 4
  %b14 = load i32, ptr %b2, align 4
  %Int_Div = sdiv i32 %a13, %b14
  ret i32 %Int_Div
}

define void @main() {
entry:
  %"function add" = call i32 @add(i32 6, i32 7)
  %c = alloca i32, align 4
  store i32 %"function add", ptr %c, align 4
  ret void
}
