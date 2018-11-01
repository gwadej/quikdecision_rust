# Quick Decision in Rust

A new implementation of a program I wrote for my wife back in the '90s to
make random decisions.

## Interface

The interface is pretty simple:

    quickdecision {command} [cmd_args ...]

    where {command} is one of:
      flip    - 50% chance of a Heads or Tails
      coin    - alias for flip
      pick {low} {high}
              - pick a number between {low} and {high}
      choose  - alias for pick
      percent {num}
              - True {num} percent of the time, otherwise False
      likely  - alias for percent
      roll {dice expr}
              - Roll the described combination of dice, returning a number
                {dice expr} is a combination of terms of the form {n}d{s}
                where {n} is a positive integer, {s} is a number of sides
                for the dice (4, 6, 8, 10, 12, 20, or 100). A term can also
                be just an integer. (e.g. 2d6+1d4+2)
                Replace the 'd' with 'x' for exploding dice.
      dice    - alias for roll
      select {strs}
              - Select one of two or more string supplied as arguments
      select @{filename}
              - Select one of the lines in the file specified
      oracle  - Return a random answer from the oracle
      help    - This screen

## Disclaimer

This program is does not use a cryptographically secure random number generator.
It should not be used to make important decisions. The program is just for
entertainment.
