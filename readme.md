# AOD - lista 3

Aby uruchomić programy należy zainstalować język rust, preferowana wersja toolchain to 1.80.0

Zaczynamy od instalacji [rustup](https://www.rust-lang.org/tools/install)

Instalujemy odpowiednią wersje tollchaina

```
rustup toolchain install 1.80.0
```

Kompilujemy programy poleceniem
```
cargo build --bin dijkstra --bin dial --bin radixheap --release
```
lub plikiem makefile
```
make
```

skompilowane programy pojawiają się w folderze:
```
target/release
```

<br><br>


Jeśli chcemy skompilować wszystkie programy, łącznie z tymi do generowania wykresów
```
cargo build --example experiment_1 --example experiment_2 --release
```
lub plikiem makefile
```
make experiments
```
