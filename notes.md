Following along with: [Rust and OpenGL from scratch](https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html)

The syntax apparently changed on use declarations in Rust 2018. I ended up needing to refer to
resources using super::resources::Resources in render_gl.rs.

Build dependencies are separate. I slipped up on this initially and put the walkdir crate, which the
build script needs, in the dependencies section.

The example code had me create a variable called gl_context. In C, you'd normally save the
context so that you can delete it later with SDL_GL_DeleteContext(gl_context). I was getting compiler
errors about never using the gl_context variable. I temporarily deleted the line that creates it and
stuff broke. Then, I tried not saving the variable since it wasn't used. Stuff was still broken.

Coming from other languages, I had this instinct to ask, "why would I save this if I don't use it?"
Well, it turns out having the variable there unused established its lifetime so the context could be
cleaned up automatically when I was done with it.

I chose to prepend the variable name with an _ so the compiler would quit complaining about it.

I'm having some issues with local namespacing. Having to use super::whatever to get to a module in a
source file in the same directory doesn't quite feel right. I might revisit it for further study.

Use the repr macro to take control of the memory layout of a struct.

At the end of [The page on refactoring the Vertex attributes](https://nercury.github.io/rust/opengl/tutorial/2018/06/27/opengl-in-rust-from-scratch-09-vertex-attribute-format.html),
the author discusses why he chose to make the wrapping function safe but the functions
that call the unsafe GL functions unsafe. It seems very indicative of the Rust
philosophy.

Procedural macros seem like a really slick idea. They let generate code at compile time to handle multiple types.

For now, I stuck with putting the shaders in an assets directory. The author of the post I've been following seems to have moved shaders into the root directory at some point.

It's apparently not possible to create a generic structure that doesn't actually
contain the generic data type. So, there's a ::std::marker::PhantomData<type> object
that acts as a placeholder but is zero-sized so is no cost.

_marker: ::std::marker::PhantomData<B>

The refactor of ArrayBuffer into a generic Buffer Struct got us ElementArrayBuffers
nearly for free.

Visual Studio code, at least the extension I'm using, flags use of procedural macros
to let you know it can't statically check them...interesting.

After using the provided Triangle code, I ended up removing my data.rs and using the
author's, which necessitated adding a couple of new dependencies. He implemented all of
the various GL data formats.
