# 3D Revolve and Inflate

3D Revolve and Inflate gives a flat drawing a shaded 3D look made of
ordinary vector shapes: spin a profile into a vase or a bottle with your
label wrapped round it, or puff a shape up like a cushion.

## Making a 3D object {#make}

Open the [[docker.shape3d]] panel with
[[menu.effects > menu.effects.shape3d]].

1. Select a curve or shape. Its fill colour (or outline colour) is the
   colour that gets shaded.
2. Choose the mode:
   - [[docker.shape3d_revolve]] spins the curve round a vertical axis at its
     left edge, like a potter's wheel: draw half the outline of a vase,
     bottle or glass. Set the [[docker.shape3d_segments]],
     [[docker.shape3d_tilt]] (how far you look down on it),
     [[docker.shape3d_turn]], [[docker.shape3d_sweep]] (less than 360 cuts it
     open) and [[docker.shape3d_axis_right]].
   - [[docker.shape3d_inflate]] puffs the shape up; [[docker.shape3d_depth]]
     sets how high.
3. Set the light: [[docker.shape3d_light_angle]],
   [[docker.shape3d_light_height]] and [[docker.shape3d_shine]].
4. To wrap art onto the surface, select the art (a logo, a label, text
   converted to curves) and click [[docker.shape3d_art]], then select the
   curve again. Place it with [[docker.shape3d_art_u]] (50 is the front),
   [[docker.shape3d_art_v]] and [[docker.shape3d_art_size]]. Art turned to
   the back is hidden.
5. Click [[docker.apply]]. [[docker.shape3d_keep]] keeps the original curve.

The result is one group of shapes, selected, and one undo removes it.
