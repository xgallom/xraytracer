use nalgebra::zero;
use nalgebra_glm::Vec3;

pub struct Material {
    pub(crate) diffuse: Vec3,
    pub(crate) emission: Vec3,
    pub(crate) index_of_refraction: f32,
    pub(crate) reflectivity: f32,
    pub(crate) reflection_cone_angle_radians: f32,
}

const DEFAULT_INDEX_OF_REFRACTION: f32 = 1f32;
const DEFAULT_REFLECTIVITY: f32 = -1f32;
const DEFAULT_REFLECTION_CONE_ANGLE_RADIANS: f32 = 0f32;

impl Material {
    pub fn make_diffuse(colour: Vec3) -> Self {
        Self {
            diffuse: colour,
            emission: zero(),
            index_of_refraction: DEFAULT_INDEX_OF_REFRACTION,
            reflectivity: DEFAULT_REFLECTIVITY,
            reflection_cone_angle_radians: DEFAULT_REFLECTION_CONE_ANGLE_RADIANS,
        }
    }

    pub fn make_specular(colour: Vec3, index_of_refraction: f32) -> Self {
        Self {
            diffuse: colour,
            emission: zero(),
            index_of_refraction,
            reflectivity: DEFAULT_REFLECTIVITY,
            reflection_cone_angle_radians: DEFAULT_REFLECTION_CONE_ANGLE_RADIANS,
        }
    }

    pub fn make_light(colour: Vec3) -> Self {
        Self {
            diffuse: zero(),
            emission: colour,
            index_of_refraction: DEFAULT_INDEX_OF_REFRACTION,
            reflectivity: DEFAULT_REFLECTIVITY,
            reflection_cone_angle_radians: DEFAULT_REFLECTION_CONE_ANGLE_RADIANS,
        }
    }

    pub fn make_glossy(
        colour: Vec3,
        index_of_refraction: f32,
        reflection_cone_angle_degrees: f32,
    ) -> Self {
        Self {
            diffuse: colour,
            emission: zero(),
            index_of_refraction,
            reflectivity: DEFAULT_REFLECTIVITY,
            reflection_cone_angle_radians: reflection_cone_angle_degrees.to_radians(),
        }
    }

    pub fn make_reflective(
        colour: Vec3,
        reflectivity: f32,
        reflection_cone_angle_degrees: f32,
    ) -> Self {
        Self {
            diffuse: colour,
            emission: zero(),
            index_of_refraction: DEFAULT_INDEX_OF_REFRACTION,
            reflectivity,
            reflection_cone_angle_radians: reflection_cone_angle_degrees.to_radians(),
        }
    }
}
