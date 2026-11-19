//! Effects > 3D Revolve and Inflate: the selected curve turned into a
//! shaded 3D look made of vector shapes.
//!
//! - Revolve spins the curve (a profile) round a vertical axis at its
//!   left or right edge, seen tilted and turned, lit from one direction:
//!   facets drawn back to front. Picked objects (art) can be wrapped onto
//!   the surface, following its curve and shading, hidden round the back.
//! - Inflate puffs the shape up like a cushion: a dome over the distance
//!   to its outline, lit the same way, drawn as shaded bands clipped by
//!   the shape; art laid on it swells with the dome.
//!
//! The result is one group in one undo step.

use tracedraw_core::document::{Shape, ShapeKind};
use tracedraw_core::geometry::{BezPath, PathEl, Point, Rect, Shape as _, Vec2};
use tracedraw_core::{Color, Command, Fill, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Revolve,
    Inflate,
}

/// The 3D docker's settings (angles in degrees).
#[derive(Debug, Clone, PartialEq)]
pub struct Settings3d {
    pub mode: Mode,
    /// Revolve: facets round the axis.
    pub segments: u32,
    /// Revolve: how far the view looks down on the object.
    pub tilt: f64,
    /// Revolve: the object's turn round its axis.
    pub turn: f64,
    /// Revolve: how far round the profile is spun.
    pub sweep: f64,
    /// Revolve: the axis at the right edge of the profile, not the left.
    pub axis_right: bool,
    /// Inflate: the dome's height in percent of the shape's half width.
    pub depth: f64,
    /// Where the light comes from on the page (135 upper left).
    pub light_angle: f64,
    /// How high the light stands over the page.
    pub light_height: f64,
    /// Highlight strength, 0 to 100.
    pub shine: f64,
    /// Objects to wrap onto the surface.
    pub art: Vec<Shape>,
    /// The art's centre: percent round, 50 in front (Revolve), or across
    /// (Inflate).
    pub art_u: f64,
    /// The art's centre: percent along the profile (Revolve) or up
    /// (Inflate).
    pub art_v: f64,
    /// The art's width in percent of the way round (Revolve) or of the
    /// shape's width (Inflate).
    pub art_size: f64,
    pub keep_original: bool,
}

impl Default for Settings3d {
    fn default() -> Self {
        Settings3d {
            mode: Mode::Revolve,
            segments: 36,
            tilt: 20.0,
            turn: 0.0,
            sweep: 360.0,
            axis_right: false,
            depth: 60.0,
            light_angle: 135.0,
            light_height: 45.0,
            shine: 40.0,
            art: Vec::new(),
            art_u: 50.0,
            art_v: 50.0,
            art_size: 30.0,
            keep_original: false,
        }
    }
}

type V3 = [f64; 3];

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn norm(a: V3) -> V3 {
    let l = dot(a, a).sqrt();
    if l < 1e-12 {
        [0.0, 0.0, 1.0]
    } else {
        [a[0] / l, a[1] / l, a[2] / l]
    }
}

/// The light: a unit vector toward it, and the highlight's half vector
/// with the viewer straight in front (+z).
fn light(s: &Settings3d) -> (V3, V3) {
    let (a, e) = (s.light_angle.to_radians(), s.light_height.to_radians());
    let l = norm([e.cos() * a.cos(), e.cos() * a.sin(), e.sin()]);
    (l, norm([l[0], l[1], l[2] + 1.0]))
}

/// `base` lit on a surface facing `n` (toward the viewer, +z).
fn shade(base: Color, n: V3, s: &Settings3d) -> Color {
    let (l, h) = light(s);
    let diffuse = dot(n, l).max(0.0);
    let k = 0.28 + 0.72 * diffuse;
    let spec = s.shine / 100.0 * dot(n, h).max(0.0).powi(40);
    let [r, g, b] = base.to_rgb8();
    let ch = |c: u8| ((c as f64 / 255.0 * k + spec).clamp(0.0, 1.0) * 255.0).round() as u8;
    Color::rgb8(ch(r), ch(g), ch(b))
}

/// A shaded piece of the result: a closed outline filled with `color`.
#[derive(Debug, Clone)]
pub struct Facet {
    pub path: BezPath,
    pub color: Color,
}

fn polygon(pts: &[Point]) -> BezPath {
    let mut p = BezPath::new();
    for (i, q) in pts.iter().enumerate() {
        if i == 0 {
            p.move_to(*q);
        } else {
            p.line_to(*q);
        }
    }
    p.close_path();
    p
}

/// The first subpath of `path` as points, curves flattened.
fn profile_points(path: &BezPath) -> Vec<Point> {
    let mut pts: Vec<Point> = Vec::new();
    let mut started = false;
    let mut done = false;
    tracedraw_core::geometry::flatten(path, 0.05, &mut |el| {
        if done {
            return;
        }
        match el {
            PathEl::MoveTo(q) => {
                if started {
                    done = true;
                } else {
                    started = true;
                    pts.push(q);
                }
            }
            PathEl::LineTo(q) => {
                if pts.last().is_none_or(|l| l.distance(q) > 1e-6) {
                    pts.push(q);
                }
            }
            PathEl::ClosePath => {
                if let Some(f) = pts.first().copied() {
                    pts.push(f);
                }
                done = true;
            }
            _ => {}
        }
    });
    pts
}

/// The revolved surface's geometry: the profile as (radius, height)
/// points with their fraction of its length, the view and the placement.
struct Lathe {
    rv: Vec<(f64, f64)>,
    along: Vec<f64>,
    axis: f64,
    mid: f64,
    tilt: f64,
    turn: f64,
    sweep: f64,
}

impl Lathe {
    fn new(profile: &[Point], s: &Settings3d) -> Option<Lathe> {
        if profile.len() < 2 {
            return None;
        }
        let x0 = profile.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
        let x1 = profile
            .iter()
            .map(|p| p.x)
            .fold(f64::NEG_INFINITY, f64::max);
        let y0 = profile.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
        let y1 = profile
            .iter()
            .map(|p| p.y)
            .fold(f64::NEG_INFINITY, f64::max);
        let axis = if s.axis_right { x1 } else { x0 };
        let rv: Vec<(f64, f64)> = profile.iter().map(|p| ((p.x - axis).abs(), p.y)).collect();
        if rv.iter().all(|(r, _)| *r < 1e-6) {
            return None;
        }
        let mut along = vec![0.0];
        for w in profile.windows(2) {
            let l = along.last().copied().unwrap_or(0.0) + w[0].distance(w[1]);
            along.push(l);
        }
        let total = along.last().copied().unwrap_or(1.0).max(1e-9);
        along.iter_mut().for_each(|a| *a /= total);
        Some(Lathe {
            rv,
            along,
            axis,
            mid: (y0 + y1) / 2.0,
            tilt: s.tilt.to_radians(),
            turn: s.turn.to_radians(),
            sweep: s.sweep.clamp(1.0, 360.0).to_radians(),
        })
    }

    /// The surface point at radius `r`, height `y`, angle `phi`, in the
    /// view (x, y on the page round the axis; z toward the viewer).
    fn view(&self, r: f64, y: f64, phi: f64) -> V3 {
        let a = self.turn + phi;
        let (x, yy, z) = (r * a.sin(), y - self.mid, r * a.cos());
        let (st, ct) = self.tilt.sin_cos();
        [x, yy * ct - z * st, yy * st + z * ct]
    }

    fn page(&self, v: V3) -> Point {
        Point::new(self.axis + v[0], self.mid + v[1])
    }

    /// Radius and height `t` (0..1) of the way along the profile.
    fn at(&self, t: f64) -> (f64, f64) {
        let t = t.clamp(0.0, 1.0);
        let i = self
            .along
            .partition_point(|a| *a < t)
            .clamp(1, self.rv.len() - 1);
        let (a0, a1) = (self.along[i - 1], self.along[i]);
        let f = if a1 > a0 { (t - a0) / (a1 - a0) } else { 0.0 };
        let (p, q) = (self.rv[i - 1], self.rv[i]);
        (p.0 + (q.0 - p.0) * f, p.1 + (q.1 - p.1) * f)
    }

    /// The surface normal toward the viewer at `t` along and angle `phi`
    /// (None when it faces away).
    fn facing(&self, t: f64, phi: f64) -> Option<V3> {
        let e = 1e-3;
        let (r0, y0) = self.at(t - e);
        let (r1, y1) = self.at(t + e);
        let (r, y) = self.at(t);
        let a = self.view(r0, y0, phi);
        let b = self.view(r1, y1, phi);
        let c = self.view(r, y, phi - e);
        let d = self.view(r, y, phi + e);
        let mut n = norm(cross(sub(b, a), sub(d, c)));
        // Outward: away from the axis.
        let out = self.view(r + 1.0, y, phi);
        let p = self.view(r, y, phi);
        if dot(n, sub(out, p)) < 0.0 {
            n = [-n[0], -n[1], -n[2]];
        }
        (n[2] > 0.0).then_some(n)
    }
}

/// Revolve: the facets of `profile` spun round its axis, back to front,
/// shaded from `base`.
pub fn revolve(profile: &[Point], base: Color, s: &Settings3d) -> Vec<Facet> {
    let Some(lathe) = Lathe::new(profile, s) else {
        return Vec::new();
    };
    let n = s.segments.clamp(3, 256) as usize;
    let rings: Vec<Vec<V3>> = (0..=n)
        .map(|j| {
            let phi = lathe.sweep * (j as f64 / n as f64 - 0.5);
            lathe
                .rv
                .iter()
                .map(|(r, y)| lathe.view(*r, *y, phi))
                .collect()
        })
        .collect();
    let mut faces: Vec<(f64, Facet)> = Vec::new();
    for j in 0..n {
        for i in 0..lathe.rv.len() - 1 {
            let (a, b, c, d) = (
                rings[j][i],
                rings[j][i + 1],
                rings[j + 1][i + 1],
                rings[j + 1][i],
            );
            let mut nv = cross(sub(c, a), sub(d, b));
            if dot(nv, nv) < 1e-14 {
                // A quad with one edge on the axis: a triangle.
                nv = cross(sub(b, a), sub(d, a));
                if dot(nv, nv) < 1e-14 {
                    continue;
                }
            }
            let mut nv = norm(nv);
            // Seen from inside (an open cup), light the side we see.
            if nv[2] < 0.0 {
                nv = [-nv[0], -nv[1], -nv[2]];
            }
            let depth = (a[2] + b[2] + c[2] + d[2]) / 4.0;
            let pts: Vec<Point> = [a, b, c, d].iter().map(|v| lathe.page(*v)).collect();
            faces.push((
                depth,
                Facet {
                    path: polygon(&pts),
                    color: shade(base, nv, s),
                },
            ));
        }
    }
    faces.sort_by(|x, y| x.0.total_cmp(&y.0));
    faces.into_iter().map(|(_, f)| f).collect()
}

/// The art's subpaths flattened, each with its colour, and the bounds of
/// them all.
fn art_polygons(art: &[Shape]) -> (Vec<(Vec<Point>, Color)>, Rect) {
    let mut out = Vec::new();
    let mut bounds: Option<Rect> = None;
    for sh in art {
        let color = match &sh.fill {
            Fill::Solid(c) => *c,
            _ => sh.stroke.as_ref().map_or(Color::BLACK, |st| st.color),
        };
        let path = sh.page_path();
        let mut cur: Vec<Point> = Vec::new();
        tracedraw_core::geometry::flatten(&path, 0.05, &mut |el| match el {
            PathEl::MoveTo(q) => {
                if cur.len() > 2 {
                    out.push((std::mem::take(&mut cur), color));
                }
                cur = vec![q];
            }
            PathEl::LineTo(q) => cur.push(q),
            PathEl::ClosePath if cur.len() > 2 => {
                out.push((std::mem::take(&mut cur), color));
            }
            _ => {}
        });
        if cur.len() > 2 {
            out.push((cur, color));
        }
        let b = path.bounding_box();
        bounds = Some(bounds.map_or(b, |a| a.union(b)));
    }
    (out, bounds.unwrap_or(Rect::ZERO))
}

/// Points of a closed polygon, its edges cut so none is longer than
/// `step`.
fn densify(pts: &[Point], step: f64) -> Vec<Point> {
    let mut out = Vec::new();
    for i in 0..pts.len() {
        let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
        let k = ((a.distance(b) / step).ceil() as usize).clamp(1, 200);
        for j in 0..k {
            out.push(a.lerp(b, j as f64 / k as f64));
        }
    }
    out
}

/// Revolve's art: the picked objects wrapped onto the surface, the parts
/// facing away left out.
pub fn revolve_art(profile: &[Point], s: &Settings3d) -> Vec<Facet> {
    let Some(lathe) = Lathe::new(profile, s) else {
        return Vec::new();
    };
    let (polys, b) = art_polygons(&s.art);
    if polys.is_empty() || b.width() <= 0.0 || b.height() <= 0.0 {
        return Vec::new();
    }
    // The art's width as a share of the way round, its height as a share
    // of the profile's length keeping its proportions on the surface.
    let wu = (s.art_size / 100.0).clamp(0.01, 1.0);
    let length: f64 = profile.windows(2).map(|w| w[0].distance(w[1])).sum();
    let mean_r = lathe.rv.iter().map(|(r, _)| *r).sum::<f64>() / lathe.rv.len() as f64;
    let width_mm = wu * lathe.sweep * mean_r.max(1e-6);
    let hv = width_mm * b.height() / b.width() / length.max(1e-6);
    let (cu, cv) = (s.art_u / 100.0, s.art_v / 100.0);
    let step = b.width().max(b.height()) / 60.0;
    let mut out = Vec::new();
    for (poly, color) in polys {
        let mut pts = Vec::new();
        let mut normals = Vec::new();
        for q in densify(&poly, step) {
            let a = (q.x - b.x0) / b.width() - 0.5;
            let c = (q.y - b.y0) / b.height() - 0.5;
            let (mut u, v) = (cu + a * wu, cv + c * hv);
            // All the way round, the art wraps past the seam.
            if s.sweep >= 360.0 {
                u = u.rem_euclid(1.0);
            }
            if !(0.0..=1.0).contains(&v) || !(0.0..=1.0).contains(&u) {
                continue;
            }
            let phi = lathe.sweep * (u - 0.5);
            if let Some(n) = lathe.facing(v, phi) {
                let (r, y) = lathe.at(v);
                pts.push(lathe.page(lathe.view(r, y, phi)));
                normals.push(n);
            }
        }
        if pts.len() < 3 {
            continue;
        }
        let n = normals.iter().fold([0.0; 3], |acc, m| {
            [acc[0] + m[0], acc[1] + m[1], acc[2] + m[2]]
        });
        out.push(Facet {
            path: polygon(&pts),
            color: shade(color, norm(n), s),
        });
    }
    out
}

/// The flattened outlines of `path`, every subpath closed.
fn outlines(path: &BezPath) -> Vec<Vec<Point>> {
    let mut out = Vec::new();
    let mut cur: Vec<Point> = Vec::new();
    tracedraw_core::geometry::flatten(path, 0.05, &mut |el| match el {
        PathEl::MoveTo(q) => {
            if cur.len() > 1 {
                out.push(std::mem::take(&mut cur));
            }
            cur = vec![q];
        }
        PathEl::LineTo(q) => cur.push(q),
        PathEl::ClosePath if cur.len() > 1 => {
            out.push(std::mem::take(&mut cur));
        }
        _ => {}
    });
    if cur.len() > 1 {
        out.push(cur);
    }
    out
}

fn segment_distance(p: Point, a: Point, b: Point) -> f64 {
    let ab = b - a;
    let l2 = ab.hypot2();
    let t = if l2 > 0.0 {
        ((p - a).dot(ab) / l2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    p.distance(a + ab * t)
}

/// Inflate's height field over `path`: cells `cell` mm wide from the
/// bounds' corner, each the dome's height there (0 outside).
pub struct Dome {
    pub origin: Point,
    pub cell: f64,
    pub cols: usize,
    pub rows: usize,
    pub h: Vec<f64>,
    pub top: f64,
}

impl Dome {
    pub fn new(path: &BezPath, depth: f64, cells: usize) -> Option<Dome> {
        let b = path.bounding_box();
        if b.width() <= 0.0 || b.height() <= 0.0 {
            return None;
        }
        let cell = b.width().max(b.height()) / cells.max(4) as f64;
        let cols = (b.width() / cell).ceil() as usize + 1;
        let rows = (b.height() / cell).ceil() as usize + 1;
        let rings = outlines(path);
        let mut d = vec![0.0; cols * rows];
        for j in 0..rows {
            for i in 0..cols {
                let p = Point::new(
                    b.x0 + (i as f64 + 0.5) * cell,
                    b.y0 + (j as f64 + 0.5) * cell,
                );
                if path.winding(p) % 2 == 0 {
                    continue;
                }
                let mut m = f64::INFINITY;
                for r in &rings {
                    for k in 0..r.len() {
                        m = m.min(segment_distance(p, r[k], r[(k + 1) % r.len()]));
                    }
                }
                d[j * cols + i] = m;
            }
        }
        let dmax = d.iter().copied().fold(0.0, f64::max);
        if dmax <= 0.0 {
            return None;
        }
        let top = dmax * depth / 100.0;
        // A round cushion: a quarter circle over the distance inward.
        let h = d
            .iter()
            .map(|&x| {
                if x <= 0.0 {
                    0.0
                } else {
                    let t = 1.0 - x / dmax;
                    top * (1.0 - t * t).max(0.0).sqrt()
                }
            })
            .collect();
        Some(Dome {
            origin: Point::new(b.x0, b.y0),
            cell,
            cols,
            rows,
            h,
            top,
        })
    }

    fn get(&self, i: isize, j: isize) -> f64 {
        if i < 0 || j < 0 || i as usize >= self.cols || j as usize >= self.rows {
            0.0
        } else {
            self.h[j as usize * self.cols + i as usize]
        }
    }

    /// The surface normal (toward the viewer) of cell (i, j).
    pub fn normal(&self, i: usize, j: usize) -> V3 {
        let (i, j) = (i as isize, j as isize);
        let dx = (self.get(i + 1, j) - self.get(i - 1, j)) / (2.0 * self.cell);
        let dy = (self.get(i, j + 1) - self.get(i, j - 1)) / (2.0 * self.cell);
        norm([-dx, -dy, 1.0])
    }

    /// The height at page point `p` (0 outside).
    pub fn height_at(&self, p: Point) -> f64 {
        let i = ((p.x - self.origin.x) / self.cell).floor() as isize;
        let j = ((p.y - self.origin.y) / self.cell).floor() as isize;
        self.get(i, j)
    }
}

/// Inflate: shaded bands (runs of cells of one shade in each row) over
/// `path`, to be clipped by it.
pub fn inflate(dome: &Dome, base: Color, s: &Settings3d) -> Vec<Facet> {
    // Shades are rounded to so many steps so runs of cells merge.
    const STEPS: f32 = 64.0;
    let q = |c: Color| {
        let step = (256.0 / STEPS) as u32;
        let [r, g, b] = c.to_rgb8();
        let f = |v: u8| ((v as u32 + step / 2) / step * step).min(255) as u8;
        Color::rgb8(f(r), f(g), f(b))
    };
    let mut out = Vec::new();
    let c = dome.cell;
    for j in 0..dome.rows {
        let mut run: Option<(usize, Color)> = None;
        for i in 0..=dome.cols {
            let here = (i < dome.cols).then(|| {
                // Cells just outside take their neighbour's shade so the
                // clip, not the grid, makes the edge.
                let (ii, jj) = (i as isize, j as isize);
                let inside = |a: isize, b: isize| dome.get(a, b) > 0.0;
                if inside(ii, jj) {
                    Some(q(shade(base, dome.normal(i, j), s)))
                } else {
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .find(|(a, b)| inside(ii + a, jj + b))
                        .map(|(a, b)| {
                            q(shade(
                                base,
                                dome.normal((ii + a) as usize, (jj + b) as usize),
                                s,
                            ))
                        })
                }
            });
            let here = here.flatten();
            match (run, here) {
                (Some((_, rc)), Some(hc)) if rc == hc => {}
                _ => {
                    if let Some((start, rc)) = run.take() {
                        let x0 = dome.origin.x + start as f64 * c;
                        let x1 = dome.origin.x + i as f64 * c;
                        let y0 = dome.origin.y + j as f64 * c;
                        // Overlapping the next cells, so no seam shows
                        // between bands (later bands cover the overlap).
                        let r = Rect::new(x0 - c * 0.3, y0 - c * 0.3, x1 + c * 0.3, y0 + c * 1.3);
                        out.push(Facet {
                            path: polygon(&[
                                Point::new(r.x0, r.y0),
                                Point::new(r.x1, r.y0),
                                Point::new(r.x1, r.y1),
                                Point::new(r.x0, r.y1),
                            ]),
                            color: rc,
                        });
                    }
                    if let Some(hc) = here {
                        run = Some((i, hc));
                    }
                }
            }
        }
    }
    out
}

/// Inflate's art: the picked objects laid on the dome, swelling with it
/// away from their centre and shaded where they lie.
pub fn inflate_art(dome: &Dome, bounds: Rect, s: &Settings3d) -> Vec<Facet> {
    let (polys, b) = art_polygons(&s.art);
    if polys.is_empty() || b.width() <= 0.0 || b.height() <= 0.0 {
        return Vec::new();
    }
    let w = bounds.width() * (s.art_size / 100.0).clamp(0.01, 1.0);
    let k = w / b.width();
    let centre = Point::new(
        bounds.x0 + bounds.width() * s.art_u / 100.0,
        bounds.y0 + bounds.height() * s.art_v / 100.0,
    );
    let step = b.width().max(b.height()) / 60.0;
    let mut out = Vec::new();
    for (poly, color) in polys {
        let pts: Vec<Point> = densify(&poly, step)
            .into_iter()
            .map(|q| {
                let p = centre + (q - b.center()) * k;
                let lift = if dome.top > 0.0 {
                    dome.height_at(p) / dome.top
                } else {
                    0.0
                };
                centre + (p - centre) * (1.0 + 0.2 * lift)
            })
            .collect();
        let mid = pts.iter().fold(Vec2::ZERO, |a, p| a + p.to_vec2()) / pts.len().max(1) as f64;
        let mp = mid.to_point();
        let i = ((mp.x - dome.origin.x) / dome.cell).floor().max(0.0) as usize;
        let j = ((mp.y - dome.origin.y) / dome.cell).floor().max(0.0) as usize;
        let n = if i < dome.cols && j < dome.rows {
            dome.normal(i, j)
        } else {
            [0.0, 0.0, 1.0]
        };
        out.push(Facet {
            path: polygon(&pts),
            color: shade(color, n, s),
        });
    }
    out
}

/// The colour an object is shaded from: its solid fill, else its
/// outline's colour, else a mid grey.
fn base_color(sh: &Shape) -> Color {
    match &sh.fill {
        Fill::Solid(c) => *c,
        _ => sh
            .stroke
            .as_ref()
            .map_or(Color::rgb8(0x9A, 0xA4, 0xB0), |st| st.color),
    }
}

impl crate::app::App {
    /// Use Selection for the 3D art: the selected objects with an outline.
    pub fn pick_3d_art(&mut self) -> bool {
        let picked: Vec<Shape> = self
            .selected_shapes()
            .into_iter()
            .filter(|sh| !sh.page_path().is_empty())
            .collect();
        if picked.is_empty() {
            return false;
        }
        self.shape_3d.art = picked;
        true
    }

    /// Apply 3D Revolve or Inflate to the first selected object: the
    /// result one group on top (the object removed unless Keep original),
    /// selected, in one undo step.
    pub fn apply_3d(&mut self, s: &Settings3d) -> bool {
        let Some(layer) = self.active_layer() else {
            return false;
        };
        let Some(src) = self
            .selected_shapes()
            .into_iter()
            .find(|sh| !matches!(sh.kind, ShapeKind::Bitmap { .. }) && !sh.page_path().is_empty())
        else {
            self.status = crate::i18n::tr("status.shape3d_select");
            return false;
        };
        let path = src.page_path();
        let base = base_color(&src);
        let make = |f: Facet, seam: bool, engine: &mut tracedraw_core::Engine| {
            let mut sh = Shape::new(
                engine.new_shape_id(),
                ShapeKind::Path {
                    path: f.path,
                    closed: true,
                },
            );
            sh.fill = Fill::Solid(f.color);
            // A hairline of the facet's own colour hides the seams
            // between facets.
            sh.stroke = seam.then(|| Stroke::new(f.color, 0.06));
            sh
        };
        let children: Vec<Shape> = match s.mode {
            Mode::Revolve => {
                let profile = profile_points(&path);
                let mut facets = revolve(&profile, base, s);
                if facets.is_empty() {
                    self.status = crate::i18n::tr("status.shape3d_flat");
                    return false;
                }
                let art = revolve_art(&profile, s);
                let mut out: Vec<Shape> = facets
                    .drain(..)
                    .map(|f| make(f, true, &mut self.engine))
                    .collect();
                out.extend(art.into_iter().map(|f| make(f, false, &mut self.engine)));
                out
            }
            Mode::Inflate => {
                let Some(dome) = Dome::new(&path, s.depth, 120) else {
                    self.status = crate::i18n::tr("status.shape3d_flat");
                    return false;
                };
                let bands: Vec<Shape> = inflate(&dome, base, s)
                    .into_iter()
                    .map(|f| make(f, false, &mut self.engine))
                    .collect();
                let mut frame = Shape::new(
                    self.engine.new_shape_id(),
                    ShapeKind::Path {
                        path: path.clone(),
                        closed: true,
                    },
                );
                frame.fill = Fill::None;
                frame.stroke = None;
                let clip = Shape::new(
                    self.engine.new_shape_id(),
                    ShapeKind::ClipFrame {
                        frame: Box::new(frame),
                        contents: bands,
                    },
                );
                let mut out = vec![clip];
                out.extend(
                    inflate_art(&dome, path.bounding_box(), s)
                        .into_iter()
                        .map(|f| make(f, false, &mut self.engine)),
                );
                out
            }
        };
        let group_id = self.engine.new_shape_id();
        let mut group = Shape::new(group_id, ShapeKind::Group { children });
        group.name = Some(crate::i18n::tr(match s.mode {
            Mode::Revolve => "docker.shape3d_revolve",
            Mode::Inflate => "docker.shape3d_inflate",
        }));
        let mut cmds = Vec::new();
        if !s.keep_original {
            cmds.push(Command::DeleteShapes {
                shapes: vec![src.id],
            });
        }
        cmds.push(Command::AddShape {
            layer,
            shape: group,
        });
        if let Err(e) = self.engine.run_batch("3D", &cmds) {
            self.status = e.to_string();
            return false;
        }
        self.selection = vec![group_id];
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A vase-like profile right of the axis at x = 0.
    fn vase() -> Vec<Point> {
        vec![
            Point::new(0.0, 0.0),
            Point::new(10.0, 0.0),
            Point::new(14.0, 12.0),
            Point::new(8.0, 24.0),
            Point::new(9.0, 30.0),
            Point::new(0.0, 30.0),
        ]
    }

    #[test]
    fn revolve_makes_facets_lit_from_the_light() {
        let s = Settings3d {
            tilt: 0.0,
            segments: 24,
            ..Default::default()
        };
        let f = revolve(&vase(), Color::rgb8(200, 200, 200), &s);
        // Five profile pieces round 24 facets, the two on the axis caps
        // as triangles.
        assert_eq!(f.len(), 5 * 24);
        // The body spans twice the widest radius, centred on the axis.
        let b = f
            .iter()
            .map(|x| x.path.bounding_box())
            .reduce(|a, b| a.union(b))
            .expect("facets");
        assert!(
            (b.x0 + 14.0).abs() < 0.5 && (b.x1 - 14.0).abs() < 0.5,
            "{b:?}"
        );
        // Lit from the upper left: the left side is brighter than the right.
        let lum = |x: f64| {
            f.iter()
                .rev()
                .find(|fc| fc.path.bounding_box().contains(Point::new(x, 6.0)))
                .map(|fc| fc.color.to_rgb8()[0])
                .expect("facet")
        };
        assert!(lum(-9.0) > lum(9.0), "{} {}", lum(-9.0), lum(9.0));
    }

    #[test]
    fn revolve_art_shows_only_on_the_near_side() {
        let mut art = Shape::new(
            tracedraw_core::ShapeId(1),
            ShapeKind::Rect {
                rect: Rect::new(0.0, 0.0, 10.0, 10.0),
                radius: 0.0,
                corners: None,
            },
        );
        art.fill = Fill::Solid(Color::rgb8(255, 0, 0));
        let front = Settings3d {
            tilt: 0.0,
            art: vec![art.clone()],
            art_u: 50.0,
            ..Default::default()
        };
        let a = revolve_art(&vase(), &front);
        assert_eq!(a.len(), 1);
        let b = a[0].path.bounding_box();
        // Centred in front of the axis, a third of the way round wide at
        // most, so narrower than the body.
        assert!(b.center().x.abs() < 1.5 && b.width() < 26.0, "{b:?}");
        // Turned to the back, nothing shows.
        let back = Settings3d {
            art_u: 0.0,
            ..front
        };
        assert!(revolve_art(&vase(), &back).is_empty());
    }

    #[test]
    fn inflate_is_highest_in_the_middle() {
        let mut p = BezPath::new();
        p.move_to((0.0, 0.0));
        p.line_to((40.0, 0.0));
        p.line_to((40.0, 20.0));
        p.line_to((0.0, 20.0));
        p.close_path();
        let d = Dome::new(&p, 60.0, 40).expect("dome");
        let mid = d.height_at(Point::new(20.0, 10.0));
        assert!((mid - d.top).abs() < d.top * 0.1, "{mid} {}", d.top);
        assert!(d.height_at(Point::new(1.0, 10.0)) < mid * 0.6);
        assert_eq!(d.height_at(Point::new(-5.0, 10.0)), 0.0);
        let s = Settings3d::default();
        let bands = inflate(&d, Color::rgb8(80, 140, 220), &s);
        assert!(!bands.is_empty());
        // Far fewer bands than cells: runs of one shade merge.
        assert!(bands.len() < d.cols * d.rows / 2, "{}", bands.len());
    }

    #[test]
    fn applying_revolve_and_inflate_is_one_step_each() {
        let mut app = crate::app::App::headless();
        let id = app
            .new_shape(ShapeKind::Ellipse {
                rect: Rect::new(20.0, 20.0, 60.0, 50.0),
                arc: None,
            })
            .expect("ellipse");
        for mode in [Mode::Revolve, Mode::Inflate] {
            app.select(vec![id]);
            let depth = app.engine.history_labels().0.len();
            let s = Settings3d {
                mode,
                ..Default::default()
            };
            assert!(app.apply_3d(&s), "{mode:?}");
            assert_eq!(app.engine.history_labels().0.len(), depth + 1);
            let g = app.selection[0];
            assert!(matches!(
                app.doc().find_shape(g).expect("result").kind,
                ShapeKind::Group { .. }
            ));
            assert!(app.doc().find_shape(id).is_none());
            app.undo();
            assert!(app.doc().find_shape(id).is_some());
        }
    }
}
