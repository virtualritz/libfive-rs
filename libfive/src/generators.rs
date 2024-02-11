/// Generators
impl Tree {
    pub fn array_x(shape: Tree, nx: u32, dx: TreeFloat) -> Self {
        Self(unsafe { sys::array_x(shape.0, nx.try_into().unwrap(), dx.0) })
    }

    pub fn array_xy(shape: Tree, nx: u32, ny: u32, delta: TreeVec2) -> Self {
        Self(unsafe {
            sys::array_xy(
                shape.0,
                nx.try_into().unwrap(),
                ny.try_into().unwrap(),
                sys::tvec2 {
                    x: delta.x.0,
                    y: delta.y.0,
                },
            )
        })
    }

    pub fn array_xyz(
        shape: Tree,
        nx: u32,
        ny: u32,
        nz: u32,
        delta: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::array_xyz(
                shape.0,
                nx.try_into().unwrap(),
                ny.try_into().unwrap(),
                nz.try_into().unwrap(),
                sys::tvec3 {
                    x: delta.x.0,
                    y: delta.y.0,
                    z: delta.z.0,
                },
            )
        })
    }

    pub fn array_polar_z(shape: Tree, n: u32, center: TreeVec2) -> Self {
        Self(unsafe {
            sys::array_polar_z(
                shape.0,
                n.try_into().unwrap(),
                sys::tvec2 {
                    x: center.x.0,
                    y: center.y.0,
                },
            )
        })
    }

    pub fn extrude_z(t: Tree, zmin: TreeFloat, zmax: TreeFloat) -> Self {
        Self(unsafe { sys::extrude_z(t.0, zmin.0, zmax.0) })
    }
}

