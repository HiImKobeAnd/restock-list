cargo build --release
cp ./target/release/restock_list /usr/local/bin
cp ./restock-list.service /etc/systemd/system
