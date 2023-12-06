#!/usr/bin/perl
#
#  Sum the ranges on the first line of the input file. ChatGPT 4.0 wrote this with:
#
#  prompt: "Write a perl script to read the first line from a file, split on whitespace,
#           discard the first token, then sum every 2nd token that is left"
#  then  : "Can it read from stdin"
#
#   $ perl sum_ranges.pl < sample.txt
#   Sum: 27
#
#   $ perl sum_ranges.pl < input.txt
#   Sum: 1934995782
#

use strict;
use warnings;

# Read the first line
my $line = <STDIN>;

# Split the line on whitespace and remove the first token
my @tokens = split(/\s+/, $line);
shift @tokens;

# Initialize sum
my $sum = 0;

# Iterate and sum every second token. [chatgpt started $i at 0 - jrh]
for (my $i = 1; $i < @tokens; $i += 2) {
    $sum += $tokens[$i] if $tokens[$i] =~ /^\d+$/;
}

print "Sum: $sum\n";
