macro_rules! special_cases {
    ($($icount: expr, $in_: ident $iwidth: expr,
       $ocount: expr, $out: ident $owidth: expr,
       $instr: expr, $id: expr, $oh: expr);*;) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert((::ty::Type::new(stringify!($in_), $iwidth, $icount),
                        ::ty::Type::new(stringify!($out), $owidth, $ocount)),
                       ($instr, ::src::Promotion::new($id, $oh)));
            )*
            map
    }}
}
