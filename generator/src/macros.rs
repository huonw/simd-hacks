macro_rules! special_cases {
    ($smallest: expr,
     $($icount: expr, $in_: ident $iwidth: expr,
       $ocount: expr, $out: ident $owidth: expr,
       $instr: expr,
       /* input doubles */ $id: expr, /* output halves */ $oh: expr,
       /* should I use this instruction for smaller types? */ $iterate: expr);*;) => {{
        let mut map = ::std::collections::HashMap::new();
         let smallest = $smallest;
        $({
            let mut icount = $icount;
            let mut ocount = $ocount;
            let mut id = $id;
            let mut oh = $oh;
            while icount >= smallest && ocount >= $smallest {
                (match map.entry((::ty::Type::new(stringify!($in_), $iwidth, icount),
                                  ::ty::Type::new(stringify!($out), $owidth, ocount))) {
                    ::std::collections::hash_map::Entry::Occupied(o) => o.into_mut(),
                    ::std::collections::hash_map::Entry::Vacant(v) => v.insert(vec![]),
                }).push(($instr, ::src::Promotion::new(id, oh)));
                icount /= 2;
                ocount /= 2;
                // the width of the input/output vectors is smaller,
                // so we better double/half more times.
                id += 1;
                oh += 1;
                if !$iterate { break }
            }
            })*
            map
    }}
}
