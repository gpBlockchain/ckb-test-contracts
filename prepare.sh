echo "get capsule "
cargo install cross --git https://github.com/cross-rs/cross
cargo install ckb-capsule --git https://github.com/quake/capsule.git --branch quake/ckb-0.111
echo "build rust contract"
cd rust/acceptance-contracts
capsule build
echo "build c contract "
cd ../../
cd c
mkdir deps
cd deps
git clone https://github.com/nervosnetwork/ckb-c-stdlib
cd ../
make all-in-docker
