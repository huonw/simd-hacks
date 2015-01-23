use ty;
use std::io::IoResult;

#[derive(Copy)]
pub enum Promotion {
    None,
    DoubleInput,
    HalveOutput,
}


pub fn impl_header(w: &mut Writer,
                   trait_: &str, unsafe_: bool,
                   self_: &ty::Type, param: Option<&ty::Type>) -> IoResult<()> {

    let mut cfgs = vec![];
    if let Some(ref c) = self_.cfg { cfgs.push(&c[]) }

    let params = match param {
        None => String::new(),
        Some(t) => {
            if let Some(ref c) = t.cfg { cfgs.push(&c[]) }

            format!("<{}>", t.name)
        }
    };

    writeln!(w, "\
{cfg}{unsafe_}impl {trait_}{params} for {self_} {{",
            cfg = if cfgs.is_empty() {"".to_string()} else {
                format!("#[cfg(all({}))] ", cfgs.connect(", "))
            },
            unsafe_ = if unsafe_ {"unsafe "} else {""},
            trait_ = trait_,
            params = params,
            self_ = self_.name)
}

pub fn subdividing(w: &mut Writer, method: &str, out: &str) -> IoResult<()> {
    write!(w,
           "let (a, b) = ::HalfVector::split(self); \
            <<{out} as ::HalfVector>::Half as ::DoubleVector>::merge(a.{method}(), b.{method}())",
            method = method, out = out)
}

pub fn method<F>(w: &mut Writer, method: &str, out: &ty::Type, promote: Promotion,
                 body: F) -> IoResult<()>
    where F: FnOnce(&mut Writer, &str, &str) -> Option<IoResult<()>>
{
    let (input, output) = match promote {
        Promotion::None => ("self", ""),
        Promotion::DoubleInput
            => ("::DoubleVector::merge(self,::std::mem::uninitialized())", ""),
        Promotion::HalveOutput
            => ("self", "::HalfVector::lower"),
    };

    try!(write!(w, "    #[inline(always)] fn {method}(self) -> {out} {{",
             method = method, out = &out.name[]));
    match body(w, input, output) {
        Some(r) => try!(r),
        None => {
            try!(w.write_str(" "));
            try!(subdividing(w, method, &out.name[]));
            try!(w.write_str(" "));
        }
    }
    w.write_str("}\n")
}

pub fn naive_impl<F>(w: &mut Writer, trait_: &str, meth: &str, in_: &ty::Type, out: &ty::Type,
                     cfgs: &[String],
                     base_case: F) -> IoResult<()>
    where F: FnOnce(&mut Writer) -> IoResult<()> {
    assert!(in_.count == out.count);
    let count = in_.count;
    try!(writeln!(w, "#[cfg(not(any({cfg})))]", cfg = cfgs.connect(",")));
    try!(impl_header(w, trait_, true, in_, Some(out)));
    try!(method(w, meth, out, Promotion::None, move |w, _, _| {
        if count == 1 {
            Some(base_case(w))
        } else {
            None
        }
    }));
    w.write_str("}\n")

}

pub fn x86_impl(w: &mut Writer, trait_: &str, meth: &str,
                in_: &ty::Type, out: &ty::Type,
                cfgs: &[String],
                instr: &str, promote: Promotion) -> IoResult<String> {
    let name = &instr[..instr.bytes().position(|b| b == b'_').unwrap()];
    let x86_64 = match name {
        "sse" | "sse2" => "target_arch = \"x86_64\",",
        _ => ""
    };
    let cfg = format!("any({x86_64}feature=\"{name}\")", x86_64 = x86_64, name = name);

    try!(writeln!(w,"#[cfg(all(not(any({previous})), {cfg}))]",
             previous = cfgs.connect(", "), cfg = cfg));
    try!(impl_header(w, trait_, true, in_, Some(out)));
    try!(method(w, meth, out, promote, |w, input, output| {
        Some(write!(w, "\n        {output}(unsafe {{ ::llvmint::x86::{instr}({input}) }})\n    ",
                    input=input, output=output, instr=instr))
    }));
    try!(w.write_str("}\n"));

    Ok(cfg)
}
