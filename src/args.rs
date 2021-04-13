use clap::{clap_app, crate_authors, crate_description, crate_version, ArgMatches};
use std::fmt::Debug;
use std::str::FromStr;

pub struct Arguments {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) fov: f32,
    pub(crate) viewport_distance: f32,
    pub(crate) camera_distance: f32,
    pub(crate) samples_per_pixel: u32,
    pub(crate) max_cpus: u32,
    pub(crate) max_depth: u32,
    pub(crate) u_samples_first_bounce: u32,
    pub(crate) v_samples_first_bounce: u32,
}

fn parse_argument<T>(matches: &ArgMatches, key: &str, default: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let message: &'static str = Box::leak(format!("Invalid value for {}", key).into_boxed_str());
    matches
        .value_of(key)
        .unwrap_or(default)
        .parse::<T>()
        .expect(message)
}

pub fn parse_arguments() -> Arguments {
    let matches = clap_app!(xraytracer =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg WIDTH: -w --width +takes_value "Sets the camera width")
        (@arg HEIGHT: -h --height +takes_value "Sets the camera height")
        (@arg FOV: --fov +takes_value "Sets the camera field of view in degrees")
        (@arg VP_DIST: --vp_dist +takes_value "Sets the viewports Z origin")
        (@arg CAM_DIST: --cam_dist +takes_value "Sets the cameras Z origin")
        (@arg SPP: --spp +takes_value "Sets the number of samples per pixel of the camera")
        (@arg MAX_CPUS: --max_cpus +takes_value "Sets the maximum number of threads to spin up")
        (@arg MAX_DEPTH: --max_depth +takes_value "Sets the maximum recursive depth of the algorithm")
        (@arg U_SFB: --u_sfb +takes_value "Sets the number of samples for U on first bounce")
        (@arg V_SFB: --v_sfb +takes_value "Sets the number of samples for V on first bounce")
    )
    .get_matches();

    Arguments {
        width: parse_argument::<u16>(&matches, "WIDTH", "1920"),
        height: parse_argument::<u16>(&matches, "HEIGHT", "1080"),
        fov: parse_argument::<f32>(&matches, "FOV", "45"),
        viewport_distance: parse_argument::<f32>(&matches, "VP_DIST", "0.5"),
        camera_distance: parse_argument::<f32>(&matches, "CAM_DIST", "-10"),
        samples_per_pixel: parse_argument::<u32>(&matches, "SPP", "40"),
        max_cpus: parse_argument::<u32>(&matches, "MAX_CPUS", "1"),
        max_depth: parse_argument::<u32>(&matches, "MAX_DEPTH", "5"),
        u_samples_first_bounce: parse_argument::<u32>(&matches, "U_SFB", "4"),
        v_samples_first_bounce: parse_argument::<u32>(&matches, "V_SFB", "4"),
    }
}
