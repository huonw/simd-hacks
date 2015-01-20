use std::collections::{BTreeMap, btree_map};

pub struct Types {
    pub by_bitsize: BTreeMap<usize, Vec<Type>>,
    pub by_count: BTreeMap<usize, Vec<Type>>,
}

#[derive(Show, Eq, PartialEq, Clone, Hash)]
pub struct Type {
    pub name: String,
    pub elem: String,
    pub tot_bitsize: usize,
    pub count: usize,
    pub cfg: Option<String>,
}

impl Type {
    pub fn new(prefix: &str, width: usize, count: usize) -> Type {
        Type {
            name: if count == 1 {
                format!("{}{}", prefix, width)
            } else {
                format!("::simdty::{}{}x{}", prefix, width, count)
            },
            elem: format!("{}{}", prefix, width),
            tot_bitsize: count * width,
            count: count,
            cfg: None
        }
    }
}

const TYPES: &'static [(&'static [&'static str],
                        &'static [usize])] =
    &[(&["u","i"], &[8, 16, 32, 64]),
     (&["f"], &[32, 64])];

pub fn simd_types(log_max_count: usize) -> Types {
    let mut ret = Types {
        by_bitsize: BTreeMap::new(),
        by_count: BTreeMap::new(),
    };

    for log_count in 0..log_max_count + 1 {
        let count = 1 << log_count;
        for &(prefixes, widths) in TYPES.iter() {
            for &prefix in prefixes.iter() {
                for &width in widths.iter() {
                    let bitsize = count * width;
                    let ty = Type::new(prefix, width, count);
                    (match ret.by_bitsize.entry(bitsize) {
                        btree_map::Entry::Occupied(o) => o.into_mut(),
                        btree_map::Entry::Vacant(v) => v.insert(vec![])
                    }).push(ty.clone());
                    (match ret.by_count.entry(count) {
                        btree_map::Entry::Occupied(o) => o.into_mut(),
                        btree_map::Entry::Vacant(v) => v.insert(vec![])
                    }).push(ty);
                }
            }
        }
    }

    for &name in ["usize", "isize"].iter() {
        for &width in [32, 64].iter() {
            let ty = Type {
                name: name.to_string(),
                elem: name.to_string(),
                tot_bitsize: width,
                count: 1,
                cfg: Some(format!("target_pointer_width=\"{}\"", width))
            };
            (match ret.by_bitsize.entry(width) {
                btree_map::Entry::Occupied(o) => o.into_mut(),
                btree_map::Entry::Vacant(v) => v.insert(vec![])
            }).push(ty.clone());
            (match ret.by_count.entry(1) {
                btree_map::Entry::Occupied(o) => o.into_mut(),
                btree_map::Entry::Vacant(v) => v.insert(vec![])
            }).push(ty)
        }
    }

    ret
}
