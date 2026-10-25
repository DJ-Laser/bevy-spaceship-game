use bevy::prelude::*;

pub fn common_parts() -> impl SceneList {
    bsn_list! {
      @super::reactor_mk5()
      @super::battery_100h4()
      @super::plasma_cannon()
      @super::engine_xn_2000()
      @super::engine_rh_500()
      @super::rcs_tl_150()
    }
}
