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

    run!(sqrt_impls)
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
