# AOD - lista 3

Aby uruchomić programy należy zainstalować język rust, preferowana wersja toolchain to 1.80.0

Zaczynamy od instalacji [rustup](https://www.rust-lang.org/tools/install)

Instalujemy odpowiednią wersje tollchaina

```
rustup toolchain install 1.80.0
```

Kompilujemy programy poleceniem, lub plikiem makefile
```
cargo build --workspace --release
```

skompilowane programy pojawiają się w folderze:
```
target/release
```
