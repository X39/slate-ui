# The Project
slate-ui is a software library that aims to provide a proper
solution for desktop graphical user interfaces built in `rust`.

The current version is pre-alpha and not ready for any use.
# The Theory
The basics of UI are simple:
- You have a set canvas of size `width` x `height`
- You have a bunch of buttons, labels, textboxes, etc. you want to draw
- You have a bunch of events you want to handle

But how to translate that best into code?
The general "agreed" way is either clunky software built
stuff, where all control is mandated by the developer or
with implicit flow, based on inheritance and callbacks.

With rust not having inheritance, the second option is out.
However, the first is also out, as it is, simply put, bad at
every level.

This is where slate-ui comes in. It aims to provide a
middleground that marries the pros of object-oriented ui
frameworks with the rather data driven nature of rust.

## The Rendering
The rendering is generally agreed to be a three step process:
1. Measure what you would like to have, size-wise
2. Arrange yourself in the space you will be given
3. Render yourself

This is simple enough to actually do, but with rust lacking
inheritance, doing rendering can be troublesome when everyone
has to implement eg. margin and padding themselves.
To solve this, slate-ui approaches this pipeline slightly
different:

### Render

We start off with a blank slate, called `Control`. This
control has nothing but a list of "Primitives".
A Primitive is a simple rendering construct that has follows
the already mentioned three steps. The catch here is:
all primitives are added up and have a priority. This
priority is used to determine the order in which they are
rendered.
The render step itself is split into three parts then:
pre-render, render and post-render. The pre-render step
is used to allow a primitive to do some pre-rendering
setup, like setting up a clipping region. The render step
is used to actually render the primitive. The post-render
step is used to clean up after the rendering or add some
overlay (like a border).
The primitives are then rendered in order of their priority,
having every step being called in the render order.

This allows for a very flexible rendering pipeline, where
every primitive can do whatever it wants, but still have
a clear way to do most shenanigans one would want to do.

### Measuring and Arranging
Just like rendering, measuring and arranging is done in
the common way but with a twist that is easier explained
using a sample: A scroll panel.

Imagine you have a scroll panel that has a height of 100px
and want to render a list of items in it.
The list of items is 200px high (note that this for now
assumes that the list is fully rendered, not virtualized).
To now render the list, we have to measure it and the
scroll panel.
The scroll panel has a scroll bar, potentially, and the
"child" to render (more on that later).
Ignoring the fact that this could be solved using two
controls, we now have to render our list and the scroll
bar in 100px height. To do this, the measure step
will first take the lowest priority child and measure it,
passing the measured size to the next child, allowing it to
append its size to the passed size. In our sample, the
list returns 200px height, which is then passed to the
scroll bar, which returns 100px height (the actual act of
rendering can later be resolved using clipping regions).
This effectively allows for a very simple way to change
the measure of a control by different composition parts.

Similar to the measure step, the arrange step will take
the measured size and only change the direction of the
passing of the size. This allows for a very simple way
to arrange controls in a space, limited only by the
imagination of the developer.

### State
While technically speaking, rendering is a three step process,
it often is overlooked that there is a step 0: Reseting the
state.

slate-ui is not stateless for its rendering! It is stateful,
using the structures to store the state of rendering.
This essentially brings back in the object-oriented way
of doings things, making it easier to reason about the
ui system.

But let us quickly elaborate on the decision to make the 
primitives stateful and thus not immutable:
Rendering is a stateful process. You have to keep track
of what you have rendered and what you will render.
While technically speaking, you could make the primitives
immutable, creating a full new tree for every step,
doing so would make the rendering process rather complicated.
This is why slate-ui opts for a stateful approach, where
the primitives are mutable and can change their state
between the render steps.

This may feel counter-intuitive for rust developers, but
it is the way used by non-rust ui frameworks for a reason.