echo "get capsule "
rustup update stable
cargo install cross --git https://github.com/cross-rs/cross
cargo install ckb-capsule --git https://github.com/nervosnetwork/capsule.git --branch develop --locked
# echo "build rust contract"
# cd rust/acceptance-contracts
# capsule build
#echo "build c contract "
#cd ../../
#cd c
#mkdir deps
#cd deps
#git clone https://github.com/nervosnetwork/ckb-c-stdlib
#cd ../
#make all-in-docker
