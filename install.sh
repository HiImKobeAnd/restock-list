cp ./target/release/restock_list /opt/restock_list
cp ./products.yaml /opt/restock_list
cp ./configuration.yaml /opt/restock_list
cp ./restock-list.service /etc/systemd/system
ln -s /opt/restock_list/restock_list /usr/local/bin/restock_list
