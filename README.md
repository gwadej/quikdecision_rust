# Quick Decision in Rust

Library for making random decisions using one of a few different methods.
Command line interface available at https://github.com/gwadej/quikdecision-cli.

## Interface

Each Command type has a `command()` method that takes the appropriate parameters
to create a Command of the right type.

Each Command also implements a `decide()` method that uses the configuration that
it was initialized with to make the appropriate choice and return a Decision enum.

The Decision enum variants wrap the values returned by each Command object with all
of the information needed to appropriately use the decision.

## coin

Simulates the flipping of a fair coin, by returning one of the two strings 'Heads' or 'Tails',
with equal probability.

The `decide()` method returns a `Decision::Text` with the value.

## dice

Simulates rolling one or more dice of various standard shapes. The `command()` method
requires a string containing a "dice expression".

The `decide()` method returns the result of the roll as a `Decision::AnnotatedNum`. The
value field contains the results of the roll as a `u32`. The extra field contains a
description of the roll (see below).

### Dice Expression

The dice expression is a combination of terms of one of three forms joined by +:

   - {n}d{s}: roll n s-sided dice (3d6)
   - {n}x{s}: roll n s-sided exploding dice (2x8)
   - {n}: an increment.

The number of sides supported are 3, 4, 6, 8, 10, 12, 20, or 100. Exploding dice work
much like normal, except when a die rolls the maximum value for the die, then it
is re-rolled to generate a value to add to the original roll. This may happen more
than once.

### Roll Description
             
The roll description attempts to capture the details of a roll in a somewhat compact
expression, while still providing more detail than just the results of the roll.

Normal dice are represented by the dice expression, followed by the results of the
individual die rolls in parentheses: `3d6(3+5+1)`.

Exploding dice are represented by the dice expression followed by an expression in
angle brackets. The expression is made of the results of each individual die in
parentheses, with any die that exploded being followed by an !, and the re-roll
added as many times as needed: `3x6<(1)+(6!+5)+(2)>`.

## oracle

Chooses a random answer from _The Oracle_. The `command()` method returns the
appropriate command.

The `decide()` method returns a `Decision::LabelledText` containing both a
randomly chosen answer from the Oracle's set of 9 positive answers, 9 negative
answers, or 6 indeterminate answers as the value, and a label randomly chosen
to make any pronouncement seem more _Oracle-like_.

## percent

Makes a `true` choice some percent of the time. The `command()` method allows
you to specify the percent of the time that the decision will be `true`.

Only integer percentages from 1 to 99 are supported.

The `decide()` method returns a `Decision::Bool` variant with the result.


## pick

Selects a number between two supplied `i32`s (inclusive) with equal probability.
The two numbers cannot be the same.

The `command()` method allows you to specify the range of values. 

The `decide()` method returns a `Decision::Num` containing the results of the
choice.

## select

Selects one of the supplied strings with equal probability. There must be
at least two strings to choose between.

The `command()` method allows you to specify the `Vec` of `String`s to choose
from.

The `decide()` method returns a `Decision::Text` containing the results of the
choice.

## Disclaimer

This library is does not use a cryptographically secure random number generator.
It should not be used to make important decisions. The program is just for
entertainment.

It is also my first Rust program that was not just a trivial exercise from a tutorial.
I assume my idioms are off, and the code is probably not the most efficient.
