tail:
    tailwindcss -c tailwind.config.js -i input.css -o output.css

build:
    cargo build --release

run: build
    sudo killall -9 advance_data_base_project
    sudo ./target/release/advance_data_base_project
