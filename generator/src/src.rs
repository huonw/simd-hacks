use ty;

pub fn impl_header(trait_: &str, unsafe_: bool,
                   self_: &ty::Type, param: Option<&ty::Type>) -> String {

    let mut cfgs = vec![];
    if let Some(ref c) = self_.cfg { cfgs.push(&c[]) }

    let params = match param {
        None => String::new(),
        Some(t) => {
            if let Some(ref c) = t.cfg { cfgs.push(&c[]) }

            format!("<{}>", t.name)
        }
    };

    format!("\
{cfg}{unsafe_}impl {trait_}{params} for {self_} {{",
            cfg = if cfgs.is_empty() {"".to_string()} else {
                format!("#[cfg(all({}))] ", cfgs.connect(", "))
            },
            unsafe_ = if unsafe_ {"unsafe "} else {""},
            trait_ = trait_,
            params = params,
            self_ = self_.name)
}
