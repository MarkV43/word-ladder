# Word Ladder Solver

This is a simple solver for the famous [Word Ladder](https://en.wikipedia.org/wiki/Word_ladder) problem.
For example, if given the two words `HEAD` and `TAIL`, it's going to produce an optimal solution from the
first word to the next, changing one letter at once, making sure all words along the way are valid:

    HEAD
    HEAL
    TEAL
    TELL
    TALL
    TAIL

## Usage

To solve a ladder, you may run `cargo r -r`, and type in the two desired words.
As an alternative, you may also run `cargo r -r -- <START> <END>`.

You may also find the largest possible ladder for a given word length by running `cargo r -r -- --longest <LENGTH>`, which in the case of a length of 4 will give a 14-step ladder as solution.

You may also use the the option `--random` or `-r` on any of the above commands to get a non-deterministic evaluation, meaning every time you solve a ladder you may get different solutions.

As a last feature, you may also use the option `--gui` to open a GUI with which to interact. This interface allows for the easy exclusion of unwanted words, and fast changing of the ladder's start and end.