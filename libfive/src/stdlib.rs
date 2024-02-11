use crate::*;

/// 2D point/vector/normal.
pub struct TreeVec2 {
    pub x: Tree,
    pub y: Tree,
}

impl TreeVec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: Tree::from(x),
            y: Tree::from(y),
        }
    }
}

impl Default for TreeVec2 {
    fn default() -> Self {
        Self {
            x: Tree::from(0.0),
            y: Tree::from(0.0),
        }
    }
}

/// 3D point/vector/normal.
pub struct TreeVec3 {
    pub x: Tree,
    pub y: Tree,
    pub z: Tree,
}

impl TreeVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: Tree::from(x),
            y: Tree::from(y),
            z: Tree::from(z),
        }
    }
}

impl Default for TreeVec3 {
    fn default() -> Self {
        Self {
            x: Tree::from(0.0),
            y: Tree::from(0.0),
            z: Tree::from(0.0),
        }
    }
}

include!("shapes.rs");
include!("generators.rs");
include!("csg.rs");

/// A collection of [`Tree`]s.
///
/// This is used for the [`*_multi()`](Tree#multi_csg) CSG operations.
pub type Trees = Vec<Tree>;

/// <a name="multi_csg"></a>
/// Operations taking multiple 2nd arguments.
impl Tree {
    pub fn union_multi(self, trees: Trees) -> Self {
        if trees.is_empty() {
            Tree::emptiness()
        } else {
            trees.into_iter().fold(self, |a, b| a.union(b))
        }
    }

    pub fn intersection_multi(self, trees: Trees) -> Self {
        if trees.is_empty() {
            Tree::emptiness()
        } else {
            trees.into_iter().fold(self, |a, b| a.intersection(b))
        }
    }

    pub fn difference_multi(self, trees: Trees) -> Self {
        if trees.is_empty() {
            self
        } else {
            self.intersection(
                trees
                    .into_iter()
                    .reduce(|a, b| a.union(b))
                    .expect("there is at least one tree")
                    .inverse(),
            )
        }
    }
}

include!("transforms.rs");
include!("text.rs");
