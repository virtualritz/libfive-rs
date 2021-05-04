use core::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Neg, Sub},
};
use libfive_sys as sys;

pub struct Tree<'a> {
    tree: sys::libfive_tree,
    // _marker needs to be invariant in 'a.
    // See "Making a struct outlive a parameter given to a method of
    // that struct": https://stackoverflow.com/questions/62374326/
    _marker: PhantomData<*mut &'a ()>,
}

pub struct Region2(sys::libfive_region2);
pub struct Region3(sys::libfive_region2);

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

    pub fn square(&self) -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_unary(Op::Square as _, self.tree) },
            _marker: PhantomData,
        }
    }

    pub fn sqrt(&self) -> Self {
        Self {
            tree: unsafe { sys::libfive_tree_unary(Op::Sqrt as _, self.tree) },
            _marker: PhantomData,
        }
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

impl<'a> Add for &Tree<'a> {
    type Output = Tree<'a>;

    fn add(self, rhs: Self) -> Self::Output {
        Tree {
            tree: unsafe { sys::libfive_tree_binary(Op::Add as _, self.tree, rhs.tree) },
            _marker: PhantomData,
        }
    }
}

impl<'a> Div for &Tree<'a> {
    type Output = Tree<'a>;

    fn div(self, rhs: Self) -> Self::Output {
        Tree {
            tree: unsafe { sys::libfive_tree_binary(Op::Div as _, self.tree, rhs.tree) },
            _marker: PhantomData,
        }
    }
}

impl<'a> Mul for &Tree<'a> {
    type Output = Tree<'a>;

    fn mul(self, rhs: Self) -> Self::Output {
        Tree {
            tree: unsafe { sys::libfive_tree_binary(Op::Mul as _, self.tree, rhs.tree) },
            _marker: PhantomData,
        }
    }
}

impl<'a> Neg for &Tree<'a> {
    type Output = Tree<'a>;

    fn neg(self) -> Self::Output {
        Tree {
            tree: unsafe { sys::libfive_tree_unary(Op::Neg as _, self.tree) },
            _marker: PhantomData,
        }
    }
}

impl<'a> Sub for &Tree<'a> {
    type Output = Tree<'a>;

    fn sub(self, rhs: Self) -> Self::Output {
        Tree {
            tree: unsafe { sys::libfive_tree_binary(Op::Sub as _, self.tree, rhs.tree) },
            _marker: PhantomData,
        }
    }
}

#[test]
fn test() {
    let x = Tree::x();
    let y = Tree::y();

    let x = Tree::x();
    let y = Tree::y();
}
