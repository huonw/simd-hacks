use ty;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
pub struct Promotion {
    input_doubles: usize,
    output_halves: usize,
}
impl Promotion {
    pub fn new(id: usize, oh: usize) -> Promotion {
        Promotion { input_doubles: id, output_halves: oh }
    }
}


pub fn impl_header(w: &mut Write,
                   trait_: &str, unsafe_: bool,
                   self_: &ty::Type, param: Option<&ty::Type>) -> io::Result<()> {

    let mut cfgs = vec![];
    if let Some(ref c) = self_.cfg { cfgs.push(c) }

    let params = match param {
        None => String::new(),
        Some(t) => {
            if let Some(ref c) = t.cfg { cfgs.push(&c) }

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

pub fn subdividing(w: &mut Write, trait_: &str, method: &str, out: &str) -> io::Result<()> {
    write!(w,
           "let (a, b) = ::HalfVector::split(in_); \
            <<{out} as ::HalfVector>::Half as ::DoubleVector>::merge(\
            {trait_}::{method}(a), {trait_}::{method}(b))",
            trait_ = trait_, method = method, out = out)
}

pub fn method<F>(w: &mut Write, trait_: &str, method: &str, out: &ty::Type, promote: Promotion,
                 body: F) -> io::Result<()>
    where F: FnOnce(&mut Write) -> Option<io::Result<()>>
{
    let mut input = "self".to_string();
    let mut i_left = "";
    let mut i_right = "";
    let mut output = "out".to_string();

    for _ in 0..promote.input_doubles {
        input = format!("::DoubleVector::merge({}, ::std::mem::uninitialized())", input);
        i_left = "unsafe {";
        i_right = "}"
    }
    for _ in 0..promote.output_halves {
        output = format!("::HalfVector::lower({})", output);
    }

    try!(write!(w, "    #[inline(always)] fn {method}(self) -> {out} {{
        let in_ = {i_left}{input}{i_right}; let out = {{",
             method = method, out = &out.name,
                input = input, i_left = i_left, i_right = i_right));
    match body(w) {
        Some(r) => try!(r),
        None => {
            try!(w.write_all(b" "));
            try!(subdividing(w, trait_, method, &out.name));
            try!(w.write_all(b" "));
        }
    }
    writeln!(w, "}}; {output} }}", output = output)
}

pub fn naive_impl<'a, F>(w: &mut Write, trait_: &str, unsafe_: bool, meth: &str, in_: &'a ty::Type,
                         out: Option<&'a ty::Type>,
                         cfgs: &[String],
                         base_case: F) -> io::Result<()>
    where F: FnOnce(&mut Write) -> io::Result<()>
{
    assert!(out.map_or(true, |o| in_.count == o.count));
    let count = in_.count;
    try!(writeln!(w, "#[cfg(not(any({cfg})))]", cfg = cfgs.connect(",")));
    try!(impl_header(w, trait_, unsafe_, in_, out));
    try!(method(w, trait_, meth, out.unwrap_or(in_), Promotion::new(0, 0), move |w| {
        if count == 1 {
            Some(base_case(w))
        } else {
            None
        }
    }));
    w.write_all(b"}\n")

}

pub fn x86_impl<'a>(w: &mut Write, trait_: &str, unsafe_: bool, meth: &str,
                    in_: &'a ty::Type, out: Option<&'a ty::Type>,
                    cfgs: &[String],
                    instr: &str, promote: Promotion) -> io::Result<String> {
    let name = &instr[..instr.bytes().position(|b| b == b'_').unwrap()];
    let x86_64 = match name {
        "sse" | "sse2" => "target_arch = \"x86_64\",",
        _ => ""
    };
    let cfg = format!("any({x86_64}feature=\"{name}\")", x86_64 = x86_64, name = name);

    try!(writeln!(w,"#[cfg(all(not(any({previous})), {cfg}))]",
             previous = cfgs.connect(", "), cfg = cfg));
    try!(impl_header(w, trait_, unsafe_, in_, out));
    try!(method(w, trait_, meth, out.unwrap_or(in_), promote, |w| {
        Some(write!(w, "unsafe{{::llvmint::x86::{instr}(in_)}}", instr=instr))
    }));
    try!(w.write_all(b"}\n"));

    Ok(cfg)
}
