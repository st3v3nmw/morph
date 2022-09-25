## morph

> I'll morph to someone else \
> I'm just a ghost \
> ~ TØP

- Paradigm: Mostly pure functional programming language
    - `impure` blocks, like Rust's `unsafe`, for now
- Functions
    - Functions are first class citizens
    - Tail recursion & tail call optimization
    - Higher order functions
    - Pipes: (|>)
    - Partial function application/currying
    - Closures
    - Decorators e.g. `@memoize`
    - Async/Await?
- Type System
    - Types are first class citizens like in Idris
    - Statically typed
    - Strongly typed?
    - Algebraic types (sum & product types)
    - `unit` i.e. `()`
    - No null pointers, Haskell's `Maybe<T>`.. `Some<T>` or `None`
    - Strong type inference
    - Hindley–Milner type system?
    - Generic types? Parameterized types?
    - Dependent types?
        - Aiming for types that are so powerful you can prove mathematical theorems with `morph`
        - `forall`, `there exists`
- Evaluation Strategy
    - Everything is an expression
    - Lazy execution
    - Generators?
    - Implicit parallelism
        - Expressions are thread-safe
        - Static cost estimation
        - Cyclomatic complexity?
        - Theoretical limits: Amdahl's law
- Control Flow
    - Pattern Matching
        - Match blocks, like Rust's
- Memory management
    - Rust's ownership model?
    - Garbage collection?
- IO
    - Monads?
- Vectorization/Array Programming
- Metaprogramming
    - AST-based macros
- Backend
    - LLVM
- Formal verification?

### Interesting Stuff to Look Into

- [Language Design](https://cs.lmu.edu/~ray/notes/languagedesignnotes/)
- [Skip](http://skiplang.com/)
- [Ponylang](https://www.ponylang.io/)
- [Gluon](https://github.com/gluon-lang/gluon)
- [Coq](https://coq.inria.fr/)
- [Kind](https://github.com/Kindelia/Kind)
- [Idris](https://github.com/idris-lang/Idris2)

### Standard Library

- Implicitly imported

#### Functions

- `all()`: Do all elements satisfy a condition?
- `any()`: Does any element satisfy a predicate?
- `map()`
- `reduce()`
- `filter()`
- `max()`
- `min()`
- `reverse()`
- `sum()`
- `zip()`

#### Types

1. List (+, append, [], all, any, filter, foreach, indexOf, join, len)
2. 

#### Abstract Algebra

Operations, Groups, Rings, Fields, Vector Spaces, etc

#### IO

#### Regex

#### Pseudo-random number generator

#### ML

#### Parser Combinator
