I did a couple puzzles last year, and got reminded that AoC is going on this year again.
I'm interested in learning rust, but never did. So this is to learning.

# Credits:
Setup taken from [Chris Biscardi](https://github.com/ChristopherBiscardi/advent-of-code), but completely stripped down to just the parts I need.

# Help:
To generate empty template for a fresh day*: `cargo generate --path ./daily_template --name day-01`
To test: `cargo test -p day-03 part1`
To run: `cargo run -p day-01 --bin part1` 
*Install cargo-generate to enable generation.

# Confessions:
- Day-01: Mostly 'stolen' from Chris, mostly due to not knowing any rust or how to get started.
- Day-02: Mostly 'stolen' from Chris, same reason, did however remove his solution for ascending/descending checking and used my own mathmatical based solution.
- Day-03: Completely self written! I should improve the solutions by better applying capture groups to extract the numbers from the `mul(x,x)` statements.
- Day-04: Mostly 'stolen' again...
- Day-05: Needed help on what data structure to use for the map (never used hashmaps before), but other than than I wrote it all on my own.
- Day-06: A lot of fun, ran into an issue I had to find a workaround for as I couldn't find a fix for it on my own.
- Day-07: At first I parsed into u32, but that overflowed and had to switch to u64 as a result. Also another reminder that I need to get better at regex in rust, as I stuffed that up at first as well.

