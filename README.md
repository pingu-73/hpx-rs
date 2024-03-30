# hpx-rs

Example build instructions:

```sh
#!/bin/bash
export _HOME="$(realpath ~)"
export CXX=/opt/homebrew/opt/llvm/bin/clang++
export CC=/opt/homebrew/opt/llvm/bin/clang
export HPX_INSTALLED_DIR=$_HOME/workspace/hpx-compiled 
export Boost_DIR=$_HOME/workspace/boost-1.84.0/ 
export HWLOC_SRC=$_HOME/workspace/hpx/build/_deps/hwloc-installed
export LD_FLAGS="-L$HPX_INSTALLED_DIR/lib -L $_HOME/workspace/hpx/build/lib $(ls "$(realpath $HPX_INSTALLED_DIR/lib)"/lib*.a | sed "s|$_HOME\/workspace\/hpx-compiled\/lib\/|-l|g" | sed 's/\.a//' | tr -s '\n' ' ' | sed 's/lib//g')"

export CXXFLAGS="${CPPFLAGS}"" -I./include -I$HPX_INSTALLED_DIR/include  -I$Boost_DIR -I$HWLOC_SRC/include"

cargo build

unset CC
unset CXX
unset HPX_INSTALLED_DIR
unset Boost_DIR
unset HWLOC_SRC
unset LD_FLAGS
unset CXXFLAGS
unset _HOME
```
