use std::intrinsics::assigner_sha2_256;

use ark_pallas::Fq;

type Block = [Fq; 2];

fn sha2_256(block_1: Block, block_2: Block) -> Block {
    let hash_ = assigner_sha2_256([block_1[0].0, block_1[1].0], [block_2[0].0, block_2[1].0]);
    [hash_[0].into(), hash_[1].into()]
}

#[circuit]
fn validate_path(merkle_path: [Block; 5], leave: Block, root: Block) -> bool {
    merkle_path
        .into_iter()
        .fold(leave, |subroot, block| sha2_256(subroot, block))
        == root
}

#[cfg(not(target_arch = "assigner"))]
fn main() {
    println!("This is zkLLVM Rust template");
}
