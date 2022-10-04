if exists("b:current_syntax")
    finish
end

au BufRead, BufNewFile *.ch set filetype chal

syn keyword varTypes auto none i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str
syn keyword keywords fn return end if elif else while for
syn region return start='->' end=':' contains=varTypes

syn match comment "#.*$"
syn region string start='"' end='"'

syn match number '\d\+'
syn match number '[-+]\d\+'
syn match number '\d\+\.\d*'
syn match number '[-+]\d\+\.\d*'

let b:current_syntax = "chal"
"hi def link retType  Todo
hi def link varTypes Type
hi def link keywords Statement
hi def link number   Constant
hi def link string   Constant
hi def link comment  Comment
hi def link return   MoreMsg
