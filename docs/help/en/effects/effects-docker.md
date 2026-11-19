# The Effects panel

An object can carry several live effects at once, applied one after the
other. The [[docker.effects]] panel lists the effects of the selected object in
the order they apply, from the top down. Open it with
[[menu.window > menu.window.dockers > menu.window.dockers_effects]].

![The Effects panel](../../img/effects-effects-docker.png)

## Managing effects {#stack}

Each row of the list has:

- a check box that turns the effect on or off. An effect turned off keeps its
  place and settings, but is not drawn, exported or printed;
- the effect's name, dimmed while it is off;
- up and down arrows that move the effect earlier or later, which changes the
  result (for example a contour of an envelope, or an envelope of a contour);
- a cross that removes the effect.

Every change is one undo step. Each effect is still edited with its own tool
or panel.

Under the list, the built-in looks give the selected objects a finished
look with one click: chrome, liquid metal, gold, liquid glass (which blurs
what lies beneath it) and clear glass. Each picture shows the selected object
wearing the look; one undo removes it. Your own saved presets come after them.

## Halftone

In the Layer effects list, **Halftone** turns the selected object, as it
looks with its other effects (an inner shadow for a rounded tube look, a
glow, an overlay), into a screen of dots, and stays live: change the object
or its other effects and the dots follow.

- **Dot**: round, square, diamond or line.
- **Color mode**: RGB light makes red, green and blue light dots, as large
  as each color channel, for a dotted look on a dark background; CMYK ink
  prints cyan, magenta, yellow and black dots on white; One color draws dots
  of the color beside it, as large as the darkness.
- **Dot spacing**, **Dot scale** (at 100 % a full tone covers its cell),
  **Softness** of the dot edges and **Angle** of the screen.
- **Center X** and **Center Y** place a dot across the object's width and
  height.
- **Cut by the object's edge**: on, the dots end at the edge; off, whole
  dots, smaller where the object covers less of them.

An inner shadow in the Layer effects list also has **Spread**: it keeps
the shadow solid over that share of its blur before it fades, for a
deeper, rounder edge.

## Other effect commands {#commands}

- [[menu.effects > menu.effects.flatten]] turns the effects of the selected
  objects into plain objects.
- [[menu.effects.picker]] shows a list of effects to apply to the selection.
- [[menu.object > menu.object.clear_effect]] removes the effect of the
  selection.
- [[menu.object > menu.object.copy_effect]] copies an effect from another
  object, and [[menu.object.clone_effect]] makes a clone that follows it; see
  [Copying and cloning effects](copy-and-clone.md).
