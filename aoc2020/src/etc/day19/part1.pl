#!/usr/bin/perl
use strict;
use warnings;

while (<>) {
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
