# Layer effects

Effects > Layer Effects opens the Effects docker, whose top section,
Layer effects (#1141), stacks looks on the selected objects and sets each
one with its values drawn live on the page: there is no OK or Cancel,
every change is one undo step ("Layer Effects") and goes to every
selected object; the values shown are the first selected object's. Code:
`ui/layer_effects.rs`; the effects in `tracedraw_core::live::Effect`, the
pixels in `tracedraw_render` (`draw_outer_shadows`, `draw_inner_shadow`,
`draw_blurred`).

The list, one row each:

| Entry | Effect | Values |
|---|---|---|
| Bevel / Emboss | `Bevel` (Soft edge or Emboss) | type, depth, level, light angle, intensity, light and shadow colors |
| Outline | `Contour`, one step outside | width, color |
| 3D | `Extrude`, back parallel | distance and direction of the depth, light angle and intensity |
| Inner shadow | `InnerShadow` | distance, angle, blur, opacity, color, blend mode |
| Outer shadow | `OuterShadow`: a drop shadow of the object's shape beneath it and all its effects draw; any number beside the object's own drop shadow | as Inner shadow |
| Inner glow | `Glow` inside: an inner shadow that does not move | radius, opacity, color, blend mode |
| Outer glow | `Glow` outside: a drop shadow that does not move, solid up to the edge and fading beyond it (feather direction Outside) | as Inner glow |
| Color overlay | `ColorOverlay`: the object's shape over it in one color | color, opacity |
| Gradient overlay | `GradientOverlay`: the shape over it with a linear gradient | from and to colors, angle, opacity |
| Gaussian blur | `GaussianBlur`: the object drawn on a layer of its own and blurred (three box passes); with pins, a variable blur (below) | radius, pins |
| Grain | `Grain`: noise over the object as it is drawn, after its blur | amount, grain size, monochrome, New grain |

- The check box switches the entry's effect on. Ticked the first time it
  adds the effect with the values shown; cleared, the effect is kept
  switched off at its place in the stack (as the stack's own switch
  below) and comes back with its values when ticked again.
- Clicking the name shows that entry's values under the list. Changing a
  value of an entry that is not on switches it on with that value.
- The cross takes the effect off the objects.
- Defaults: bevel as the Bevel docker's with level 4; outline 1 mm black;
  3D 2 mm down and right; inner shadow 1 mm at 315°, 50 %, blur 1 mm,
  Multiply; outer shadow 2.1 mm at 315°, 50 %, blur 1.5 mm, Multiply;
  glows pale yellow, 75 %, Screen, 1.5 mm inside and 2 mm outside;
  color overlay red at 50 %; gradient overlay black to white at 90°,
  50 %; blur 1 mm; grain 25 %, 0.1 mm, monochrome.

Variable blur: Add blur pin, under the blur's radius, puts a pin on the
object (the first in its middle with the blur's radius, the next ones
round it with none); each pin has its own radius field and a cross. The
radius at any point is each pin's radius weighed by the inverse square
of the distance to it, so it is the pin's own on a pin and changes
smoothly between pins. The renderer blurs the object's layer at up to
six radii from none to the widest pin's and gives each pixel a blend of
the two nearest its radius (`blur_by_pins`; the radii are worked out on
a grid every 4 pixels). Pins are kept in the object's own coordinates,
so they move, scale and turn with it, and they are effect nodes of the
Shape tool (110 + k, `interaction2::effect_nodes`): round handles with a
thin ring as wide as the pin's radius, dragged on the page in one undo
step. At most 64 pins.

Grain: a smooth value noise in grains `size` mm across, fixed to the
page (it does not crawl as the view pans or zooms), added to each drawn
pixel by up to half its alpha at 100 %; monochrome puts the same noise
in the three channels, otherwise each has its own. New grain changes the
seed. A pattern or texture over the object with a merge mode stays the
transparency's job (pattern or texture transparency, 28 merge modes).

Fill opacity, under the list, fades the object itself (`FillOpacity`,
which multiplies the object's opacity after the other effects took it as
it was): overlays, shadows and glows keep their own. At 100 % the effect
goes. An outer glow is solid under the object, so with the fill faded it
shows through.

The renderer draws outer shadows and glows, inner glows and the blur;
the vector exports (PDF, SVG) draw the overlays, as objects, and leave
the pixel effects out, as they do the inner shadow. A page's bands are
drawn with room for the widest of these to meet (`reach_walk`). Presets
keep all of them (`style_presets::portable`).

Tests: `live::tests::layer_effect_overlays_and_fill_opacity`,
`tests::layer_effects_shadow_glow_and_blur_the_object` (render),
`ui::layer_effects::tests::entries_switch_set_and_remove_their_effects`,
`tests::blur_pins_vary_the_blur_and_grain_marks_the_object` (render),
`live::symmetry_tests::blur_pins_and_grain_round_trip`,
`ui::layer_effects::tests::blur_pins_are_dragged_on_the_page`.
