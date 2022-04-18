# random_api
jak uruchomić
1. pobranie repo
```
git clone https://github.com/MagullOff/random_api && cd random_api
```
2. instalacja rusta (https://www.rust-lang.org/tools/install) i upewnienie się że system jest aktualny oraz zainstalowany jest pakiet build-essential i libpq-dev
3. zmiana na wersje nightly
```
rustup default nightly
```
4. instalacja i uruchomienie postgreSQL (powinien być zainstalowany na ubuntu)
5. instalacja diesel_cli
```
cargo install diesel_cli --no-default-features --features postgres
```
6. utowrzenie bazy danych
```
echo DATABASE_URL=postgres://postgres:postgres@localhost/api-db > .env
diesel setup
diesel migration run
diesel migration redo
```
7. pliki konfiguracyjne 

Zarówno powyższa komenda jak i plik Config.toml z repozytorium zakłada że hasło i login do postgresa to postgres i postgres. Należy to zamienić na faktyczne dane przed uruchomieniem

8. uruchomienie testów 
```
cargo test
```
9. uruchom aplikacje
```
cargo build && cargo run
```
10. użycie
aplikacja została napisana tak, aby znaleziony obrazek był wyświetlany w przeglądarce po zakończeniu requesta
