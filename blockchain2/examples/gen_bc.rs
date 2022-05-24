use blockchain2::blocks::blockchain::Blockchain;

// RUST_LOG=info cargo run --example gen_bc --quiet
fn main() {
    tracing_subscriber::fmt().init();

    let mut bc = Blockchain::new();

    bc.mine_block("Justin -> Bob 2 btc");
    bc.mine_block("Justin -> Bruce 2 btc");

    bc.blocks_info();
}