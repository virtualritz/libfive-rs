#![doc(
    html_logo_url = "https://raw.githubusercontent.com/virtualritz/libfive-rs/HEAD/libfive/libfive-logo.png"
)]
//! A high level wrapper around [`libfive`](https://libfive.com/) – a set of
//! tools for solid modeling based on [functional representation](https://en.wikipedia.org/wiki/Function_representation).
//!
//! Particulalry suited for parametric- and procedural modeling. An
//! infrastructure for generative design, mass customization, and
//! domain-specific CAD tools.
//!
//! ## Example
//!
//! ```ignore
//! # use libfive::*;
//! # fn example() -> Result<()> {
//! let csg_shape = Tree::sphere(1.0.into(), TreeVec3::default())
//!     .difference_multi(vec![
//!         Tree::sphere(0.6.into(), TreeVec3::default()),
//!         Tree::cylinder_z(
//!             0.6.into(),
//!             2.0.into(),
//!             TreeVec3::new(0.0, 0.0, -1.0),
//!         ),
//!         Tree::cylinder_z(
//!             0.6.into(),
//!             2.0.into(),
//!             TreeVec3::new(0.0, 0.0, -1.0),
//!         )
//!         .reflect_xz(),
//!         Tree::cylinder_z(
//!             0.6.into(),
//!             2.0.into(),
//!             TreeVec3::new(0.0, 0.0, -1.0),
//!         )
//!         .reflect_yz(),
//!     ]);
//!
//! csg_shape.to_stl(
//!     "csg_shape.stl",
//!     &Region3::new(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0),
//!     &BRepSettings::default(),
//! )?;
//! # }
//! ```
//! The STL file generated from this code is show below.
//!
//! ## Features
//!
//! <img src="https://raw.githubusercontent.com/virtualritz/libfive-rs/HEAD/libfive/f-rep-shape.png" alt="Generated CSG Shape" width="33%" padding-left="15%" align="right" align="top">
//!
//! * [`ahash`](https://crates.io/crates/ahash) – On by default. Use [`AHashMap`](https://docs.rs/ahash/latest/ahash/struct.AHashMap.html)
//!   for hashing when resolving variable names. Disabling this will fall back
//!   to the slower [`HashMap`](std::collections::HashMap).
//!
//! * `stdlib` – On by default. Add an extensive list of higher level operations
//!   – the *libfive stdlib*.
//!
//!   To disable either/both of the above features unset default features in
//!   `Cargo.toml`:
//!
//!   ```toml
//!   [dependencies.libfive]
//!   default-features = false
//!   ```
//! * `packed_opcodes` - Tightly pack opcodes. This breaks compatibility with
//!   older saved f-rep files.
use core::{
    convert::TryInto,
    ffi::c_void,
    mem,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
    ptr, result, slice,
};
use libfive_sys as sys;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::ffi::CString;

#[cfg(feature = "ahash")]
type HashMap<K, V> = ahash::AHashMap<K, V>;

#[cfg(not(feature = "ahash"))]
type HashMap<K, V> = std::collections::HashMap<K, V>;

/// A specialized [`Result`] type for `libfive` operations.
///
/// This type is broadly used across `libvive` for any operation which may
/// produce an error.
///
/// This typedef is generally used to avoid writing out [`Error`] directly and
/// is otherwise a direct mapping to [`Result`].
pub type Result<T> = result::Result<T, Error>;

/// A list specifying general categories of errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// [`libfive::Error`]: Error
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Error {
    /// The sepcified variable could not be updated.
    VariablesCouldNotBeUpdated,
    /// The requested variable could not be found.
    VariableNotFound,
    /// The variable with this name was already added,
    VariableAlreadyAdded,
    /// The resp. file could not be opened for writing.
    FileWriteFailed,
    /// The resp. file could not be opened for reading..
    FileReadFailed,
    /// The queried tree is not a constant.
    TreeIsNotConstant,
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

/// Series of 2D or 3D points forming a
/// [polygonal chain](https://en.wikipedia.org/wiki/Polygonal_chain).
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
/// The `positions` list has layout `[x0, y0, z0, x1, y1, z1, ...]`.
///
/// The `triangles` list has layout `[t0.v0, t0.v1, t0.v2, t1.v0, t1.v1, t1.v2,
/// ...]` where `t`*n* is triangle *n* and `v`*m* is vertex index *m*.
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

/// The algorithm used for computing a
/// [boundary representation](https://en.wikipedia.org/wiki/Boundary_representation)
/// from a [`Tree`].
#[derive(Copy, Clone, Debug, Eq, FromPrimitive, IntoPrimitive, PartialEq)]
#[repr(u32)]
pub enum BRepAlgorithm {
    #[num_enum(default)]
    DualContouring = sys::libfive_brep_alg_DUAL_CONTOURING as _,
    IsoSimplex = sys::libfive_brep_alg_ISO_SIMPLEX as _,
    Hybrid = sys::libfive_brep_alg_HYBRID as _,
}

/// [Boundary representation](https://en.wikipedia.org/wiki/Boundary_representation)
/// settings passed to any of the rendering/export functions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BRepSettings {
    /// The meshing region is subdivided until the smallest region’s edge is
    /// below 1/`resolution` in size. Make this larger to get a higher
    /// resolution model.
    pub resolution: f32,
    /// This value is used when deciding whether to collapse cells. If it is
    /// very large, then only linear regions are merged.  Set as `0.1` to
    /// completely disable cell merging.
    pub quality: f32,
    /// Number of worker threads to use while meshing.  Set as 0 to use the
    /// platform's-default number of threads.
    pub workers: u32,
    /// The meshing algorithm.
    pub algorithm: BRepAlgorithm,
}

/// Defaults for rendering a [`Tree`].
///
/// `resolution`: `10`
/// `quality`: `8`
/// `workers`: `0` (determined autmatically)
/// `algorithm`: [`DualContouring`](BRepAlgorithm::DualContouring)
impl Default for BRepSettings {
    fn default() -> Self {
        let s = unsafe { sys::libfive_brep_settings_default() };
        Self {
            resolution: s.res,
            quality: s.quality,
            workers: s.workers,
            algorithm: s.alg.into(),
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

impl Default for Variables {
    fn default() -> Self {
        Variables::new()
    }
}

impl Variables {
    /// Creates a new, empty set of variables.
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

    /// Adds the variable `name` to the set.
    ///
    /// # Errors
    ///
    /// Returns [`Error::VariableAlreadyAdded`] if the variable already exists
    /// in the set.
    pub fn add(&mut self, name: &str, value: f32) -> Result<Tree> {
        let name = name.to_string();
        if self.map.contains_key(&name) {
            Err(Error::VariableAlreadyAdded)
        } else {
            let tree = unsafe { sys::libfive_tree_var() };
            let id = unsafe { sys::libfive_tree_id(tree) };

            self.map.insert(name, self.variables.len());
            self.variables.push(id);
            self.values.push(value);
            // Update struct.
            self.sys_variables.vars = self.variables.as_ptr() as *const _ as _;
            self.sys_variables.values = self.values.as_ptr() as *const _ as _;
            self.sys_variables.size = self.variables.len().try_into().unwrap();

            Ok(Tree(tree))
        }
    }

    /// Sets the variable `name` to `value`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::VariableNotFound`] if the variable does not exist in
    /// the set.
    pub fn set(&mut self, name: &str, value: f32) -> Result<()> {
        if let Some(&index) = self.map.get(name) {
            self.values[index] = value;
            Ok(())
        } else {
            Err(Error::VariableNotFound)
        }
    }
}

impl Drop for Variables {
    fn drop(&mut self) {
        println!("Dropping");
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

    pub fn update(&mut self, variables: &Variables) -> Result<()> {
        if unsafe {
            sys::libfive_evaluator_update_vars(self.0, variables.sys_variables)
        } {
            Err(Error::VariablesCouldNotBeUpdated)
        } else {
            Ok(())
        }
    }

    pub fn to_stl(
        &self,
        path: impl Into<Vec<u8>>,
        region: &Region3,
        settings: &BRepSettings,
    ) -> Result<()> {
        let path = CString::new(path).unwrap();
        if unsafe {
            sys::libfive_evaluator_save_mesh(
                self.0,
                region.0,
                mem::transmute(*settings),
                path.as_ptr(),
            )
        } {
            Ok(())
        } else {
            Err(Error::FileWriteFailed)
        }
    }
}

impl Drop for Evaluator {
    fn drop(&mut self) {
        unsafe { sys::libfive_evaluator_delete(self.0) };
    }
}

/// 2D bounding region.
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
/// # Core
///
/// * [Constant][`TreeFloat::from::<f32>()`]
/// * [Bases](#bases)
/// * [Functions](#functions)
/// * [Evaluation, import & export](#eval)
///
/// # Standard Library
///
/// These features are dependent on the `stdlib` feature being enabled.
///
/// * [Shapes](#shapes)
/// * [Generators](#generators)
/// * [Constructive solid geometry](#csg)
/// * [Transformations](#transforms)
/// * [Text](#text)
#[derive(Eq, PartialEq)]
pub struct Tree(sys::libfive_tree);

impl Clone for Tree {
    fn clone(&self) -> Self {
        Self(unsafe { sys::libfive_tree_clone(self.0) })
    }
}

/// An alias for [`Tree`].
///
/// Used to make the kind of sensible input more obvious for some operators.
pub type TreeFloat = Tree;

/// # Constants <a name="constant"></a>
impl From<f32> for TreeFloat {
    /// Creates a constant [`Tree`].
    fn from(constant: f32) -> Self {
        Self(unsafe { sys::libfive_tree_const(constant) })
    }
}

/// # Bases <a name="bases"></a>
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

    //pub fn variable() -> Self {}
}

/// # Functions <a name="functions"></a>
impl Tree {
    fn_unary!(square, Square);
    fn_unary!(sqrt, Sqrt);
    fn_unary!(neg, Neg);
    fn_unary!(sin, Sin);
    fn_unary!(cos, Cos);
    fn_unary!(tan, Tan);
    fn_unary!(asin, Asin);
    fn_unary!(acos, Acos);
    fn_unary!(atan, Atan);
    fn_unary!(exp, Exp);
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

    /// Checks if the tree is a variable.
    pub fn is_variable(&self) -> bool {
        unsafe { sys::libfive_tree_is_var(self.0) }
    }

    /// Returns the value of the tree if it is constant.
    ///
    /// I.e. if it was created from an [`f32`] value.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TreeIsNotConstant`] if the tree is not constant.
    pub fn as_f32(&self) -> Result<f32> {
        let mut success = false;
        let value = unsafe {
            sys::libfive_tree_get_const(self.0, &mut success as *mut _)
        };

        if success {
            Ok(value)
        } else {
            Err(Error::TreeIsNotConstant)
        }
    }
}

/// # Evaluation, Import & Export <a name="eval"></a>
///
/// ## Common Arguments
///
/// * `region` – A bounding box that will be subdivided into an octree. For
/// clean lines/triangles, it should be near-cubical, but that this is not a
/// hard requirement.
///
/// * `resolution` – Should be approximately half the model's smallest feature
///   size. Subdivision halts when all sides of the region are
/// below it.
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
    pub fn to_triangle_mesh<T: Point3>(
        &self,
        region: &Region3,
        settings: &BRepSettings,
    ) -> Option<TriangleMesh<T>> {
        match unsafe {
            sys::libfive_tree_render_mesh(
                self.0,
                region.0,
                mem::transmute(*settings),
            )
            .as_mut()
        } {
            Some(raw_mesh) => {
                let mesh = TriangleMesh::<T> {
                    positions: (0..raw_mesh.vert_count)
                        .into_iter()
                        .map(|index| {
                            let vertex =
                                &unsafe { *raw_mesh.verts.add(index as _) };
                            T::new(vertex.x, vertex.y, vertex.z)
                        })
                        .collect(),
                    triangles: (0..raw_mesh.tri_count)
                        .into_iter()
                        .map(|index| {
                            let triangle =
                                &unsafe { *raw_mesh.tris.add(index as _) };
                            [triangle.a, triangle.b, triangle.c]
                        })
                        .collect(),
                };

                unsafe {
                    sys::libfive_mesh_delete(raw_mesh as *mut _ as _);
                }

                Some(mesh)
            }
            None => None,
        }
    }

    /// Renders a tree to a set of 2D contours.
    pub fn to_contour_2d<T: Point2>(
        &self,
        region: Region2,
        z: f32,
        settings: &BRepSettings,
    ) -> Option<Vec<Contour<T>>> {
        match unsafe {
            sys::libfive_tree_render_slice(
                self.0,
                region.0,
                z,
                mem::transmute(*settings),
            )
            .as_mut()
        } {
            Some(raw_contours) => {
                let contours = (0..raw_contours.count)
                    .into_iter()
                    .map(|index| {
                        let contour =
                            unsafe { raw_contours.cs.add(index as _).as_ref() }
                                .unwrap();
                        (0..contour.count)
                            .into_iter()
                            .map(|index| {
                                let point = unsafe {
                                    contour.pts.add(index as _).as_ref()
                                }
                                .unwrap();
                                T::new(point.x, point.y)
                            })
                            .collect()
                    })
                    .collect();

                unsafe {
                    sys::libfive_contours_delete(raw_contours as *mut _ as _);
                }

                Some(contours)
            }
            None => None,
        }
    }

    /// Renders a tree to a set of 3D contours.
    pub fn to_contour_3d<T: Point3>(
        &self,
        region: Region2,
        z: f32,
        settings: &BRepSettings,
    ) -> Option<Vec<Contour<T>>> {
        let raw_contours = unsafe {
            sys::libfive_tree_render_slice3(
                self.0,
                region.0,
                z,
                mem::transmute(*settings),
            )
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

    /// Computes a contour and saves it to `path` in [`SVG`](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics) format.
    pub fn to_svg(
        &self,
        path: impl Into<Vec<u8>>,
        region: &Region2,
        z: f32,
        settings: &BRepSettings,
    ) -> Result<()> {
        let path = CString::new(path).unwrap();
        if unsafe {
            sys::libfive_tree_save_slice(
                self.0,
                region.0,
                z,
                mem::transmute(*settings),
                path.as_ptr(),
            )
        } {
            Ok(())
        } else {
            Err(Error::FileWriteFailed)
        }
    }

    /// Computes a mesh and saves it to `path` in [`STL`](https://en.wikipedia.org/wiki/STL_(file_format)) format.
    pub fn to_stl(
        &self,
        path: impl Into<Vec<u8>>,
        region: &Region3,
        settings: &BRepSettings,
    ) -> Result<()> {
        let path = CString::new(path).unwrap();
        if unsafe {
            sys::libfive_tree_save_mesh(
                self.0,
                region.0,
                mem::transmute(*settings),
                path.as_ptr(),
            )
        } {
            Ok(())
        } else {
            Err(Error::FileWriteFailed)
        }
    }

    /// Serializes the tree to a file.
    ///
    /// The file format is not archival and may change without notice.
    ///
    /// Note that old files may fail to load if the `packed_opcodes` feature is
    /// enabled.
    pub fn save(&self, path: impl Into<Vec<u8>>) -> Result<()> {
        let path = CString::new(path).unwrap();
        if unsafe { sys::libfive_tree_save(self.0, path.as_ptr()) } {
            Ok(())
        } else {
            Err(Error::FileWriteFailed)
        }
    }

    /// Deserializes a tree from a file.
    ///
    /// Note that files may fail to load with older versions of `libfive` if
    /// the `packed_opcodes` feature is enabled.
    pub fn load(&self, path: impl Into<Vec<u8>>) -> Result<Tree> {
        let path = CString::new(path).unwrap();
        match unsafe { sys::libfive_tree_load(path.as_ptr()).as_mut() } {
            Some(tree) => Ok(Self(tree as _)),
            None => Err(Error::FileReadFailed),
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

#[cfg(feature = "stdlib")]
mod stdlib;

#[cfg(feature = "stdlib")]
pub use stdlib::*;

/*
#[test]
fn test_2d() -> Result<()> {
    let circle = Tree::x().square() + Tree::y().square() - 1.0.into();

    circle.to_svg(
        "circle.svg",
        &Region2::new(-2.0, 2.0, -2.0, 2.0),
        0.0,
        &BRepSettings::default(),
    )?;

    Ok(())
}*/

#[test]
#[cfg(feature = "stdlib")]
fn test_3d() -> Result<()> {
    let f_rep_shape = Tree::sphere(1.0.into(), TreeVec3::default())
        .difference_multi(vec![
            Tree::sphere(0.6.into(), TreeVec3::default()),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            ),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            )
            .reflect_xz(),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            )
            .reflect_yz(),
        ]);

    f_rep_shape.to_stl(
        "f-rep-shape.stl",
        &Region3::new(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0),
        &BRepSettings::default(),
    )?;

    Ok(())
}
/*
#[test]
#[cfg(feature = "stdlib")]
fn test_eval_3d() -> Result<()> {
    //let mut variables = Variables::new();

    //let inner_radius = variables.add("inner_radius", 0.6)?;

    let csg_shape = Tree::sphere(1.0.into(), TreeVec3::default())
        .difference_multi(vec![
            Tree::sphere(0.6.into(), TreeVec3::default()),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            ),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            )
            .reflect_xz(),
            Tree::cylinder_z(
                0.6.into(),
                2.0.into(),
                TreeVec3::new(0.0, 0.0, -1.0),
            )
            .reflect_yz(),
        ]);

    //let mut evaluator = Evaluator::new(&csg_shape, &variables);
    //evaluator.update(&variables);

    csg_shape.to_stl(
        "csg_shape.stl",
        &Region3::new(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0),
        &BRepSettings {
            //workers: 0,
            ..Default::default()
        },
    )?;
    /*
    variables.set("inner_radius", 0.4);
    evaluator.update(&variables);

    csg_shape.to_stl(
        "csg_shape_0_4.stl",
        &Region3::new(-2.0, 2.0, -2.0, 2.0, -2.0, 2.0),
        10.0,
    )?;*/

    Ok(())
}*/
