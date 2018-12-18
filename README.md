# Rispreter 
### Another lisp-kinda-of interpreter writen in Rust
#### work-in-progress

---
# About

Rispreter it's my attempt to port **orangeduck/BuildYourOwnLisp** to Rust and learn some rusties and lispies (and c) along the way.

There are some objectives/todos that I want to see in this project, they are (in priority order somehow?) :

* **Learn some programming and have some fun.**

- Implement the basic language as described in the guide, trying at first to stay simple and close with the data structures and logics.

    - or just make it work at first, that's a good sign no?

- Documentation and tests are top priority.

    - I think the documentation part should come after the first refactoring i'm planning.

    - **Tests on everything**, to keep us sane, and in Rust's tests are fun to write.

- Refactoring of the code to make it more rust like, but keeping clarity and good reasonig.

    - The first thing i notice that need a refactor it's the `Lval` struct. I don't like a struct holding everything in it's body, its confusing. We could use Enums variants to describe types maybe (My first try wasn't very successful), or associate with some bounded Type in it's construction... It's that right or possible (eg: Lval<String>). This have implications in big part of the structure of the lval `crate`... should I make this a top priority right now? *thoughts*...

    - After that we can documentate. Docs are *seksi*.

- Implement parts of *Rust Standard Library* that can make sense in the context of this project.

    - I want to see this project being useful.

    - The ***how*** it's a thing we gonna discover, i hope.

- Performance should be considered, it's not like I want to see a fast lisp implementation in Rust. Just that it's cool to know that a thing you are building it's working cool and easy.

- The **Bonus Projects • Chapter 16** of the guide shows some cool ideias that could be implemented after we get a reasonable amount of working code. 

    - Native types are already a priority.
    
    - Operation system interaction should be another priority, but i think that it's tied with the rust `std` implementation. 
    
    - Macros are aliens to me right now, i would like to take another aproach. 
    
    - Pool allocation and GC are topics too much advanced to me right now and are projects on their own, but i think that as we code this whole stuff in Rust we are like being seduced to write safe and performant code. It's not like a frisky pointer will gona leak out of our repl and dangle the program memory space.

    - Tail call optimisation it's a thing i need to reasearch and see how to implement this in Rust, but yes, should be a priority.

    - Lexical scoping and static typing are things to consider after we (i hope) have some solid base.

---

I shall mention some links, projects and references that I follow while i work in this project:

- http://www.buildyourownlisp.com/ (the reason)

- https://en.wikibooks.org/wiki/Write_Yourself_a_Scheme_in_48_Hours

- The https://blog.subnetzero.io/ along with the educational project https://gitlab.com/subnetzero/iridium it's a interisting project that implement a register based VM. That made me think in Lisp-machines and how they could be implemented as a VM or how to think in a bytecode level while implementing a language.

- https://www.nand2tetris.org/ its a source of inspiration and is what made me fall in love with programming. 

Other projects that i keep a eye: 
    
- https://github.com/murarth/ketos (they compile it to bytecode, that's very cool!)

- https://github.com/JunSuzukiJapan/macro-lisp (coool!)

- https://github.com/dtolnay/quote (I have a feeling that this can be very useful someday)