#! /bin/sh

awk -F ',' '{print $2, $3, $5, $6, $7}' ./Periodic\ Table\ of\ Elements.csv | tr ' ' ',' > elements.csv