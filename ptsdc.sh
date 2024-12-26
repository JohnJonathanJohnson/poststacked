#!/bin/bash

input=$1
output=$2

./ptsdc $input
cc out.c -o $output
rm out.c
