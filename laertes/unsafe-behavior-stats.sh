#!/bin/bash

echo "Benchmark,Statistic,ReadFromUnion,MutGlobalAccess,InlineAsm,ExternCall,RawPtrDeref,UnsafeCast,Alloc"

for i in results/*/$1.txt ; do tail -9 $i | head -3 ; done
