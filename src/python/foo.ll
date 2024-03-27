; ModuleID = '<module>'
source_filename = "<module>"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 1, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 2, ptr %b, align 4
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %sum = add i32 %a1, %b2
  %c = alloca i32, align 4
  store i32 %sum, ptr %c, align 4
  %temp = alloca i32, align 4
  ret i32 0
}
