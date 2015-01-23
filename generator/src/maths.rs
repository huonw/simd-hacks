use std::io::{self, File};
use ty;
use src;

pub fn maths_impls(types: &ty::Types, dst: &Path) {
    let dst = dst.join("maths_impls");
    io::fs::mkdir_recursive(&dst, io::USER_RWX).unwrap();
    let mut out = File::create(&dst.join("mod.rs")).unwrap();
    macro_rules! run {
        ($($name: ident),*) => { {
            $(
                $name(types, &dst);
                writeln!(&mut out, "mod {};", stringify!($name)).unwrap();
                )*
        } }
    }

    run!(sqrt_impls, rsqrt_impls)
}

fn sqrt_impls(types: &ty::Types, dst: &Path) {
    let mut out = File::create(&dst.join("sqrt_impls.rs")).unwrap();

    for ty in types.all.iter() {
        if !ty.elem.starts_with("f") {
            // only floating point types.
            continue
        }
        if ty.tot_bitsize >= 512 {
            // LLVM doesn't seem to handle large types correctly.
            continue
        }
        src::impl_header(&mut out, "::maths::sqrt::Sqrt", false, ty, None).unwrap();
        src::method(&mut out, "sqrt", ty, src::Promotion::None, |w, _, _| {
            Some(write!(w, "\n        \
        extern {{ #[link_name = \"llvm.sqrt.{llvm}\"] fn sqrt(x: {ty}) -> {ty}; }}
        unsafe {{sqrt(self)}}\n    ",
                   llvm = ty.llvm,
                   ty = ty.name))
        }).unwrap();
        out.write_str("}\n").unwrap();
    }
}
fn rsqrt_impls(types: &ty::Types, dst: &Path) {
    let dst = dst.join("rsqrt_impls");
    io::fs::mkdir_recursive(&dst, io::USER_RWX).unwrap();
    let mut out = File::create(&dst.join("mod.rs")).unwrap();

    writeln!(&mut out, "mod naive;").unwrap();
    let mut naive = File::create(&dst.join("naive.rs")).unwrap();
    writeln!(&mut naive, "#![cfg(feature = \"shims\")]").unwrap();

    writeln!(&mut out, "mod x86;").unwrap();
    let mut x86 = File::create(&dst.join("x86.rs")).unwrap();
    writeln!(&mut x86, "#![cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))]").unwrap();

    writeln!(&mut out, "mod arm;").unwrap();
    let mut arm = File::create(&dst.join("arm.rs")).unwrap();
    writeln!(&mut arm, "#![cfg(any(target_arch = \"arm\"))]").unwrap();

    let x86_special = special_cases! {
        4, f 32, 4, f 32, "sse_rsqrt_ps", None;
        8, f 32, 8, f 32, "avx_rsqrt_ps_256", None;
    };
    let mut cfgs = vec![];
    for ty in types.all.iter() {
        if !ty.elem.starts_with("f") {
            // only floating point types.
            continue
        }
        cfgs.clear();
        let pair = (ty.clone(), ty.clone());
        if let Some(&(instr, promote)) = x86_special.get(&pair) {
            let c = src::x86_impl(&mut x86, "::maths::sqrt::RSqrt", false, "rsqrt",
                                  ty, None,
                                  &cfgs[],
                                  instr, promote).unwrap();
            cfgs.push(c);
        }

        src::naive_impl(&mut naive, "::maths::sqrt::RSqrt", false, "rsqrt", ty, None, &cfgs[], |w| {
            write!(w, " 1.0 / ::maths::sqrt::Sqrt::sqrt(self)")
        }).unwrap();
    }
}
