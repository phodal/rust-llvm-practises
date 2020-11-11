From: [https://people.seas.harvard.edu/~chong/software.html](https://people.seas.harvard.edu/~chong/software.html)

## Language Design and Analysis

*   Flamio
    [Flamio](https://people.seas.harvard.edu/~chong/abstracts/PedersenC2019.html) is a programming language (a domain-specific languages embedded in Haskell) that instantiates the Flow-Limited Authorization Model (FLAM) with coarse-grained information-flow control. (See the publication for a link to download the code.)

*   Whip
    [Whip](https://people.seas.harvard.edu/~chong/abstracts/WayeDC2017.html) is a contract system for modern services. Whip (i) provides programmers with a higher-order contract language tailored to the needs of modern services; and (ii) monitors services at run time to detect services that do not live up to their advertised interfaces. Contract monitoring is local to a service. Services are treated as black boxes, allowing heterogeneous implementation languages without modification to services' code. Thus, Whip does not disturb the loosely coupled nature of modern services. See the [Whip website](http://whip.services/) for more details and links to code.

*   Pidgin
    [Pidgin](https://people.seas.harvard.edu/~chong/abstracts/JohnsonWMC2015.html) is a program analysis and understanding tool that enables the specification and enforcement of precise application-specific information security guarantees. Pidgin combines program-dependence graphs (PDGs), which precisely capture the information flows in a whole application, with a custom PDG query language. Queries express properties about the paths in the PDG; because paths in the PDG correspond to information flows in the application, queries can be used to specify global security policies. You can download Pidgin from the [Accrue project page](https://people.seas.harvard.edu/~chong/accrue.html).

*   Shill
    Shill is a shell scripting language designed to make it easy to follow the Principle of Least Privilege. Shill uses capabilities to control what access scripts have to your system. Every Shill script comes with a contract that describes what it can do, so users can run third-party scripts with confidence. Using capability-based sandboxes, Shill's security guarantees extend even to native executables launched by scripts. See the [Shill web page](http://shill.seas.harvard.edu/) for more information.

*   Accrue
    The Accrue Interprocedural Java Analysis Framework (Accrue) is a framework for interprocedural analysis of Java programs. It is built using the [Polyglot extensible compiler framework](http://www.cs.cornell.edu/Projects/polyglot/), and is also suitable for interprocedural analysis of programs written in Java language extensions implemented with Polyglot. We developed Accrue to enable inference and discovery of strong information security guarantees in Java programs. See the [Accrue project page](https://people.seas.harvard.edu/~chong/accrue.html) for more information.

*   Polyglot
    Polyglot is an extensible Java source-to-source compiler. Polyglot facilitates extension of the Java programming language with new language features. See the [Polyglot home page](http://www.cs.cornell.edu/Projects/polyglot/) for more information.

*   Jif
    Jif is a security-typed programming language that extends Java with support for information flow control and access control, enforced at both compile time and run time. See the [Jif home page](http://www.cs.cornell.edu/jif/) for more information.

*   Jif<sub>_E_</sub>
    Jif<sub>_E_</sub> extends the Jif programming language with support for [declassification and erasure](https://people.seas.harvard.edu/~chong/abstracts/ChongM08.html) information security policies. Jif<sub>_E_</sub> has been used to implement [Civitas](https://www.cs.cornell.edu/projects/civitas/), a remote voting system. Please contact me if you are interested in obtaining a copy of the Jif<sub>_E_</sub> compiler and run-time system.