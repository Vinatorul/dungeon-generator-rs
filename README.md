# Dungeon Generator

[![Build Status](https://travis-ci.org/Vinatorul/dungeon-generator-rs.svg?branch=master)](https://travis-ci.org/Vinatorul/dungeon-generator-rs)
[![Coverage Status](https://coveralls.io/repos/Vinatorul/dungeon-generator-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/Vinatorul/dungeon-generator-rs?branch=master)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Vinatorul/dungeon-generator-rs/blob/master/LICENSE)

A crate containing different generation algorithms under single interface

## [Documentation](http://vinatorul.github.io/dungeon-generator-rs/dungeon_generator/)

## Example
For example check
[this repo](https://github.com/Vinatorul/level_generator_protohttps://github.com/Vinatorul/level_generator_proto).
![example](example.png)

## License
`Dungeon Generator` is licensed under the MIT license. Please read the LICENSE file in this repository for more information.

# [How to Contribute](https://github.com/Vinatorul/tileengine-rs/blob/master/CONTIBUTING.md)

Contributions are always welcome! Please use the following guidelines when contributing to `Dungeon Generator`

1. Fork `Dungeon Generator`
2. Clone your fork (`git clone https://github.com/$YOUR_USERNAME/dungeon-generator-rs && cd dungeon-generator-rs`)
3. Create new branch (`git checkout -b new-branch`)
4. Make your changes, and commit (`git commit -am "your message"`)
 * I use a [conventional](https://github.com/ajoslin/conventional-changelog/blob/a5505865ff3dd710cf757f50530e73ef0ca641da/conventions/angular.md) changelog format so I can update my changelog using [clog](https://github.com/thoughtram/clog)
 * In addition to the conventions defined above, I also use `imp`, `wip`, `gr`.
 * Format your commit subject line using the following format: `TYPE(COMPONENT): MESSAGE` where `TYPE` is one of the following:
    - `feat` - A new feature
    - `imp` - An improvement to an existing feature
    - `perf` - A performance improvement
    - `tests` - Changes to the testing framework or tests only
    - `fix` - A bug fix
    - `refactor` - Code functionality doesn't change, but underlying structure may
    - `style` - Stylistic changes only, no functionality changes
    - `wip` - A work in progress commit (Should typically be `git rebase`'ed away)
    - `chore` - Catch all or things that have to do with the build system, etc
 * The `COMPONENT` is optional, and may be a single file, directory, or logical component. Can be omitted if commit applies globally
5. Run the tests (`cargo test`)
6. `git rebase` into concise commits and remove `--fixup`s (`git rebase -i HEAD~NUM` where `NUM` is number of commits back)
7. Push your changes back to your fork (`git push origin $your-branch`)
8. Create a pull request! (You can also create the pull request first, and we'll merge when ready. This a good way to discuss proposed changes.)
