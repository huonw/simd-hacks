#![allow(unstable)]
use std::io::File;
use std::os;

mod ty;
mod src;
mod conversions;

fn vector_impls(ty: &ty::Types, dst: &Path) {
    let mut out = File::create(&dst.join("vector_impls.rs")).unwrap();
    for (_width, types) in ty.by_bitsize.iter() {
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

    for (_width, types) in ty.by_bitsize.iter() {
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

fn half_double_impls(ty: &ty::Types, dst: &Path) {
    let mut out = File::create(&dst.join("half_double_impls.rs")).unwrap();
    let max = *ty.by_count.keys().max().unwrap();
    for (&count, types) in ty.by_count.iter() {
        for ty in types.iter() {
            if count > 1 {
                let o = ty::Type::new(&ty.elem[..1], ty.width, ty.count / 2);
                writeln!(&mut out, "\
{header}
    type Half = {out};
    #[inline(always)] fn split(self) -> ({out}, {out}) {{ unsafe {{ ::std::mem::transmute(self) }} }}
}}",
                         header = src::impl_header("::HalfVector", true, ty, None),
                         out = o.name).unwrap();
            }
            if count < max {
                let o = ty::Type::new(&ty.elem[..1], ty.width, ty.count * 2);
                writeln!(&mut out, "\
{header}
    type Double = {out};
    #[inline(always)] fn merge(self, other: {in_}) -> {out} {{ unsafe {{ ::std::mem::transmute((self, other)) }} }}
}}",
                         header = src::impl_header("::DoubleVector", true, ty, None),
                         in_ = ty.name,
                         out = o.name).unwrap();
            }

        }
    }
}


fn main() {
    let dst = Path::new(&os::args()[1]);

    // 64 == 1<<6
    let log_max_count = 6;
    let types = ty::simd_types(log_max_count);

    let mut index = File::create(&dst.join("mod.rs"));

    macro_rules! run {
        ($($name: ident),*) => { {
            $(
                $name(&types, &dst);
                writeln!(&mut index, "mod {};", stringify!($name)).unwrap();
                )*
        } }
    }

    {
        use conversions::convert_impls;
        run!(vector_impls, bitcast_impls, convert_impls, half_double_impls);
    }
}
