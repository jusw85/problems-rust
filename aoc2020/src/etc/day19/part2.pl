#!/usr/bin/perl
use strict;
use warnings;

while (<>) {
    s/8: 42/8: 42 | 42 8/;
    s/11: 42 31/11: 42 31 | 42 11 31/;

    s/0: 8 11/0: !\(42 11\) ~ 42 0 | 42 11/;

    s/:/ =/g;
    s/(\d+)(?= \d+)/$1 ~/g;
    s/= (.*)$/= _{$1}/;
    s/(\d+)/r$1/g;
} continue {
    print or die;
    if (eof) {
        print "main = _{SOI ~ r0 ~ EOI}\n";
    }
}
