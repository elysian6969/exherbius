'lil paludis wrapper for summoning demons, apparently

```
pkg a(dd) <packages...>
pkg i(nfo)
pkg r(remove) <packages...>
pkg s(ync)
pkg u(pdate)
```

spell translations

```shell
$ pkg a sys-libs/musl
"cave" "resolve" "--no-blockers-from" "dev-libs/argp-standalone" "--no-blockers-from" "dev-libs/musl-fts" "--no-blockers-from" "dev-libs/musl-obstack" "--no-blockers-from" "sys-libs/musl-compat" "--preserve-world" "sys-libs/musl" "--execute"
...

$ pkg a -ti686-linux-gnu sys-libs/glibc
"cave" "--environment" ":gnu" "resolve" "--cross-host" "i686-pc-linux-gnu" "--make" "cross-compile" "--preserve-world" "sys-libs/glibc" "--execute"
...

$ pkg a -tx86_64-linux-gnu sys-libs/glibc
"cave" "--environment" ":gnu" "resolve" "--cross-host" "x86_64-pc-linux-gnu" "--make" "cross-compile" "--preserve-world" "sys-libs/glibc" "--execute"
...
```

### yet to update below

download [exherbo musl tarball](https://dev.exherbo.org/stages/exherbo-x86_64-pc-linux-musl-current.tar.xz)

you know what to do

flag key

`-1`/`--preserve-world`

`-x`/`--execute`

`-0`/`--no-dependencies-from`

`-m`/`--make`

`-4`/`--cross-host`

argument to `-m`/`--make` key

`x`/`cross-compile`

configuration

`/etc/paludis/bashrc`
```
CHOST="x86_64-pc-linux-musl"

i686_pc_linux_gnu_CFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"
i686_pc_linux_gnu_CXXFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"

x86_64_pc_linux_gnu_CFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"
x86_64_pc_linux_gnu_CXXFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"

x86_64_pc_linux_musl_CFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"
x86_64_pc_linux_musl_CXXFLAGS="-march=native -Ofast -fomit-frame-pointer -pipe"
```

`/etc/paludis/options.conf`

```
*/* build_options: jobs=12 -recommended_tests
*/* targets: i686-pc-linux-gnu x86_64-pc-linux-gnu x86_64-pc-linux-musl
*/* lua_abis: 5.2

*/* -bash-completion -python

sys-apps/paludis scm
sys-devel/gcc -threads
```

`/etc/palduis/repositories/i686-pc-linux-gnu.conf`

```
format = exndbam
location = ${root}/var/db/paludis/repositories/cross-installed/i686-pc-linux-gnu
name = i686-pc-linux-gnu
split_debug_location = /usr/i686-pc-linux-gnu/lib/debug
tool_prefix = i686-pc-linux-gnu-
cross_compile_host = i686-pc-linux-gnu
```

`/etc/paludis/repositories/x86_64-pc-linux-gnu.conf`

```
format = exndbam
location = ${root}/var/db/paludis/repositories/cross-installed/x86_64-pc-linux-gnu
name = x86_64-pc-linux-gnu
split_debug_location = /usr/x86_64-pc-linux-gnu/lib/debug
tool_prefix = x86_64-pc-linux-gnu-
cross_compile_host = x86_64-pc-linux-gnu
```

`sys-kernel/linux-headers`

```bash
cave resolve sys-kernel/linux-headers -x1
cave resolve -mx -4 i686-pc-linux-gnu sys-kernel/linux-headers -x1
cave resolve -mx -4 x86_64-pc-linux-gnu sys-kernel/linux-headers -x1
```

`sys-libs/glibc`

```bash
cave resolve -mx -4 i686-pc-linux-gnu -0 sys-libs/glibc sys-libs/glibc -x1
cave resolve -mx -4 x86_64-pc-linux-gnu -0 sys-libs/glibc sys-libs/glibc -x1
```

fix missing compiler shit, lmao!

```
cd /usr/host/bin

# c compiler
ln -s i686-pc-linux-gnu-gcc-11 i686-pc-linux-gnu-cc
ln -s x86_64-pc-linux-gnu-gcc-11 x86_64-pc-linux-gnu-cc

# c preprocessor
ln -s cpp i686-pc-linux-gnu-cpp
ln -s cpp x86_64-pc-linux-gnu-cpp

# c++ compiler
ln -s i686-pc-linux-gnu-g++-11 i686-pc-linux-gnu-c++
ln -s x86_64-pc-linux-gnu-g++-11 x86_64-pc-linux-gnu-c++
```

`sys-libs/glibc` (again)

```bash
cave resolve -mx -4 i686-pc-linux-gnu sys-libs/glibc -x1
cave resolve -mx -4 x86_64-pc-linux-gnu sys-libs/glibc -x1
```

`sys-libs/libstdc++`

```bash
cave resolve -mx -4 i686-pc-linux-gnu sys-libs/libstdc++ -x1
cave resolve -mx -4 x86_64-pc-linux-gnu sys-libs/libstdc++ -x1
```

`dev-libs/gmp`

```bash
cave resolve -mx -4 i686-pc-linux-gnu -0 dev-libs/gmp dev-libs/gmp -x1
cave resolve -mx -4 x86_64-pc-linux-gnu -0 dev-libs/gmp dev-libs/gmp -x1
```

`dev-libs/mpfr`

```bash
cave resolve -mx -4 i686-pc-linux-gnu -0 dev-libs/mpfr dev-libs/mpfr -x1
cave resolve -mx -4 x86_64-pc-linux-gnu -0 dev-libs/mpfr dev-libs/mpfr -x1
```

`dev-libs/mpc`

```bash
cave resolve -mx -4 i686-pc-linux-gnu -0 dev-libs/mpc dev-libs/mpc -x1
cave resolve -mx -4 x86_64-pc-linux-gnu -0 dev-libs/mpc dev-libs/mpc -x1
```

`dev-libs/zlib`

```bash
cave resolve -mx -4 i686-pc-linux-gnu -0 dev-libs/zlib dev-libs/zlib -x1
cave resolve -mx -4 x86_64-pc-linux-gnu -0 dev-libs/zlib dev-libs/zlib -x1
```

### thanks

- [exherbo docs/multiarch](https://www.exherbo.org/docs/multiarch/multiarch.html)
