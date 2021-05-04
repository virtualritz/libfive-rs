use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};
use libfive_sys as sys;
use std::ffi::CString;

pub struct Tree<'a> {
    tree: sys::libfive_tree,
    // _marker needs to be invariant in 'a.
    // See "Making a struct outlive a parameter given to a method of
    // that struct": https://stackoverflow.com/questions/62374326/
    _marker: PhantomData<*mut &'a ()>,
}

pub trait MeshFn {
    fn new(x: f32, y: f32, z: f32) -> Self;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

pub struct Mesh<T: MeshFn> {
    pub positions: Vec<T>,
    pub triangles: Vec<[u32; 3]>,
}

pub struct FlatMesh {
    pub positions: Vec<f32>,
    pub triangles: Vec<u32>,
}

impl<T: MeshFn> From<Mesh<T>> for FlatMesh
{
    fn from(mesh: Mesh<T>) -> FlatMesh {
        FlatMesh {
            positions: mesh
                .positions
                .into_iter()
                .flat_map(|point| std::array::IntoIter::new([point.x(), point.y(), point.z()]))
                .collect(),
            triangles: mesh
                .triangles
                .into_iter()
                .flat_map(|triangle| std::array::IntoIter::new(triangle))
                .collect(),
        }
    }
}

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

pub struct Region3(sys::libfive_region3);

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
        pub fn $func_name(&self) -> Self {
            Self {
                tree: unsafe { sys::libfive_tree_unary(Op::$op_code as _, self.tree) },
                _marker: PhantomData,
            }
        }
    };
}

macro_rules! fn_binary {
    ($func_name:ident, $op_code:ident, $other:ident) => {
        pub fn $func_name(&self, $other: &Self) -> Self {
            Self {
                tree: unsafe {
                    sys::libfive_tree_binary(Op::$op_code as _, self.tree, $other.tree)
                },
                _marker: PhantomData,
            }
        }
    };
}

macro_rules! op_binary {
    ($func_name:ident, $op_code:ident) => {
        impl<'a> $op_code for &Tree<'a> {
            type Output = Tree<'a>;

            fn $func_name(self, rhs: Self) -> Self::Output {
                self.$func_name(rhs)
            }
        }
    };
}

impl<'a> Tree<'a> {
    pub fn x() -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_x() },
            _marker: PhantomData,
        }
    }

    pub fn y() -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_y() },
            _marker: PhantomData,
        }
    }

    pub fn z() -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_z() },
            _marker: PhantomData,
        }
    }

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

    pub fn save_slice(&self, region: &Region2, z: f32, resolution: f32, path: impl Into<Vec<u8>>) {
        let path = CString::new(path).unwrap();
        unsafe {
            sys::libfive_tree_save_slice(self.tree, region.0, z, resolution, path.as_ptr());
        }
    }

    pub fn to_mesh<T: MeshFn>(&self, region: &Region3, resolution: f32) -> Mesh<T> {
        let libfive_mesh =
            &mut unsafe { *sys::libfive_tree_render_mesh(self.tree, region.0, resolution) };

        let mesh = Mesh::<T> {
            positions: (0..libfive_mesh.vert_count)
                .into_iter()
                .map(|index| {
                    let vertex = &unsafe { *libfive_mesh.verts.add(index as _) };
                    T::new(vertex.x, vertex.y, vertex.z)
                })
                .collect(),
            triangles: (0..libfive_mesh.tri_count)
                .into_iter()
                .map(|index| {
                    let triangle = &unsafe { *libfive_mesh.tris.add(index as _) };
                    [triangle.a, triangle.b, triangle.c]
                })
                .collect(),
        };

        unsafe {
            sys::libfive_mesh_delete(libfive_mesh as *mut _ as _);
        }

        mesh
    }
}

impl<'a> Drop for Tree<'a> {
    fn drop(&mut self) {
        unsafe { sys::libfive_tree_delete(self.tree) };
    }
}

impl<'a> From<f32> for Tree<'a> {
    fn from(number: f32) -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_const(number) },
            _marker: PhantomData,
        }
    }
}

op_binary!(add, Add);
op_binary!(div, Div);
op_binary!(mul, Mul);
op_binary!(rem, Rem);
op_binary!(sub, Sub);

impl<'a> Neg for &Tree<'a> {
    type Output = Tree<'a>;

    fn neg(self) -> Self::Output {
        Self::Output {
            tree: unsafe { sys::libfive_tree_unary(Op::Neg as _, self.tree) },
            _marker: PhantomData,
        }
    }
}

#[test]
fn test() {
    let x2 = Tree::x().square();
    let y2 = Tree::y().square();

    let out = &(&x2 + &y2) - &Tree::from(1.0);

    out.save_slice(&Region2::new(-2.0, 2.0, -2.0, 2.0), 0.0, 10.0, "test.svg");
}
