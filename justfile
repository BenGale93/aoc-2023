new day year:
    cargo new d{{day}}
    cd d{{day}} && aoc download --day {{day}} --year {{year}}
    echo 'aoc_utils = { path = "../aoc_utils/" }' >> d{{day}}/Cargo.toml

lint:
    cargo clippy -- -W clippy::nursery -W clippy::pedantic
