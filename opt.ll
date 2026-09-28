; ModuleID = 'output.ll'
source_filename = "joyc"

; Function Attrs: nofree nosync nounwind memory(none)
define i32 @fib(i32 %n) local_unnamed_addr #0 {
entry:
  %INT_LESSER3 = icmp slt i32 %n, 2
  br i1 %INT_LESSER3, label %common.ret, label %end.bb

common.ret.loopexit:                              ; preds = %end.bb
  %0 = add i32 %INT_ADD, 1
  br label %common.ret

common.ret:                                       ; preds = %common.ret.loopexit, %entry
  %accumulator.tr.lcssa = phi i32 [ 1, %entry ], [ %0, %common.ret.loopexit ]
  ret i32 %accumulator.tr.lcssa

end.bb:                                           ; preds = %entry, %end.bb
  %n.tr5 = phi i32 [ %INT_SUB5, %end.bb ], [ %n, %entry ]
  %accumulator.tr4 = phi i32 [ %INT_ADD, %end.bb ], [ 0, %entry ]
  %INT_SUB = add nsw i32 %n.tr5, -1
  %func_fib = tail call i32 @fib(i32 %INT_SUB)
  %INT_SUB5 = add nsw i32 %n.tr5, -2
  %INT_ADD = add i32 %func_fib, %accumulator.tr4
  %INT_LESSER = icmp samesign ult i32 %n.tr5, 4
  br i1 %INT_LESSER, label %common.ret.loopexit, label %end.bb
}

; Function Attrs: nofree nosync nounwind memory(none)
define void @main() local_unnamed_addr #0 {
entry:
  %func_fib = tail call i32 @fib(i32 5)
  ret void
}

attributes #0 = { nofree nosync nounwind memory(none) }
