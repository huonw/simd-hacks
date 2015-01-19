#![allow(unstable)]
use std::io::File;
use std::os;

mod ty;
mod src;

fn vector_impls(ty: &ty::Types, dst: &Path) {
    let mut out = File::create(&dst.join("vector_impls.rs")).unwrap();
    for (_width, types) in ty.iter() {
        for ty in types.iter() {
            writeln!(&mut out, "\
{header}
    type Item = {elem}; #[inline(always)] fn count(_: Option<Self>) -> usize {{ {count} }}
}}",
                     header = src::impl_header("::Vector", true, ty, None),
                     elem = ty.elem,
                     count = ty.count).unwrap();
        }
    }
}

fn bitcast_impls(ty: &ty::Types, dst: &Path) {
    let mut out = File::create(&dst.join("bitcast_impls.rs")).unwrap();

    for (_width, types) in ty.iter() {
        for i in types.iter() {
            for o in types.iter() {
                writeln!(&mut out, "\
{header}
    #[inline(always)] fn bitcast(self) -> {out} {{ unsafe {{ ::std::mem::transmute(self) }} }}
}}",
                         header = src::impl_header("::Bitcast", true, i, Some(o)),
                         out = o.name).unwrap();
            }
        }
    }
}


fn main() {
    let dst = Path::new(&os::args()[1]);

    // 64 == 1<<6
    let types = ty::simd_types(6);

    let mut index = File::create(&dst.join("mod.rs"));

    macro_rules! run {
        ($($name: ident),*) => { {
            $(
                $name(&types, &dst);
                writeln!(&mut index, "mod {};", stringify!($name)).unwrap();
                )*
        } }
    }

    run!(vector_impls, bitcast_impls);
}
