# Quick Decision in Rust

A new implementation of a program I wrote for my wife back in the '90s to
make random decisions.

## Interface

The interface is pretty simple:

    quickdecision {command} [cmd_args ...]

    where {command} is one of:
    
      flip
             Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability
             of returning either one.
      coin
             Returns one of the two strings 'Heads' or 'Tails'. There is an equal probability
             of returning either one.
      pick {low} {high}
             Selects a number between two supplied values (inclusive) with equal probability.
             The two numbers cannot be the same.
      choose
             Selects a number between two supplied values (inclusive) with equal probability.
             The two numbers cannot be the same.
      percent {num}
             Treats the supplied integer as a percentage and returns the string 'True'
             that percent of the time. Otherwise, return the string 'False'.
      likely {num}
             Treats the supplied integer as a percentage and returns the string 'True'
             that percent of the time. Otherwise, return the string 'False'.
      roll {dice expr}
             Roll the described combination of dice, returning a number and description of the
             roll. The {dice expr} is a combination of terms of one of three forms joined by +:
               - {n}d{s}: roll n s-sided dice (3d6)
               - {n}x{s}: roll n s-sided exploding dice (2x8)
               - {n}: an increment.
             The number of sides support are 4, 6, 8, 10, 12, 20, or 100. Exploding dice work
             much like normal, except when a die rolls the maximum value for the die, then it
             is re-rolled to generate a value to add to the original roll. This may happen more
             than once.
             The return is the sum of all of the rolls followed by a string representing the
             individual rolls. Normal dice are represented by the expression, followed by the
             sum of the individual die rolls in parens: 3d6(3+5+1). Exploding dice are
             represented by the expression followed by an expression in angle brackets. The
             expression is made of the results of each individual die in parens, with any die
             that exploded being followed by an !: 3x6<(1)+(6!+5)+(2)>.
      dice {dice expr}
             Roll the described combination of dice, returning a number and description of the
             roll. The {dice expr} is a combination of terms of one of three forms joined by +:
               - {n}d{s}: roll n s-sided dice (3d6)
               - {n}x{s}: roll n s-sided exploding dice (2x8)
               - {n}: an increment.
             The number of sides support are 4, 6, 8, 10, 12, 20, or 100. Exploding dice work
             much like normal, except when a die rolls the maximum value for the die, then it
             is re-rolled to generate a value to add to the original roll. This may happen more
             than once.
             The return is the sum of all of the rolls followed by a string representing the
             individual rolls. Normal dice are represented by the expression, followed by the
             sum of the individual die rolls in parens: 3d6(3+5+1). Exploding dice are
             represented by the expression followed by an expression in angle brackets. The
             expression is made of the results of each individual die in parens, with any die
             that exploded being followed by an !: 3x6<(1)+(6!+5)+(2)>.
      select {strs}
             Selects one of the supplied strings with equal probability. There must be
             at least two strings to choose between.
      select @{filename}
             Loads a series of strings from the specified file. (Each line is one string.)
             Selects one of the supplied strings with equal probability. There must be
             at least two strings to choose between.
      oracle
             This command returns a string randomly selected from the Oracle's set of
             9 positive answers, 9 negative answers, or 6 indeterminate answers.
      help
             The help screen
      man
             A long form description of the various commands.

## Disclaimer

This program is does not use a cryptographically secure random number generator.
It should not be used to make important decisions. The program is just for
entertainment.

It is also my first Rust program that was not just a trivial exercise from a tutorial.
I assume my idioms are off, and the code is probably not the most efficient.
