# Changelog

All important changes for the QuikDecision Rust library will be documented
in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.2] - 2021-05-10
### Changed
- Replaced hand-coded PartialEq implementations with derivations

## [0.14.1] - 2020-11-21
### Changed
- Separate dev dependencies from release dependencies

## [0.14.0] - 2020-09-27
### Added
- Crate-specific Error and Result enums
- Ability to return Shuffled decks
### Changed
- Improved error reporting/handling
- Refactor some of the enums to contain classes instead of raw data
- Refactoring to prepare for real changes

## [Unreleased]
### Added
- Iterator interface to the quikdecision::Command enum, turning a command into an infinite iterator.
### Changed
- Change glyph into an Option, so that we can handle cards without a unicode character to match.

## [0.8.5] - 2019-01-13
### Added
- Adding support for drawing a card from a deck of cards.

## [0.8.2] - 2019-01-03
### Added
- Adding support for shuffling a list of strings.

## [0.8.1] - 2019-01-01
### Changed
- Update help on the PercentLikely command
- Make the parameter validation on Pick Number more idiomatic
- Minor code cleanup.

## [0.8.0] - 2018-12-21
### Changed
- DiceRolls now support 3-sided dice

## [0.7.5] - 2018-12-07
### Changed
- Update code to 2018 edition of Rust

## [0.7.1] - 2018-11-26
### Added
- Add support for a version command
