use colorgrad::{
    blues, br_bg, bu_gn, bu_pu, cividis, cool, cubehelix_default, gn_bu, greens, greys, inferno,
    magma, or_rd, oranges, pi_yg, plasma, pr_gn, pu_bu, pu_bu_gn, pu_or, pu_rd, purples, rainbow,
    rd_bu, rd_gy, rd_pu, rd_yl_bu, rd_yl_gn, reds, sinebow, spectral, turbo, viridis, warm, yl_gn,
    yl_gn_bu, yl_or_br, yl_or_rd, Color, Gradient,
};

/// Load a colormap from the current data bin.
pub fn load_colormap(mut name: &str, num_colors: usize) -> Vec<Color> {
    // colormap names suffixed with "_r" are reversed
    let mut reverse: bool = false;
    if name.ends_with("_r") {
        name = &name[0..name.len() - 2];
        reverse = true;
    }

    // QUESTION: can this be done with a macro?
    let gradient: Gradient = match name {
        "blues" => blues(),
        "br_bg" => br_bg(),
        "bu_gn" => bu_gn(),
        "bu_pu" => bu_pu(),
        "cividis" => cividis(),
        "cool" => cool(),
        "cubehelix_default" => cubehelix_default(),
        "gn_bu" => gn_bu(),
        "greens" => greens(),
        "greys" => greys(),
        "inferno" => inferno(),
        "magma" => magma(),
        "or_rd" => or_rd(),
        "oranges" => oranges(),
        "pi_yg" => pi_yg(),
        "plasma" => plasma(),
        "pr_gn" => pr_gn(),
        "pu_bu" => pu_bu(),
        "pu_bu_gn" => pu_bu_gn(),
        "pu_or" => pu_or(),
        "pu_rd" => pu_rd(),
        "purples" => purples(),
        "rainbow" => rainbow(),
        "rd_bu" => rd_bu(),
        "rd_gy" => rd_gy(),
        "rd_pu" => rd_pu(),
        "rd_yl_bu" => rd_yl_bu(),
        "rd_yl_gn" => rd_yl_gn(),
        "reds" => reds(),
        "sinebow" => sinebow(),
        "spectral" => spectral(),
        "turbo" => turbo(),
        "viridis" => viridis(),
        "warm" => warm(),
        "yl_gn" => yl_gn(),
        "yl_gn_bu" => yl_gn_bu(),
        "yl_or_br" => yl_or_br(),
        "yl_or_rd" => yl_or_rd(),
        s => panic!("Unknown gradient {}", s),
    };
    let mut vec_color = gradient.colors(num_colors);
    if reverse {
        vec_color.reverse();
    }
    vec_color
}
