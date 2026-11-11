#!/bin/bash

runner=$1

if test -f src/main.rs
then
	echo "Attempting to compile rust code..."
	cargo build --release
	runner="./target/release/project4"
elif test -f Main.py
then
	runner="python3 Main.py"
elif test -f main.cpp
then
	echo "Attempting to compile c++ code..."
	g++ main.cpp
	runner="./a.out"
fi

for value in {0..9}
do
	echo ""
	echo "Running ${value}.code"
	timeout 5 ${runner} Correct/${value}.code Correct/${value}.data > Correct/${value}.student
	echo "Running diff with ${value}.expected"
	grep -o '[[:digit:]]\+' Correct/${value}.student > Correct/temp1
	grep -o '[[:digit:]]\+' Correct/${value}.expected > Correct/temp2
	if cmp -s "Correct/temp1" "Correct/temp2"; then
		echo "Print looks good"
		score=$(($score + 1))
	else
		echo "Student output and expected output are different"
	fi
done

rm Correct/temp1
rm Correct/temp2

echo ""
echo "Correct cases score out of 10:"
echo $score

echo "Done!"