for i in {1..5}
do
	cargo run --release -- run -t 4 -c true "(Main 0)"	
done &

for i in {1..5}
do
        cargo run --release -- run -t 4 -c true "(Main 0)"
done
