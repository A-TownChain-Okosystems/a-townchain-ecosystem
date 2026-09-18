//! Deterministic fork-choice boundary.
use crate::Block;pub fn choose<'a>(a:&'a Block,b:&'a Block)->&'a Block{if a.height>b.height{a}else if b.height>a.height{b}else if a.id<=b.id{a}else{b}}
