# BIRDS

Combinators implemented in Rust named after Raymond Smullyan's birds.

Lambda calculus and Haskell use single argument functions that return another function to "simulate" multiple arguments. Implementing such ideas in Rust can quickly get annoying in Rust both due to the `()` syntax used to invoke functions and the need to `Box` our closures. Therefore I've simply used multiple arguments in such cases. (If you have any better ideas please file an [issue](https://github.com/ArchitBhonsle/birds/issues) detailing your idea.

_This is just a hobby project to give myself an excuse to explore combinatory logic and Rust generics._

## Combinators

- [x] Bluebird
- [ ] Blackbird
- [ ] Bunting
- [ ] Becard
- [x] Cardinal
- [ ] Dove
- [ ] Dickcissel
- [ ] Dovekies
- [ ] Eagle
- [ ] Bald Eagle
- [ ] Finch
- [ ] Goldfinch
- [ ] Hummingbird
- [ ] Identity Bird (aka Idiot)
- [ ] Jay
- [ ] Kestrel (True)
- [ ] Lark
- [ ] Mockingbird
- [ ] Double Mockingbird
- [ ] Owl
- [ ] Queer Bird
- [ ] Quixotic Bird
- [ ] Quizzical Bird
- [ ] Quirky Bird
- [ ] Quacky Bird
- [ ] Robin
- [ ] Starling
- [ ] Thrush
- [ ] Turing
- [ ] Vireo (aka Pairing)
- [ ] Warbler
- [ ] Converse Warbler
- [ ] Why Bird (aka Sage Bird)
- [ ] Identity Bird Once Removed
- [ ] Warbler Once Removed
- [ ] Cardinal Once Removed
- [ ] Robin Once Removed
- [ ] Finch Once Removed
- [ ] Vireo Once Removed
- [ ] Identity Bird Twice Removed
- [ ] Warbler Twice Removed
- [ ] Cardinal Twice Removed
- [ ] Robin Twice Removed
- [ ] Finch Twice Removed
- [ ] Vireo Twice Removed
- [ ] Kite (False)
- [ ] Omega
- [ ] Konstant Mocker
- [ ] Crossed Konstant Mocker
- [ ] Theta

## Credits

- [To Mock a Mockingbird](https://en.wikipedia.org/wiki/To_Mock_a_Mockingbird) for the cute names.
- [data.aviary.birds](https://hackage.haskell.org/package/data-aviary-0.4.0/docs/Data-Aviary-Birds.html) for providing the awesome type signatures and the idea for this library.
- [Combinator birds](https://www.angelfire.com/tx4/cus/combinator/birds.html) for providing a nice list of all the birds.
