## morph

### Steps

- [ ] Lexer
- [ ] Parser
- [ ] Semantic Analyzer
- [ ] IR Lowering
    - [ ] Type Checking & Inference
    - [ ] Desugaring
- [ ] LLVM IR Code Generation
- [ ] Build Binary

### Random Thoughts

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
    - `let` -> implicit annotation of mutability
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
    - Zero cost abstractions
    - Implicit parallelism
        - Expressions are thread-safe
        - Static cost estimation
        - Cyclomatic complexity?
        - Theoretical limits: Amdahl's law
- Control Flow
    - Pattern Matching
        - Match blocks, like Rust's
    - Break
        - `break`
        - `break ~N` -> Break `N`th outermost loop
    - Range
        - `for (x in start..stop,step) {}`
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

##### "Abstract"

1. Iterable (list, set, tuple): Sized
2. Mapping (dict)
3. Numeric (int, float, complex?)
4. Abstract Base Class
5. Generics


##### "Concrete"

1. str, bytes
2. List (+, append, [], all, any, filter, foreach, indexOf, join, len)
3. Union (| -> sugar), Maybe<T> => Union<T, None>
4. Callable<<T, ...>, T>
5. Self

#### Abstract Algebra

Operations, Groups, Rings, Fields, Vector Spaces, etc

#### IO

#### Regex

#### Pseudo-random number generator

#### ML

#### Parser Combinator
