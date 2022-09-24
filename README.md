# ANI Converter [ WIP ]

Ani Converter is a tool for extracting Windows animated and static cursor files (``.ani`` or ``.cur``) and convert them to XCursors which are used on a wide variety of Linux distributions.

## Motivation

The project is inspired by another converter project https://github.com/paddygord/cursor-converter which is based on the ani2ico program by TeoBigusGeekus.

However I noticed a couple of issues with the ani2ico based converters:

- ani2ico only extracts the ``icon`` headers and without sections like ``anih``, ``seq `` &``rate``.
- cursor-converter assumes all icons are in a simple sequence (1,2,3...) which is not necessarily true.
- cursor-converter assumes all icons are visible for 200ms which is usually wrong.

## Function

TODO