//! # Using the library in combinatoric style

//! # About examples
//! Examples tend to omit [`help`](NamedArg::help), you should try to specify them whenever possible.
//!
//! Most of the examples stop at defining the [`Parser`], to be able to run them you need to
//! convert your `Parser` into [`OptionParser`] with [`Parser::to_options`].
//!
//! ```rust
//! # use bpaf::*;
//! fn parser() -> OptionParser<bool> {
//!     short('s')
//!         .switch()
//!         .to_options() // <- important bit
//! }
//! ```

//! In addition to examples in the documentation there's a bunch more in the github repository:
//! <https://github.com/pacak/bpaf/tree/master/examples>

//! # Recommended reading order

//! Combinatoric and derive APIs share the documentation and most of the names, recommended reading order:
//! 1. [`construct!`] - what combinations are and how you should read the examples
//! 2. [`NamedArg`], [`positional`] and [`command`] - on consuming data
//! 3. [`Parser`] - on transforming the data
//! 4. [`OptionParser`] - on running the result

//! # Getting started

//! 1. Define primitive field parsers using builder pattern starting with [`short`], [`long`],
//! [`command`], [`positional`] or [`any`], add more information using [`help`](NamedArg::help),
//! [`env`](NamedArg::env) and other member functions.
//!
//!    For some constructors you end up with parser objects right away,
//!    some require finalization with [`argument`](NamedArg::argument), [`flag`](NamedArg::flag)
//!    or [`switch`](NamedArg::switch).
//!
//!    At the end of this step you'll get one or more parser
//!    one or more objects implementing trait [`Parser`], such as `impl Parser<String>`.
//!
//! 2. If you need additional parsing and validation you can use trait [`Parser`]: [`map`](Parser::map),
//!    [`parse`](Parser::parse), [`guard`](Parser::guard).
//!
//!    You can change type or shape of contained or shape with [`many`](Parser::many),
//!    [`some`](Parser::some), [`optional`](Parser::optional) and add a fallback values with
//!    [`fallback`](Parser::fallback), [`fallback_with`](Parser::fallback_with).
//!
//! 3. You can compose resulting primitive parsers using [`construct`] macro into a concrete
//!    datatype and still apply additional processing from step 2 after this.
//!
//! 4. Transform the toplevel parser created at the previous step into [`OptionParser`] with
//!    [`to_options`](Parser::to_options) and attach additional metadata with
//!    [`descr`](OptionParser::descr) and other methods available to `OptionParser`.
//!
//! 5. [`run`](OptionParser::run) the resulting option parser at the beginning of your program.
//!    If option parser succeeds you'll get the results. If there are errors or user asked for help info
//!    `bpaf` handles this and exits.
//!
//! 6. Add [`check_invariants`](OptionParser::check_invariants) to your test code.

#[cfg(doc)]
use crate::{parsers::*, *};
