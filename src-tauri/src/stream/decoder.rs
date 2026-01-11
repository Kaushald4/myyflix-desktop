use base64::{engine::general_purpose, Engine as _};
use std::collections::HashMap;

/* ---------------- helpers ---------------- */

fn atob(input: &str) -> String {
    let bytes = general_purpose::STANDARD.decode(input).unwrap();
    String::from_utf8(bytes).unwrap()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/* ---------------- JS: bMGyx71TzQLfdonN ---------------- */

fn bmg_yx_71_tz_ql_fdon_n(input: &str) -> String {
    let mut chunks = Vec::new();
    let bytes = input.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        let end = (i + 3).min(bytes.len());
        chunks.push(std::str::from_utf8(&bytes[i..end]).unwrap());
        i += 3;
    }

    chunks.into_iter().rev().collect()
}

/* ---------------- JS: Iry9MQXnLs ---------------- */

fn iry_9_mqx_n_ls(input: &str) -> String {
    let key = "pWB9V)[*4I`nJpp?ozyB~dbr9yt!_n4u";

    let hex_decoded: String = input
        .as_bytes()
        .chunks(2)
        .map(|c| {
            let hex = std::str::from_utf8(c).unwrap();
            char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()
        })
        .collect();

    let xored: String = hex_decoded
        .bytes()
        .enumerate()
        .map(|(i, b)| (b ^ key.as_bytes()[i % key.len()]) as char)
        .collect();

    let shifted: String = xored.bytes().map(|b| (b - 3) as char).collect();

    atob(&shifted)
}

/* ---------------- JS: IGLImMhWrI ---------------- */

fn rot13(c: char) -> char {
    match c {
        'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
        'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
        _ => c,
    }
}

fn igl_im_mh_wr_i(input: &str) -> String {
    let rev = reverse(input);
    let rot: String = rev.chars().map(rot13).collect();
    let final_rev = reverse(&rot);
    atob(&final_rev)
}

/* ---------------- JS: GTAxQyTyBx ---------------- */

fn gta_x_qy_ty_bx(input: &str) -> String {
    let rev = reverse(input);
    let filtered: String = rev.chars().step_by(2).collect();
    atob(&filtered)
}

/* ---------------- JS: C66jPHx8qu ---------------- */

fn c66_j_phx_8_qu(input: &str) -> String {
    let rev = reverse(input);
    let key = "X9a(O;FMV2-7VO5x;Ao\u{5}:dN1NoFs?j,";

    let decoded: String = rev
        .as_bytes()
        .chunks(2)
        .map(|c| {
            let hex = std::str::from_utf8(c).unwrap();
            char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()
        })
        .collect();

    decoded
        .bytes()
        .enumerate()
        .map(|(i, b)| (b ^ key.as_bytes()[i % key.len()]) as char)
        .collect()
}

/* ---------------- JS: MyL1IRSfHe ---------------- */

fn my_l1_irs_f_he(input: &str) -> String {
    let rev = reverse(input);
    let shifted: String = rev.bytes().map(|b| (b - 1) as char).collect();

    shifted
        .as_bytes()
        .chunks(2)
        .map(|c| {
            let hex = std::str::from_utf8(c).unwrap();
            char::from_u32(u32::from_str_radix(hex, 16).unwrap()).unwrap()
        })
        .collect()
}

/* ---------------- JS: detdj7JHiK ---------------- */

fn detdj_7_j_hi_k(input: &str) -> String {
    let sliced = &input[10..input.len() - 16];
    let key = "3SAY~#%Y(V%>5d/Yg\"$G[Lh1rK4a;7ok";

    let decoded = atob(sliced);

    decoded
        .bytes()
        .enumerate()
        .map(|(i, b)| (b ^ key.as_bytes()[i % key.len()]) as char)
        .collect()
}

/* ---------------- JS: nZlUnj2VSo ---------------- */
fn n_zl_unj_2_v_so(input: &str) -> String {
    let mut out = String::with_capacity(input.len());

    for c in input.chars() {
        let mapped = match c {
            'a'..='w' => ((c as u8) + 3) as char,
            'x' => 'a',
            'y' => 'b',
            'z' => 'c',

            'A'..='W' => ((c as u8) + 3) as char,
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',

            _ => c,
        };
        out.push(mapped);
    }

    out
}

/* ---------------- JS: laM1dAi3vO ---------------- */

fn la_m1_d_ai_3_v_o(input: &str) -> String {
    let rev = reverse(input).replace('-', "+").replace('_', "/");

    let decoded = atob(&rev);

    decoded.bytes().map(|b| (b - 5) as char).collect()
}

/* ---------------- JS: GuxKGDsA2T ---------------- */

fn gux_kg_ds_a_2_t(input: &str) -> String {
    let rev = reverse(input).replace('-', "+").replace('_', "/");

    let decoded = atob(&rev);

    decoded.bytes().map(|b| (b - 7) as char).collect()
}

/* ---------------- JS: LXVUMCoAHJ ---------------- */

fn lxv_um_co_ah_j(input: &str) -> String {
    let rev = reverse(input).replace('-', "+").replace('_', "/");

    let decoded = atob(&rev);

    decoded.bytes().map(|b| (b - 3) as char).collect()
}

/* ---------------- registry + public API ---------------- */

type DecoderFn = fn(&str) -> String;

fn decoders() -> HashMap<&'static str, DecoderFn> {
    HashMap::from([
        ("Iry9MQXnLs", iry_9_mqx_n_ls as DecoderFn),
        ("IGLImMhWrI", igl_im_mh_wr_i),
        ("GTAxQyTyBx", gta_x_qy_ty_bx),
        ("C66jPHx8qu", c66_j_phx_8_qu),
        ("MyL1IRSfHe", my_l1_irs_f_he),
        ("detdj7JHiK", detdj_7_j_hi_k),
        ("nZlUnj2VSo", n_zl_unj_2_v_so),
        ("laM1dAi3vO", la_m1_d_ai_3_v_o),
        ("GuxKGDsA2T", gux_kg_ds_a_2_t),
        ("LXVUMCoAHJ", lxv_um_co_ah_j),
    ])
}

pub fn add_stream_host(input: &str) -> String {
    let host = "thrumbleandjaxon.com";
    input
        .replace("{v1}", host)
        .replace("{v2}", host)
        .replace("{v3}", host)
        .replace("{v4}", host)
        .replace("{v5}", host)
}

pub fn get_stream_link(id: &str, decoded_text: &str) -> Option<String> {
    for (key, decoder) in decoders() {
        if id == bmg_yx_71_tz_ql_fdon_n(key) {
            return Some(add_stream_host(&decoder(decoded_text)));
        }
    }
    None
}
