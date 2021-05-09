//! A set of tools for solid modeling based on [functional representation](https://en.wikipedia.org/wiki/Function_representation).
//! Particulalry suited for parametric- and procedural design.
//!
//! It is infrastructure for generative design, mass customization, and
//! domain-specific CAD tools.
//!
//! ## Example
//!
//! Todo
//!
//! ## Features
//!
//! * [`ahash`](https://crates.io/crates/ahash) – On by default. Use [`AHashMap`](https://docs.rs/ahash/latest/ahash/struct.AHashMap.html)
//!   for hashing when reading files and merging vertices. To disable and use
//!   the slower [`HashMap`](std::collections::HashMap) instead, unset default
//!
//! * `stdlib` – On by default. Add an extensive list of higher level
//!   operations.
//!
//! features in `Cargo.toml`:
//!
//!   ```toml
//!   [dependencies.tobj]
//!   default-features = false
//!   ```
use core::{
    ffi::c_void,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    ptr, slice,
};
use libfive_sys as sys;
use std::{
    convert::TryInto,
    ffi::{CStr, CString},
};

#[cfg(feature = "ahash")]
type HashMap<K, V> = ahash::AHashMap<K, V>;

#[cfg(not(feature = "ahash"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

pub enum Error {
    VariablesCouldNotBeUpdated,
    VariableDoesNotExist,
}

/// Trait to aid with using arbitrary 2D point types on a [`Contour`].
pub trait Point2 {
    fn new(x: f32, y: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

/// Trait to aid with using arbitrary 3D point types on a [`TriangleMesh`].
pub trait Point3 {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

/// Series of 2D or 3D points forming a polyline.
pub type Contour<T> = Vec<T>;

/// Bitmap representing occupancy in a slice of a [`Tree`].
///
/// It contains `width()` * `height()` pixels, in row-major order.
pub struct Bitmap(*mut sys::libfive_pixels);

impl Bitmap {
    /// Returns the bitmap pixel buffer as a flat `[bool]` slice.
    ///
    /// The length is `width()` × `height()`.
    pub fn as_slice(&self) -> &[bool] {
        let bitmap = unsafe { self.0.as_ref() }.unwrap();
        unsafe {
            slice::from_raw_parts(
                bitmap.pixels,
                (bitmap.width * bitmap.height) as _,
            )
        }
    }

    /// Returns the bitmap pixel buffer as a flat, mutable `[bool]` slice.
    ///
    /// The length is `width()` × `height()`.
    pub fn as_slice_mut(&mut self) -> &mut [bool] {
        let bitmap = unsafe { self.0.as_mut() }.unwrap();
        unsafe {
            slice::from_raw_parts_mut(
                bitmap.pixels,
                (bitmap.width * bitmap.height) as _,
            )
        }
    }

    /// Returns the value of the poixel `x`, `y`.
    pub fn pixel(&self, x: u32, y: u32) -> bool {
        assert!(x < self.width() && y < self.height());
        self.as_slice()[(y * self.height() + x) as usize]
    }

    /// Returns the width of the bitmap.
    pub fn width(&self) -> u32 {
        unsafe { self.0.as_ref() }.unwrap().width
    }

    /// Returns the height of the bitmap.
    pub fn height(&self) -> u32 {
        unsafe { self.0.as_ref() }.unwrap().height
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { sys::libfive_pixels_delete(&mut self.0 as *mut _ as _) };
    }
}

/// Triangle mesh.
///
/// The `positions` type is generic. You can use whatever type you like. Just
/// implement the [`Point3`] trait on it.
///
/// The `triangles` are a list of indices into the `positions`.
pub struct TriangleMesh<T: Point3> {
    pub positions: Vec<T>,
    pub triangles: Vec<[u32; 3]>,
}

/// Flat triangle mesh.
///
/// The `positions` list has layout [x0, y0, z0, x1, y0, z1, ...].
///
/// The `triangles` list has layout [t0.v0, t0.v1, t0.v2, t1.v0, t1.v1, t1.v2,
/// ...] where t*n* is triangle *n* and v*n* is vertex index *n*.
pub struct FlatTriangleMesh {
    pub positions: Vec<f32>,
    pub triangles: Vec<u32>,
}

impl<T: Point3> From<TriangleMesh<T>> for FlatTriangleMesh {
    fn from(mesh: TriangleMesh<T>) -> FlatTriangleMesh {
        FlatTriangleMesh {
            positions: mesh
                .positions
                .into_iter()
                .flat_map(|point| {
                    std::array::IntoIter::new([point.x(), point.y(), point.z()])
                })
                .collect(),
            triangles: mesh
                .triangles
                .into_iter()
                .flat_map(|triangle| std::array::IntoIter::new(triangle))
                .collect(),
        }
    }
}

/// Set of variables to parameterize a [`Tree`].
pub struct Variables {
    map: HashMap<String, usize>,
    variables: Vec<*const c_void>,
    values: Vec<f32>,
    sys_variables: sys::libfive_vars,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            variables: Vec::new(),
            values: Vec::new(),
            sys_variables: sys::libfive_vars {
                vars: ptr::null(),
                values: ptr::null_mut(),
                size: 0,
            },
        }
    }

    pub fn add(&mut self, name: &str, value: f32) -> Tree {
        let tree = unsafe { sys::libfive_tree_var() };
        let id = unsafe { sys::libfive_tree_id(tree) };

        self.map.insert(name.to_string(), self.variables.len());
        self.variables.push(id);
        self.values.push(value);
        self.sys_variables.vars = self.variables.as_ptr() as *const _ as _;
        self.sys_variables.values = self.values.as_ptr() as *const _ as _;
        self.sys_variables.size = self.variables.len().try_into().unwrap();

        Tree(tree)
    }

    pub fn set(&mut self, name: &str, value: f32) -> Result<(), Error> {
        if let Some(&index) = self.map.get(name) {
            self.values[index] = value;
            Ok(())
        } else {
            Err(Error::VariableDoesNotExist)
        }
    }
}

impl Drop for Variables {
    fn drop(&mut self) {
        unsafe {
            sys::libfive_vars_delete(&mut self.sys_variables as *mut _ as _)
        };
    }
}

/// Helper for controlling evaluation of [`Variables`] on a [`Tree`].
pub struct Evaluator(sys::libfive_evaluator);

impl Evaluator {
    pub fn new(tree: &Tree, variables: &Variables) -> Self {
        Self(unsafe {
            sys::libfive_tree_evaluator(tree.0, variables.sys_variables)
        })
    }

    pub fn update(&mut self, variables: &Variables) -> Result<(), Error> {
        if unsafe {
            sys::libfive_evaluator_update_vars(self.0, variables.sys_variables)
        } {
            Err(Error::VariablesCouldNotBeUpdated)
        } else {
            Ok(())
        }
    }
}

impl Drop for Evaluator {
    fn drop(&mut self) {
        unsafe { sys::libfive_evaluator_delete(self.0) };
    }
}

/// 2D bounding region.
#[derive(Clone, Copy, Debug)]
pub struct Region2(sys::libfive_region2);

impl Region2 {
    pub fn new(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
        Self(sys::libfive_region2 {
            X: sys::libfive_interval {
                lower: x_min,
                upper: x_max,
            },
            Y: sys::libfive_interval {
                lower: y_min,
                upper: y_max,
            },
        })
    }
}

/// 3D bounding region.
#[derive(Clone, Copy, Debug)]
pub struct Region3(sys::libfive_region3);

impl Region3 {
    pub fn new(
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
    ) -> Self {
        Self(sys::libfive_region3 {
            X: sys::libfive_interval {
                lower: x_min,
                upper: x_max,
            },
            Y: sys::libfive_interval {
                lower: y_min,
                upper: y_max,
            },
            Z: sys::libfive_interval {
                lower: z_min,
                upper: z_max,
            },
        })
    }
}

#[allow(dead_code)]
#[repr(i32)]
enum Op {
    Invalid = 0,

    Constant = 1,
    VarX = 2,
    VarY = 3,
    VarZ = 4,
    VarFree = 5,
    ConstVar = 6,

    Square = 7,
    Sqrt = 8,
    Neg = 9,
    Sin = 10,
    Cos = 11,
    Tan = 12,
    Asin = 13,
    Acos = 14,
    Atan = 15,
    Exp = 16,
    Abs = 28,
    Log = 30,
    Recip = 29,

    Add = 17,
    Mul = 18,
    Min = 19,
    Max = 20,
    Sub = 21,
    Div = 22,
    Atan2 = 23,
    Pow = 24,
    NthRoot = 25,
    Mod = 26,
    NanFill = 27,
    Compare = 31,

    Oracle = 32,
}

macro_rules! fn_unary {
    ($func_name:ident, $op_code:ident) => {
        #[inline]
        pub fn $func_name(&self) -> Self {
            Self(unsafe { sys::libfive_tree_unary(Op::$op_code as _, self.0) })
        }
    };
}

macro_rules! fn_binary {
    ($func_name:ident, $op_code:ident, $other:ident) => {
        #[inline]
        pub fn $func_name(self, $other: Self) -> Self {
            Self(unsafe {
                sys::libfive_tree_binary(Op::$op_code as _, self.0, $other.0)
            })
        }
    };
}

macro_rules! op_binary {
    ($func_name:ident, $op_code:ident) => {
        impl $op_code for Tree {
            type Output = Tree;
            #[inline]
            fn $func_name(self, rhs: Tree) -> Self::Output {
                self.$func_name(rhs)
            }
        }
    };
}

/// Tree of operations.
///
/// * [Bases](#bases)
/// * [Core](#core)
/// * [Constructive solid geometry](#csg)
/// * [Shapes](#shapes)
/// * [Transformations](#transforms)
/// * [Text](#text)
/// * [Evaluation, Export & Rendering](#eval)
pub struct Tree(sys::libfive_tree);

/// An alias for [`Tree`].
///
/// Useful to make the kind of expected input more obvious for some operators.
pub type TreeFloat = Tree;

pub struct TreeVec2 {
    pub x: Tree,
    pub y: Tree,
}

pub struct TreeVec3 {
    pub x: Tree,
    pub y: Tree,
    pub z: Tree,
}

/// ## Bases <a name="bases"></a>
impl Tree {
    #[inline]
    pub fn x() -> Self {
        Self(unsafe { sys::libfive_tree_x() })
    }

    #[inline]
    pub fn y() -> Self {
        Self(unsafe { sys::libfive_tree_y() })
    }

    #[inline]
    pub fn z() -> Self {
        Self(unsafe { sys::libfive_tree_z() })
    }
}

impl From<f32> for TreeFloat {
    /// Creates a constant. Most often you will use this for variables.
    fn from(number: f32) -> Self {
        Self(unsafe { sys::libfive_tree_const(number) })
    }
}

/// ## Core <a name="core"></a>
impl Tree {
    fn_unary!(square, Square);
    fn_unary!(sqrt, Sqrt);
    fn_unary!(neg, Neg);
    fn_unary!(sin, Sin);
    fn_unary!(cos, Cos);
    fn_unary!(tan, Tan);
    fn_unary!(asin, Asin);

    /// Computes the arccosine of a `self`.
    fn_unary!(acos, Acos);
    fn_unary!(atan, Atan);
    fn_unary!(exp, Exp);

    /// Computes the absolute value of `self`.
    fn_unary!(abs, Abs);
    fn_unary!(log, Log);
    fn_unary!(recip, Recip);

    fn_binary!(add, Add, rhs);
    fn_binary!(mul, Mul, rhs);
    fn_binary!(min, Min, rhs);
    fn_binary!(max, Max, rhs);
    fn_binary!(sub, Sub, rhs);
    fn_binary!(div, Div, rhs);
    fn_binary!(atan2, Atan2, other);
    fn_binary!(pow, Pow, exp);
    fn_binary!(nth_root, NthRoot, n);
    fn_binary!(rem, Mod, rhs);
    fn_binary!(nan_fill, NanFill, rhs);
    fn_binary!(compare, Compare, rhs);
}

include!("stdlib.rs");

/// # Evaluation, Export & Rendering <a name="eval"></a>
impl Tree {
    /// Renders a 2D slice at the given `z` height into a [`Bitmap`].
    #[inline]
    pub fn to_bitmap(
        &self,
        region: &Region2,
        z: f32,
        resolution: f32,
    ) -> Bitmap {
        Bitmap(unsafe {
            sys::libfive_tree_render_pixels(self.0, region.0, z, resolution)
        })
    }

    /// Renders the tree to a [`TriangleMesh`].
    ///
    /// The `region` is a bounding box that will be subdivided into an octree.
    /// For clean  triangles, it should be near-cubical, but that this is
    /// not a hard requirement.
    ///
    /// The `resolution` should be approximately half the model's smallest
    /// feature size. Subdivision halts when all sides of the region are
    /// below it.
    pub fn to_triangle_mesh<T: Point3>(
        &self,
        region: &Region3,
        resolution: f32,
    ) -> TriangleMesh<T> {
        let libfive_mesh = unsafe {
            sys::libfive_tree_render_mesh(self.0, region.0, resolution).as_mut()
        }
        .unwrap();

        let mesh = TriangleMesh::<T> {
            positions: (0..libfive_mesh.vert_count)
                .into_iter()
                .map(|index| {
                    let vertex =
                        &unsafe { *libfive_mesh.verts.add(index as _) };
                    T::new(vertex.x, vertex.y, vertex.z)
                })
                .collect(),
            triangles: (0..libfive_mesh.tri_count)
                .into_iter()
                .map(|index| {
                    let triangle =
                        &unsafe { *libfive_mesh.tris.add(index as _) };
                    [triangle.a, triangle.b, triangle.c]
                })
                .collect(),
        };

        unsafe {
            sys::libfive_mesh_delete(libfive_mesh as *mut _ as _);
        }

        mesh
    }

    /// Renders a tree to a set of 2D contours.
    ///
    /// The `region` is a bounding box that will be subdivided into an octree.
    /// For clean  triangles, it should be near-cubical, but that this is
    /// not a hard requirement.
    ///
    /// The `resolution` should be approximately half the model's smallest
    /// feature size. Subdivision halts when all sides of the region are
    /// below it.
    pub fn to_slice_2d<T: Point2>(
        &self,
        region: Region2,
        z: f32,
        resolution: f32,
    ) -> Option<Vec<Contour<T>>> {
        let raw_contours = unsafe {
            sys::libfive_tree_render_slice(self.0, region.0, z, resolution)
                .as_ref()
        };

        if let Some(raw_contours) = raw_contours {
            let contours = (0..raw_contours.count)
                .into_iter()
                .map(|index| {
                    let contour =
                        unsafe { raw_contours.cs.add(index as _).as_ref() }
                            .unwrap();
                    (0..contour.count)
                        .into_iter()
                        .map(|index| {
                            let point =
                                unsafe { contour.pts.add(index as _).as_ref() }
                                    .unwrap();
                            T::new(point.x, point.y)
                        })
                        .collect()
                })
                .collect();

            unsafe {
                sys::libfive_contours_delete(&raw_contours as *const _ as _);
            }

            Some(contours)
        } else {
            None
        }
    }

    /// Renders a tree to a set of 3D contours.
    ///
    /// The `region` is a bounding box that will be subdivided into an octree.
    /// For clean  triangles, it should be near-cubical, but that this is
    /// not a hard requirement.
    ///
    /// The `resolution` should be approximately half the model's smallest
    /// feature size. Subdivision halts when all sides of the region are
    /// below it.
    pub fn to_slice_3d<T: Point3>(
        &self,
        region: Region2,
        z: f32,
        resolution: f32,
    ) -> Option<Vec<Contour<T>>> {
        let raw_contours = unsafe {
            sys::libfive_tree_render_slice3(self.0, region.0, z, resolution)
                .as_ref()
        };

        if let Some(raw_contours) = raw_contours {
            let contours = (0..raw_contours.count)
                .into_iter()
                .map(|index| {
                    let contour =
                        unsafe { raw_contours.cs.add(index as _).as_ref() }
                            .unwrap();

                    (0..contour.count)
                        .into_iter()
                        .map(|index| {
                            let point =
                                unsafe { contour.pts.add(index as _).as_ref() }
                                    .unwrap();
                            T::new(point.x, point.y, point.z)
                        })
                        .collect()
                })
                .collect();

            unsafe {
                sys::libfive_contours_delete(&raw_contours as *const _ as _);
            }

            Some(contours)
        } else {
            None
        }
    }

    /// Computes a slice and saves it to `path` in [`SVG`](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics) format.
    pub fn to_slice_svg(
        &self,
        region: &Region2,
        z: f32,
        resolution: f32,
        path: impl Into<Vec<u8>>,
    ) {
        let path = CString::new(path).unwrap();
        unsafe {
            sys::libfive_tree_save_slice(
                self.0,
                region.0,
                z,
                resolution,
                path.as_ptr(),
            );
        }
    }
}

impl Drop for Tree {
    fn drop(&mut self) {
        unsafe { sys::libfive_tree_delete(self.0) };
    }
}

op_binary!(add, Add);
op_binary!(div, Div);
op_binary!(mul, Mul);
op_binary!(rem, Rem);
op_binary!(sub, Sub);

impl Neg for Tree {
    type Output = Tree;

    fn neg(self) -> Self::Output {
        Self(unsafe { sys::libfive_tree_unary(Op::Neg as _, self.0) })
    }
}

/*
#[test]
fn test_svg() {
    let x = Tree::x();
    let x2 = x.square();

    let y = Tree::y();
    let y2 = y.square();

    let sum = x2.add(&y2);

    let one = Tree::from(1.0);

    let out = sum.sub(&one);

    out.to_slice_svg(
        &Region2::new(-2.0, 2.0, -2.0, 2.0),
        0.0,
        10.0,
        "test.svg",
    );
}
*/

#[test]
fn test() {
    let x = Tree::x();

    let out2 = {
        let y = Tree::y();
        x.union(&y)
    };

    out2.to_slice_svg(
        &Region2::new(-2.0, 2.0, -2.0, 2.0),
        0.0,
        10.0,
        "test.svg",
    );
}

/*
#[test]
fn test2() {
    let x = Tree2::x();

    let out2 = {
        let y = Tree2::y();
        x.union_self(y)
    };

    /*
    out1.to_slice_svg(
        &Region2::new(-2.0, 2.0, -2.0, 2.0),
        0.0,
        10.0,
        "test.svg",
    );*/
}*/
