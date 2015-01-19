use std::collections::{BTreeMap, btree_map};

pub type Types = BTreeMap<usize, Vec<Type>>;

pub struct Type {
    pub name: String,
    pub elem: String,
    pub tot_bitsize: usize,
    pub count: usize,
    pub cfg: Option<String>,
}

const TYPES: &'static [(&'static [&'static str],
                        &'static [usize])] =
    &[(&["u","i"], &[8, 16, 32, 64]),
     (&["f"], &[32, 64])];

pub fn simd_types(log_max_count: usize) -> Types {
    let mut ret = BTreeMap::new();

    for log_count in 0..log_max_count + 1 {
        let count = 1 << log_count;
        for &(prefixes, widths) in TYPES.iter() {
            for &prefix in prefixes.iter() {
                for &width in widths.iter() {
                    let bitsize = count * width;
                    (match ret.entry(bitsize) {
                        btree_map::Entry::Occupied(o) => o.into_mut(),
                        btree_map::Entry::Vacant(v) => v.insert(vec![])
                    }).push(Type {
                        name: if count == 1 {
                            format!("{}{}", prefix, width)
                        } else {
                            format!("::simdty::{}{}x{}", prefix, width, count)
                        },
                        elem: format!("{}{}", prefix, width),
                        tot_bitsize: bitsize,
                        count: count,
                        cfg: None
                    });
                }
            }
        }
    }

    for &name in ["usize", "isize"].iter() {
        for &width in [32, 64].iter() {
            (match ret.entry(width) {
                btree_map::Entry::Occupied(o) => o.into_mut(),
                btree_map::Entry::Vacant(v) => v.insert(vec![])
            }).push(Type {
                name: name.to_string(),
                elem: name.to_string(),
                tot_bitsize: width,
                count: 1,
                cfg: Some(format!("target_pointer_width=\"{}\"", width))
            })
        }
    }

    ret
}
