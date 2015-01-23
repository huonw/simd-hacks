use std::io::{self, File};
use src;
use ty;

pub fn convert_impls(tys: &ty::Types, dst: &Path) {
    io::fs::mkdir_recursive(&dst.join("convert_impls"), io::USER_RWX).unwrap();
    let mut out = File::create(&dst.join("convert_impls/mod.rs")).unwrap();

    writeln!(&mut out, "mod naive;").unwrap();
    let mut naive = File::create(&dst.join("convert_impls/naive.rs")).unwrap();
    writeln!(&mut naive, "#![cfg(feature = \"shims\")]").unwrap();

    writeln!(&mut out, "mod x86;").unwrap();
    let mut x86 = File::create(&dst.join("convert_impls/x86.rs")).unwrap();
    writeln!(&mut x86, "#![cfg(any(target_arch = \"x86\", target_arch = \"x86_64\"))]").unwrap();

    writeln!(&mut out, "mod arm;").unwrap();
    let mut arm = File::create(&dst.join("convert_impls/arm.rs")).unwrap();
    writeln!(&mut arm, "#![cfg(any(target_arch = \"arm\"))]").unwrap();

    let x86_special = special_cases! {
        2, i 32, 2, f 64, "sse2_cvtdq2pd", DoubleInput;
        2, f 64, 2, i 32, "sse2_cvttpd2dq", HalveOutput;
        2, f 32, 2, f 64, "sse2_cvtps2pd", DoubleInput;
        2, f 64, 2, f 32, "sse2_cvtpd2ps", HalveOutput;

        4, i 32, 2, f 64, "sse2_cvtdq2pd", None;
        2, f 64, 4, i 32, "sse2_cvttpd2dq", None;
        4, f 32, 2, f 64, "sse2_cvtps2pd", None;
        2, f 64, 4, f 32, "sse2_cvtpd2ps", None;

        4, i 32, 4, f 32, "sse2_cvtdq2ps", None;
        4, f 32, 4, i 32, "sse2_cvttps2dq", None;

        4, i 32, 4, f 64, "avx_cvtdq2_pd_256", None;
        4, f 64, 4, i 32, "avx_cvtt_pd2dq_256", None;
        4, f 64, 4, f 32, "avx_cvt_pd2_ps_256", None;
        4, f 32, 4, f 64, "avx_cvt_ps2_pd_256", None;

        8, i 32, 8, f 32, "avx_cvtdq2_ps_256", None;
        8, f 32, 8, i 32, "avx_cvtt_ps2dq_256", None;
    };

    let mut cfgs = vec![];
    for i in tys.all.iter() {
        for o in tys.all.iter() {
            let pair = (i.clone(), o.clone());
            cfgs.clear();
            if let Some(&(instr, promote)) = x86_special.get(&pair) {
                let c = src::x86_impl(&mut x86, "::Convert", "convert",
                                        i, o,
                                        &cfgs[],
                                        instr, promote).unwrap();
                cfgs.push(c);
            }

            if i.count == o.count {
                let writer = if i.count == 1 {
                    &mut out as &mut Writer
                } else {
                    &mut naive as &mut Writer
                };
                src::naive_impl(writer,"::Convert", "convert", i, o, &cfgs[], |w| {
                    write!(w, " self as {out} ", out = o.name)
                }).unwrap()
            }
        }
    }

}
