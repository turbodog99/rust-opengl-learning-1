# Working Through Learning OpenGL on Rust

This is my working through and hopefully extending [Rust and OpenGL from scratch](https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html).

For some time, I've had an interest in graphics programming, but I've been very busy with personal matters, and I haven't had as much time for personal coding as I would like.

This is my attempt to work through two goals: learning the Rust programming language and learning modern OpenGL.

I understand that in this day and age, if I want to write a game, I'd probably be better off using an engine like Unity and gaining the ability to publish to multiple platforms at once, but working through this is due to my curiosity about how graphics programming works under the covers.

As you can see from my repos, I got something working in regular, fixed pipeline, OpenGL years ago on a Mac, and I started working through getting the modern pipeline with shaders and buffers working but put it down. With the guidance from the above-mentioned blog posts, I got the modern pipeline at least running enough to display a triangle in Rust so far.

While I've grown to appreciate modern garbage-collected and dynamic languages, I've missed the run-time efficiency of native languages such as C++. In this day and age, computing resources are cheap enough that it has usually been more cost effective to use more servers than to use a language that is more efficient at run time but more complex at development time. There was a hard choice to be made between run time efficiency and development efficiency.

C and C++ have always been some of the most efficient languages out there, but they're dangerous from a security perspective, and it takes more experience and more time to write safe programs in them. The introduction to the video [Building on an unsafe foundation](https://youtu.be/rTo2u13lVcQ) by Jason Orendorff gives some great examples of what I'm talking about.

Rust tries to reduce the need to trade run-time efficiency for development efficiency and safety. I've often said you rarely get a free lunch in programming; you usually find you traded something off for a preferable gain. Some languages, such as Ruby, are extremely concise and require less thinking about memory management or types while hacking at a solution to a problem. Many developers fail to notice the complexity that eventually gets pushed into automated tests. A good compiler that tracks type and memory safety can make sure code that would normally break and require a great deal of testing in a language such as Ruby never executes at all.

We've felt the pain of trying to scale dynamic languages, and I think the recent popularity of languages such as TypeScript is a reaction to that pain.

In a statically-typed language, much of the testing of what inputs are passed to functions is moved to development and compile time, and it's effectively automatic. The case could be made that the need to think about types and memory safety slows down the process of fleshing out how things work and hacking on things until they work. If you believe that, you never really bought in to TDD, did you? If the compiler isn't forcing you to reason about whether, for example, the data passed into your function is actually the type you expect, are the tests you're writing after the fact really covering all of those bases?

Many dynamic languages, as an idiom, discourage type testing within functions. So, you end up pushing the testing up a level to the code that's calling your functions. There's a disconnect here: maybe the calling code isn't even yours. So, maybe you end up writing tests as a hypothetical caller and hope the real caller calls your code correctly. You end up still needing to push testing the validity of the passed type to, maybe, the API layer, if you test it at all. With a statically-typed language with type introspection, you can, for example, write REST code that effectively has automatic error handling with respect to the type validity of passed data without writing any tests about type validity at all.

The last time I thought I should learn a new programming language was when I saw a presentation on Clojure. As a functional language that emphasizes immutability, it has its own ways of making entire classes of bugs impossible to write. It still requires more type and input testing than statically typed languages, but it has a way of making the parts feel simple in isolation so that I'm more confident in what I'm writing than when I use an object-oriented language. I thought learning Clojure helped me grow as a programmer, and I'd highly recommend learning it if not just for that experience.

I find Rust to be exciting because it can give much of the performance of C++ while avoiding many of its dangers. I've found in experimenting with it that Rust mostly works by wrapping the dangerous parts in code that's marked unsafe, but it still ends up much safer than C++. Rust is slowly taking on a role in systems programming, and it's possible our code in general will get safer as the C++ foundation most languages are based on gets replaced. Many bugs in hundreds of languages can be traced to their underlying C++ code. It's still possible to leak memory and resources in Rust, but that's true even of the dynamic languages. Clojure is garbage-collected, but I've still had to track down major resource leaks in Clojure programs.

In the end, it's possible to write bad code in any language. Rust is no different, but I think it's a good step in language evolution.

So, I'm here trying to do something that's traditionally done in C or C++ for efficiency in Rust to take the language for a spin and kick the tires. For now, this is just me following the example code, and I'd recommend following the above-mentioned series of blog posts rather than using the code I have here. It ended up mostly the same anyway, but I find I learn better by following along and building something rather than studying code that's already sitting there.

Doing OpenGL programming will make me use many of the languages features and APIs. I think it's a good place to deep dive for the learning experience.
