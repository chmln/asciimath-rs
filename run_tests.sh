#!/bin/bash
output=$((cargo test) 2>&1)
OUT=$?
if [ $OUT -eq 0 ];then
   echo "OK."
else
   echo "$output"
fi
