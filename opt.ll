; ModuleID = 'output.ll'
source_filename = "Joy IR"

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define i32 @add(i32 %a, i32 %b) local_unnamed_addr #0 {
entry:
  %Int_Greater = icmp sgt i32 %a, %b
  %Int_Add = add i32 %b, %a
  %Int_EqEq = icmp eq i32 %a, %b
  %Int_Mul = mul i32 %b, %a
  %spec.select = select i1 %Int_EqEq, i32 0, i32 %Int_Mul
  %common.ret.op = select i1 %Int_Greater, i32 %Int_Add, i32 %spec.select
  ret i32 %common.ret.op
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none)
define void @main() local_unnamed_addr #0 {
entry:
  ret void
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) }
