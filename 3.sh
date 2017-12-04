#!/bin/bash
# too lazy to solve this one in rust
# and the sequence of numbers is known...
curl -s http://oeis.org/A141481/b141481.txt | awk '{ if( int($2) > 361527 ){print $2; exit(0);} }'

