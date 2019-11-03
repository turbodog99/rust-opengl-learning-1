Following along with: [Rust and OpenGL from scratch](https://nercury.github.io/rust/opengl/tutorial/2018/02/08/opengl-in-rust-from-scratch-00-setup.html)

The syntax apparently changed on use declarations in Rust 2018. I ended up needing to refer to
resources using super::resources::Resources in render_gl.rs.

Build dependencies are separate. I slipped up on this initially and put the walkdir crate, which the
build script needs, in the dependencies section.

The example code had me create a variable called gl_context. In C, you'd normally save the
context so that you can delete it later with SDL_GL_DeleteContext(gl_context). I was getting compiler errors about never using the gl_context variable. I tried not saving the variable since it wasn't used. Stuff broke.

Coming from other languages, I had this instinct to ask, "why would I save this if I don't use it?"
Well, it turns out having the variable there unused established its lifetime so the context could be
cleaned up automatically when I was done with it.

I chose to prepend the variable name with an \_ so the compiler would quit complaining about it.

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
contain the generic data type. So, there's a `::std::marker::PhantomData<type>` object
that acts as a placeholder but is zero-sized so is no cost.

```
_marker: ::std::marker::PhantomData<B>
```

The refactor of ArrayBuffer into a generic Buffer Struct got us ElementArrayBuffers
nearly for free.

Visual Studio code, at least the extension I'm using, flags use of procedural macros
to let you know it can't statically check them...interesting.

After using the provided Triangle code, I ended up removing my data.rs and using the
author's, which necessitated adding a couple of new dependencies. He implemented all of
the various GL data formats.

I've completed the tutorial author's suggestions. I'm moving on to a bit of refactoring
of some issues I noticed.

I didn't like that the initial window size was hardcoded. Right from the start, it
introduces the possibility that the viewport size could diverge from the window size.
It may turn out that I want to use a different viewport size, but not yet. Also, creating
the initial size constants contributes to the readability of the code. I'm not usually a
fan of incorporating type information in variables, but I do like including information
about what purpose a value serves.

I noticed a dependency in ColorBuffer that I think should go away. It has code to set
the OpenGL clear color as well as code to actually clear the buffer. It seems odd to
me that I'm asking an object that represents a color to clear the contents of an
OpenGL viewport. We have a Viewport object, and I think this maybe more properly
belongs there.

I'm punting on this for the time being, but just putting it out there: I'll probably want
to support Vulkan or another API in the future. I think the OpenGL dependency could be
decoupled. I think for now this falls in the "You Aren't Going to Need It" (YAGNI) category.
If I decide to add Vulkan support, I think it would be best to use traits that are
mixed in at build time depending on which API is to be built. I've noticed a number
of games provide two executables: one for OpenGL and one for Vulkan. Checking the API
on every call at runtime would introduce too much overhead.

While thinking on moving the ColorBuffer functions into Viewport, I also noticed
another disconnect that concerns me: the Viewport object maintains state that is
separate from the underlying OpenGL viewport that can easily get out of sync. It's
actually very easy for this to happen.

I started the above refactor that I was considering, and I kept feeling as though
this ColorBuffer and Viewport object thing felt like wrapping an interface for not
much gain. The suggested ColorBuffer code also seemed to go out of its way to
shoehorn a nalgebra vector into a color. This made the color buffer code use
color.x, for example, to set red, which is off-putting.

I'm not sure that I'll have much need for *getting* a viewport size. I'm also not
sure how much is gained by using a Viewport object to store the state of the viewport
when it can be retrieved using OpenGL (the source of truth). Getting the viewport size
from OpenGL is a bit messy, but the retrieval can be a standalone wrapper function
without holding onto my own object state.

So, I'm replacing the suggested ColorBuffer with a plain color that just has
red, green, blue, and alpha fields. I'm removing the Viewport object altogether
and just going with the OpenGL versions.

My problem with writing too many wrappers is that I find if I get too disconnected
from what I'm wrapping, I'm not learning the wrapped library. I don't gain much knowledge
of the wrapped library, and maintaining the wrapper tends to be a pain as well. So,
I try to wrap sparingly.
