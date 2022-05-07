echo "============`date +%F' '%T`==========="

dev()
{
  RUST_BACKTRACE=1 cargo run
}
prod()
{
  RUSTFLAGS="-C target-cpu=native" cargo build --release
  echo "Starting ..."
  ./target/release/actix-web
}
build()
{
  #RUSTFLAGS="-C target-cpu=native" cargo build --release
  RUSTFLAGS="-Ctarget-cpu=native -Ztune-cpu=native -Zmutable-noalias=yes -Clink-arg=-fexperimental-new-pass-manager \
  -Copt-level=3 \
  -Ccodegen-units=1 -Cpanic=abort -Cembed-bitcode=yes -Cforce-frame-pointers=n -Cdebug-assertions=no -Coverflow-checks=no\
  -Ccontrol-flow-guard=no -Clink-dead-code=no -Zno-parallel-llvm" \
  cargo build --release
}
linux()
{
  cargo build --release --target=x86_64-unknown-linux-musl
}
fmt()
{
   cargo fmt
}


parallel=1

case $(uname) in
 FreeBSD)
    nproc=$(sysctl -n hw.ncpu)
    ;;
 Darwin)
    nproc=$(sysctl -n hw.ncpu) # sysctl -n hw.ncpu is the equivalent to nproc on macOS.
    ;;
 *)
    nproc=$(nproc)
    ;;
esac

# simulate ninja's parallelism
case nproc in
 1)
    parallel=$(( nproc + 1 ))
    ;;
 2)
    parallel=$(( nproc + 1 ))
    ;;
 *)
    parallel=$(( nproc + 2 ))
    ;;
esac

echo "cpu_num = " $parallel


case "$1" in
  dev)
    dev
    ;;
  prod)
    prod
    ;;
  build)
    build
    ;;
  linux)
    linux
    ;;
  fmt)
    fmt
    ;;
esac