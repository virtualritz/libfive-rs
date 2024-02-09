/// Transforms
impl Tree {
    pub fn moveit(self, offset: TreeVec3) -> Self {
        Self(unsafe {
            sys::move_(
                self.0,
                sys::tvec3 {
                    x: offset.x.0,
                    y: offset.y.0,
                    z: offset.z.0,
                },
            )
        })
    }

    pub fn reflect_x(self, x0: TreeFloat) -> Self {
        Self(unsafe { sys::reflect_x(self.0, x0.0) })
    }

    pub fn reflect_y(self, y0: TreeFloat) -> Self {
        Self(unsafe { sys::reflect_y(self.0, y0.0) })
    }

    pub fn reflect_z(self, z0: TreeFloat) -> Self {
        Self(unsafe { sys::reflect_z(self.0, z0.0) })
    }

    pub fn reflect_xy(self) -> Self {
        Self(unsafe { sys::reflect_xy(self.0) })
    }

    pub fn reflect_yz(self) -> Self {
        Self(unsafe { sys::reflect_yz(self.0) })
    }

    pub fn reflect_xz(self) -> Self {
        Self(unsafe { sys::reflect_xz(self.0) })
    }

    pub fn symmetric_x(self) -> Self {
        Self(unsafe { sys::symmetric_x(self.0) })
    }

    pub fn symmetric_y(self) -> Self {
        Self(unsafe { sys::symmetric_y(self.0) })
    }

    pub fn symmetric_z(self) -> Self {
        Self(unsafe { sys::symmetric_z(self.0) })
    }

    pub fn scale_x(self, sx: TreeFloat, x0: TreeFloat) -> Self {
        Self(unsafe { sys::scale_x(self.0, sx.0, x0.0) })
    }

    pub fn scale_y(self, sy: TreeFloat, y0: TreeFloat) -> Self {
        Self(unsafe { sys::scale_y(self.0, sy.0, y0.0) })
    }

    pub fn scale_z(self, sz: TreeFloat, z0: TreeFloat) -> Self {
        Self(unsafe { sys::scale_z(self.0, sz.0, z0.0) })
    }

    pub fn scale_xyz(self, s: TreeVec3, center: TreeVec3) -> Self {
        Self(unsafe {
            sys::scale_xyz(
                self.0,
                sys::tvec3 {
                    x: s.x.0,
                    y: s.y.0,
                    z: s.z.0,
                },
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn rotate_x(self, angle: TreeFloat, center: TreeVec3) -> Self {
        Self(unsafe {
            sys::rotate_x(
                self.0,
                angle.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn rotate_y(self, angle: TreeFloat, center: TreeVec3) -> Self {
        Self(unsafe {
            sys::rotate_y(
                self.0,
                angle.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn rotate_z(self, angle: TreeFloat, center: TreeVec3) -> Self {
        Self(unsafe {
            sys::rotate_z(
                self.0,
                angle.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn taper_x_y(
        self,
        base: TreeVec2,
        h: TreeFloat,
        scale: TreeFloat,
        base_scale: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::taper_x_y(
                self.0,
                sys::tvec2 {
                    x: base.x.0,
                    y: base.y.0,
                },
                h.0,
                scale.0,
                base_scale.0,
            )
        })
    }

    pub fn taper_xy_z(
        self,
        base: TreeVec3,
        height: TreeFloat,
        scale: TreeFloat,
        base_scale: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::taper_xy_z(
                self.0,
                sys::tvec3 {
                    x: base.x.0,
                    y: base.y.0,
                    z: base.z.0,
                },
                height.0,
                scale.0,
                base_scale.0,
            )
        })
    }

    pub fn shear_x_y(
        self,
        base: TreeVec2,
        height: TreeFloat,
        offset: TreeFloat,
        base_offset: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::shear_x_y(
                self.0,
                sys::tvec2 {
                    x: base.x.0,
                    y: base.y.0,
                },
                height.0,
                offset.0,
                base_offset.0,
            )
        })
    }

    pub fn repel(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_x(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_x(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_y(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_y(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_z(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_z(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_xy(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_xy(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_yz(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_yz(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn repel_xz(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::repel_xz(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_x(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_x(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_y(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_y(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_z(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_z(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_xy(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_xy(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_yz(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_yz(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn attract_xz(
        self,
        locus: TreeVec3,
        radius: TreeFloat,
        exaggerate: TreeFloat,
    ) -> Self {
        Self(unsafe {
            sys::attract_xz(
                self.0,
                sys::tvec3 {
                    x: locus.x.0,
                    y: locus.y.0,
                    z: locus.z.0,
                },
                radius.0,
                exaggerate.0,
            )
        })
    }

    pub fn revolve_y(self, x0: TreeFloat) -> Self {
        Self(unsafe { sys::revolve_y(self.0, x0.0) })
    }

    pub fn twirl_x(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_x(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn twirl_axis_x(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_axis_x(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn twirl_y(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_y(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn twirl_axis_y(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_axis_y(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn twirl_z(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_z(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }

    pub fn twirl_axis_z(
        self,
        amount: TreeFloat,
        radius: TreeFloat,
        center: TreeVec3,
    ) -> Self {
        Self(unsafe {
            sys::twirl_axis_z(
                self.0,
                amount.0,
                radius.0,
                sys::tvec3 {
                    x: center.x.0,
                    y: center.y.0,
                    z: center.z.0,
                },
            )
        })
    }
}

