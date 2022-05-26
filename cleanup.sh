#! /bin/sh

cat './Periodic Table of Elements.csv' | tr -d ' ' | awk -F ',' '{print $2, $3, $5, $6, $7}' | tr ' ' ',' > elements.csv
