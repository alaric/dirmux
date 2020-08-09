# dirmux

![Rust Build Status Badge](https://github.com/alaric/dirmux/workflows/Rust/badge.svg)

Directory multiplexing program for managing multiple directories at once. With this you are able to
tag directories and execute commands against them all at once.

This is particularly useful if you have situations such as many git repositories and you want to
keep them all up to date, or have to make cross-cutting changes against all of them.

The dirmux command works from anywhere on the file system, so whenever you want to see status across
your git repos or check on your TODOs it's easy and fast to do.

## Trying it out

If you have the rust toolchain installed:

    cargo install dirmux

If you want a different package manager, please enter an issue for it.

## Motivational Examples

This command will run the 'git fetch --all' command against all known directories:

    $ dirmux git fetch --all

The output will look like this (but coloured):

    ~/projects/blog:
    Fetching origin
    ~/notes:
    Fetching origin

As you can see above, it captures and prints the output from each command and prints it along side
the directory entry. If the output is _empty_ it will omit the directory entirely.
    
This is a built-in command will print a short, 1 line per directory with 'interesting' content,
summary of the git repositories tagged 'work':

    $ dirmux -t work status
    
Which will provide an output similar to this:

       ~/projects/dirmux 1M           main
         ~/projects/blog 1M 3?        main
              ~/dotfiles 1M           nvim_lsp2
                 ~/notes 1?           main

If the branch is not changed and there are no modified files in the git index or working set, it
will omit to print the directory.

Other examples that might be worth trying:

    $ dirmux -t home rg TODO   # Finding something to do in my home projects
    $ dirmux -t autosync git commit -am "Auto-sync" # Or equivalent script to also push...

## Why yet-another multi-git/multi-directory tool?

I've been a long time user of [gr](https://github.com/mixu/gr) and it's always bothered me in some
ways. If you're operating on 100s of directories it's quite slow and doesn't execute the commands in
parallel, and the output is quite padded. The tagging has been unintuitive for me (I've included the
'@' into the tag names and confused myself many times). These led me to build this again as a fun
rainy quarantine weekend Rust project.


