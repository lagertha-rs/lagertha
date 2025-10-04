# Test data

## Description

This directory contains test data files used for testing various functionalities of the project.

## Contents

- `fixtures.toml`: A configuration file defining test fixtures from java standard libraries.
- `java/`: A directory containing Java source files for testing.
- `prepare_fixtures.py`: A Python script to prepare test fixtures. It is used for the CI pipeline and for local testing.
  It compiles Java files and organizes them into the appropriate directory structure. For the classes from
  `fixtures.toml` it extracts the required classes from the JDK.

## Usage

To prepare the test fixtures, run the `prepare_fixtures.py` script. This will compile the Java files and set up
the necessary directory structure for testing. The target directory for the compiled classes is `target/test-classes`.

## TODO:

- delete snapshots for custom cases without Main postfix