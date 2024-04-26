# Let's Talk Schism!

## Why?

Whenever someone invents a new language, the first question that gets asked
is "Why another language?".

I don't like that question. The answer to it is "'cuz it's fun". In my opinion,
there's no reason to not invent new languages. In a lot of ways, the languages
that we're all using today kinda suck. They've gotten better over time - and
each of the improvements has come because some nerd somewhere said "Hey, you know what? I'm going to try to invent a language that makes stuff better".

Why schism in particular?

I've been fascinated by functional languages for a long time. The whole
idea of building systems that don't use state for everything is fantastic,
and I'd really like to see more of it. But at the same time, I think
that functional languages have typically fallen down in a few areas:

1. Syntax. They're often read as gobbledegook. In particular, points-free code 
  in a language like Haskell is, to me at least, completely impenetrable.
2. Laziness. I get the attraction of laziness. But in practice, I just
 don't buy it. It's hard to predict the performance of a complex lazy
 system, and I've seen experts get it wrong, because it's just so
 counterintuitive.
3. Concurrency. People say that function programming makes concurrency easier.
  But in practice, in normal programming, it's just not true. It's not
  particularly easy to take a piece of Haskell that wasn't written 
  for concurrency, and make it concurrent. And the packages for concurrency
  that I've seen feed back into the gobbledegood problem.
4. State. Monads are great. I love them. But at the end of the day,
  I don't believe that they scale. One monad makes for really
  beautiful, elegant stateful code. Two monads makes confusing
  code. Thee is spaghetti. But state is an inevitable fact of life:
  every program that we write ultimately has to deal with state in
  some form. You can't escape that, and making working with state
  awkward so that you can have a theoretically beautiful langage
  is the wrong tradeoff.

Based on that, I'm interested in something that makes it easy to
write points-free functional code in a reasonably clear way,
and also makes it easy to use state when necessary, without
making things too complicated. 

But why schism - a weird backwards language like Forth?

Concatenation! Concatenative programming _seems_ like a viable
approach. It gives you points-free when you want it. It gives
you a very clean, natural semantics for combining things. And
it goes back to my earliest days of programming, when Forth was 
one of the first languages that I programmed in. It seems like
an approach that can do some of what I want, and it also seems
like something fun and a little bit silly for me to dig my teeth
into.

## Schism Basics

Every schism program is made up of a bunch of <em>sects</em>. A sect
is a section of code - a module. (But since the language is named schism,
calling a module a sect just fit!) A sect is just
a collection of declarations: it starts by importing identifiers
from other sects, and then declares its own batch of stuff.

The things that can be declared are:
* Functions. Functions map from a stack to a stack. The function
  declaration needs to describe the shape of the stack both before
  and after the invocation, using a stack-effect, which I'll describe
  below.
* Objects, which are the schism version of classes. They declare a
  type of object, a set of parent objects that are composed into
  the object types, and a collection of object members. Object
  members come in three kinds:
   * slots, which are basically object properties/fields.
   * methods,  which are operations implemented as synchronous
     functions by the object.
   * actions, which are asynchronous operations implemented by
     the object. More about actions below.
* Variables. Obvious.
* Signatures. Basically interfaces. Like Go, you don't have to
  declare what signatures you implement: if you implement
  the methods and actions in a signature, that's good enough.


### Functions

```
fun fact ( int -- int ) is
    dup 0 = if
        drop 1
    else
        dup 1 - fact *
    end
end
```

```
fun ['a, 'b, 'c] third ( 'a 'b 'c -- 'a 'b 'c 'a ) is
    local c  /* create a local c,  and store the top value of the stack into it.
                The stack is now ('a 'b). */
    local b  /* create a local b,  and store the top value of the stack into it.
                The stack is now ('a) */
    local a /* create a local a,  and store the top value of the stack into it.
                The stack is now () */
    a b c a  /* push a, then b, then c, then a - so the stack is now (a b c a). */
end
```
### Actions

Actions are where concurrency shows its head in Schism. When
you call an action, a new stack is created, the parameters
declared in the action's type are moved into that new stack,
and the action starts executing using that stack in a new
thread. Actions don't return results: they're just off,
running. To get a result back, you need to either provide
a callback action, or use a synchronizing value like
a semaphore.

The catch here is that an object is single-threaded: if
there's an action or method currently running, the
action execution will be deferred until that action/method
is done. 

