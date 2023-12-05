#!/usr/bin/perl
#
# this chatgpt 4.0 perl script helped me notice I missed typing the & ampersand symbol into the regex,
# which caused my part 2 solution to be too low
#

use strict;
use warnings;

# Initialize an empty hash to store character counts
my %char_count;

# Read from file passed as an argument or from standard input
while (my $line = <>) {
    # Increment the count for each character
    foreach my $char (split //, $line) {
        $char_count{$char}++;
    }
}

# Print the counts for each character
foreach my $char (sort keys %char_count) {
    print "'$char' occurs $char_count{$char} times\n";
}
