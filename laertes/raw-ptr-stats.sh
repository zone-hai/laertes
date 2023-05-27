#!/bin/bash

echo "Benchmark,Statistic,{VoidPtr},{PtrArith},{Extern},{}"

for i in results/*/$1.txt ; do tail -18 $i | head -6 ; done
