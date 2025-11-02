# word-bomb-defuser

Solver for `#word-bomb-mini`, written in blazing-fast Rust.

## usage

Run

```bash
$ cargo run --release
```

in the repo directory. Then use the prompt.

* The solver is compatible with regex. For example, `a.q` will find matches for words containing "a_q".
* The solver has memory. Hit enter multiple times to repeat a prompt while ignoring previously-seen words.

## statement of game design

"Word Bomb" refers to a general party game of the following design:

> **Prompter**
> 
> 1. Think of a word (from an agreed-upon reference dictionary), such as "mycelium".
> 
> 2. Select a substring, such as "yc". This is the _prompt_.

> **Guesser(s)**
> 
> 1. Find a word (a) containing the substring given, such as "cycle" for "yc", that
>    has (b) not been used in previous rounds, (c) in a limited time frame.
>    This is a _response_.
>
> 2. Survive the reduction of response space, and possibly the reduction of
>    response time, until all other adversarial guessers are eliminated.

Then the design of a solver is obvious.

Faithfully acquire (this is what `word-bomb-defuser`
did for the Word Bomb implementation `#word-bomb-mini`) or
approximate (this is what similar projects do) the reference dictionary, then
find an acceptable response quickly.

Regarding the guesser role,

- 1a is satisfied by Rust regex.
    - `_` is morphed into `.` since many implementations of Word Bomb use this pattern.
    - Lengths are sorted, since competitive Word Bomb implementations constrain word length.
- 1b is partially satisfied by the memory of `word-bomb-defuser`.
    - Repeating a prompt or sending an empty prompt will subtract the set of currently viewed
    responses from the response space and look for alternatives (the limit will approach zero).
    - Changing the prompt will refresh the response space.
- 1c and 2 are partially satisfied by Rust regex.
    - Rust regex is faster than equivalents such as Python's `re.compile` and C++ stdlib regex.
    - Entry of the word is constrained by user typing speed. Other projects use text
    recognition and simulated keyboard inputs to fully satisfy this constraint,
    at the expense of portability.
