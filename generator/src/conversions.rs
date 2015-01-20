use std::collections::HashMap;
use std::io::{self, File};
use src;
use ty;
use ty::Type;

fn convert_naive(w: &mut Writer, in_: &ty::Type, out: &ty::Type, cfgs: &[String]) {
    assert!(in_.count == out.count);
    let count = in_.count;
    let header = src::impl_header("::Convert", true, in_, Some(out));
    write!(w, "\
#[cfg(not(any({cfg})))]
{header}
    #[inline(always)] fn convert(self) -> {out} {{ ",
           cfg = cfgs.connect(","), header = header, out = out.name).unwrap();

    if count == 1 {
        write!(w, "self as {out}", out = out.name).unwrap();
    } else {
        write!(w,
               "let (a, b) = ::HalfVector::split(self); \
               <<{out} as ::HalfVector>::Half as ::DoubleVector>::merge(a.convert(), b.convert())",
               out = out.name).unwrap();
    }
    writeln!(w," }}\n}}").unwrap();

}

fn convert_x86(w: &mut Writer, in_: &ty::Type, out: &ty::Type, instr: &str) -> String {
    let name = &instr[..instr.bytes().position(|b| b == b'_').unwrap()];
    let x86_64 = match name {
        "sse" => "target_arch = \"x86_64\",",
        "sse2" => "target_arch = \"x86_64\",",
        _ => ""
    };
    let cfg = format!("any({x86_64}feature=\"{name}\")", x86_64 = x86_64, name = name);
    writeln!(w,"\
#[cfg({cfg})]
{header}
    #[inline(always)] fn convert(self) -> {out} {{ unsafe {{ ::llvmint::x86::{instr}(self) }} }}
}}",
             cfg = cfg,
             header = src::impl_header("::Convert", true, in_, Some(out)),
             out = out.name,
             instr = instr).unwrap();

    cfg
}

macro_rules! special_cases {
    ($($count: expr, $in_: ident $iwidth: expr, $out: ident $owidth: expr, $instr: expr);*;) => {{
        let mut map = HashMap::new();
        $(
            map.insert((Type::new(stringify!($in_), $iwidth, $count),
                        Type::new(stringify!($out), $owidth, $count)),
                       $instr);
            )*
            map
    }}
}
pub fn convert_impls(tys: &ty::Types, dst: &Path) {
    io::fs::mkdir_recursive(&dst.join("convert_impls"), io::USER_RWX).unwrap();
    let mut out = File::create(&dst.join("convert_impls/mod.rs")).unwrap();

    writeln!(&mut out, "mod naive;").unwrap();
    let mut naive = File::create(&dst.join("convert_impls/naive.rs")).unwrap();
    writeln!(&mut naive, "#![cfg(not(feature = \"no_naive\"))]").unwrap();

    writeln!(&mut out, "mod x86;").unwrap();
    let mut x86 = File::create(&dst.join("convert_impls/x86.rs")).unwrap();
    writeln!(&mut x86, "#![cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))]").unwrap();

    writeln!(&mut out, "mod arm;").unwrap();
    let mut arm = File::create(&dst.join("convert_impls/arm.rs")).unwrap();
    writeln!(&mut arm, "#![cfg(any(target_arch = \"arm\"))]").unwrap();

    let x86_special = special_cases! {
        4, i 32, f 32, "sse2_cvtdq2ps";
        4, f 32, i 32, "sse2_cvtps2dq";

        4, i 32, f 64, "avx_cvtdq2_pd_256";
        4, f 64, i 32, "avx_cvt_pd2dq_256";
        4, f 64, f 32, "avx_cvt_pd2_ps_256";
        4, f 32, f 64, "avx_cvt_ps2_pd_256";

        8, i 32, f 32, "avx_cvtdq2_ps_256";
        8, f 32, i 32, "avx_cvt_ps2dq_256";
    };

    let mut cfgs = vec![];
    for (count, types) in tys.by_count.iter() {
        for i in types.iter() {
            for o in types.iter() {
                let pair = (i.clone(), o.clone());
                cfgs.clear();
                if let Some(&instr) = x86_special.get(&pair) {
                    cfgs.push(convert_x86(&mut x86, i, o, instr));
                }

                if *count == 1 {
                    convert_naive(&mut out, i, o, &cfgs[]);
                } else {
                    convert_naive(&mut naive, i, o, &cfgs[]);
                }
            }
        }
    }
}
