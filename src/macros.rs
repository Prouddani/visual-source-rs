#[macro_export]
macro_rules! vs_str {
    ($s:literal) => {
        $crate::field_types::string::VSString::from($s)
    }
}

#[macro_export]
macro_rules! vs_num {
    ($n:literal) => {
        $crate::field_types::number::VSNumber::from($n)
    }
}

#[macro_export]
macro_rules! vs_obj {
    ($path:literal) => {
        $crate::field_types::object::VSObject::from_path($path)
    }
}

#[macro_export]
macro_rules! vs_bool {
    ($b:literal) => {
        $crate::field_types::bool::VSBool::from($b)
    }
}

#[macro_export]
macro_rules! vs_vec2 {
    ($x:literal, $y:literal) => {
        $crate::field_types::vector2::VSVector2::from(($x, $y))
    }
}

#[macro_export]
macro_rules! vs_vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        $crate::field_types::vector3::VSVector3::from(($x, $y, $z))
    }
}

#[macro_export]
macro_rules! vs_brickcolor {
    ($i:literal) => {
        $crate::field_types::brickcolor::VSBrickColor::from($i)
    };

    ($r:literal, $g:literal, $b:literal) => {
        $crate::field_types::brickcolor::VSBrickColor::from_rgb(($r, $g, $b))
    }
}

#[macro_export]
macro_rules! vs_col3 {
    ($r:literal, $g:literal, $b:literal) => {
        $crate::field_types::color3::VSColor3::from(($r, $g, $b))
    }
}

#[macro_export]
macro_rules! vs_tuple {
    () => {
        $crate::field_types::tuple::VSTuple::new();
    }
}

#[macro_export]
macro_rules! vs_udim2 {
    ($xscale:literal, $xoffset:literal, $yscale:literal, $yoffset:literal) => {
        $crate::field_types::udim2::VSUDim2::from(($xscale, $xoffset, $yscale, $yoffset))
    }
}

#[macro_export]
macro_rules! vs_nil {
    () => {
        $crate::field_types::nil::VSNil
    }
}