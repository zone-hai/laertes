#!/bin/bash

rm -f rewrite-invocations/*~
rm -fr rewrite-workspace/*
cp -r test-inputs/* rewrite-workspace/

# run unsafe-counter on all benchmarks
for i in rewrite-invocations/*
do
    export benchmark=$(basename $i)
    mkdir -p results/$benchmark
    pushd ../unsafe-counter
    BENCHMARK=$benchmark cargo run --release --bin unsafe-counter -- `cat ../laertes/$i` | tee ../laertes/results/$benchmark/before.txt
    popd
done

# run resolve-imports on all benchmarks
for i in rewrite-invocations/*
do
    export benchmark=$(basename $i)
    cargo run --release --bin resolve-imports -- `cat $i`
    if [ $benchmark == "tinycc" ] || [ $benchmark == "optipng" ] ; then
	# apply the patch for these benchmarks
	pushd rewrite-workspace
	patch -p1 <../$benchmark.patch
	popd
    fi
       
    mkdir -p results/$benchmark
    pushd ../unsafe-counter
    BENCHMARK=$benchmark cargo run --release --bin unsafe-counter -- `cat ../laertes/$i` | tee ../laertes/results/$benchmark/after-resolve-imports.txt
    popd
done

# run resolve-lifetimes on all benchmarks
for i in rewrite-invocations/*
do
    export benchmark=$(basename $i)
    export extra_args=""
    if [ $benchmark == "urlparser" ] ; then
        extra_args="--single-mod"
    fi
    # skip libxml2
    if [ $benchmark != "libxml2" ] ; then
	cargo run --release --bin resolve-lifetimes -- -f $extra_args `cat $i`
	cargo run --release --bin resolve-imports -- -f `cat $i`
	
	mkdir -p results/$benchmark
	pushd ../unsafe-counter
	BENCHMARK=$benchmark cargo run --release --bin unsafe-counter -- `cat ../laertes/$i` | tee ../laertes/results/$benchmark/after-resolve-lifetimes.txt
	popd
    fi
done
