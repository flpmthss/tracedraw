# 3D Revolve and Inflate

Effects > 3D Revolve and Inflate opens a panel that turns the first selected
object (not a picture) into a shaded 3D look made of vector shapes. Apply
adds the result as one group on top, deletes the object unless Keep
original is on, selects the group, all in one undo step ("3D"). The shaded
colour is the object's solid fill, else its outline colour, else a mid
grey.

## Settings

| Control | Default | Range | Meaning |
|---|---|---|---|
| Mode | Revolve | Revolve, Inflate | Below |
| Facets | 36 | 3 to 256 | Revolve: facets round the axis |
| Tilt | 20° | -80 to 80° | Revolve: the view looks down on the object by this much |
| Turn | 0° | 0 to 360° | Revolve: the object turned round its axis |
| Sweep | 360° | 10 to 360° | Revolve: how far round the profile is spun, centred on the front |
| Axis on the right | off | | Revolve: the axis at the profile's right edge, not its left |
| Depth | 60 % | 10 to 200 % | Inflate: the dome's height in percent of the largest distance from the outline inward |
| Light direction | 135° | 0 to 360° | Where the light comes from on the page, counter-clockwise from the right |
| Light height | 45° | 0 to 90° | The light's elevation over the page |
| Shine | 40 | 0 to 100 | Highlight strength (Blinn highlight, power 40) |
| Art | none | | Use selection picks the selected objects as art |
| Art across | 50 % | 0 to 100 % | Revolve: the art's centre round the way (50 in front); Inflate: across the shape's bounds |
| Art up | 50 % | 0 to 100 % | Revolve: along the profile's length; Inflate: up the shape's bounds |
| Art size | 30 % | 1 to 100 % | The art's width: of the way round (Revolve) or of the shape's width (Inflate) |
| Keep original | off | | Keep the object under the result |

Shading: brightness `0.28 + 0.72 max(0, n·l)` times the colour, plus the
highlight `shine max(0, n·h)^40`, with `l` toward the light and `h` halfway
between it and the viewer.

## Revolve

The first subpath of the object's outline (flattened) is the profile; the
axis is the vertical line through its left (or right) edge. Each profile
segment and each of the Facets steps round the axis make a four-sided facet
(a triangle where it touches the axis), drawn back to front as a filled
curve with a hairline outline of its own colour. Facets seen from inside
(an open cup) are lit on the side seen. Art is flattened, laid on the
surface (its width as a share of the way round at the profile's mean
radius, its height keeping its proportions along the profile; all the way
round it wraps past the back) and drawn on top in its own colour, shaded
where it lies; the parts facing away are left out.

## Inflate

A height field over a grid of 120 cells across the object's longer side:
inside the outline (even-odd), the distance to the outline `d`, the height
`top sqrt(1 - (1 - d/dmax)^2)` with `top` = Depth × `dmax`. Each cell is
shaded by its slope; cells of the same shade (64 steps a channel) in a row
merge into one band, a little larger than its cells so no seam shows. The
bands go in a clip frame shaped like the object. Art is placed on the
shape's bounds, swells away from its centre with the dome (up to 20 %) and
is shaded by the slope under its middle.

## Checks

- `shape3d::tests::revolve_makes_facets_lit_from_the_light`
- `shape3d::tests::revolve_art_shows_only_on_the_near_side`
- `shape3d::tests::inflate_is_highest_in_the_middle`
- `shape3d::tests::applying_revolve_and_inflate_is_one_step_each`
