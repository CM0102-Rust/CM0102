# Mutations (sx10_13), screens:
# - the last character adds its advance instead of its ink extent: the text width's comparer catches it in the game
#   comparing (every centred or right-aligned text asks for the width);
# - the shadow blur divided by 24 (darker shadows): only the original builds glyphs in the comparison mode, so this one
#   is only seen in Rust mode (the shadowed texts of the screens, flag 0x80 of FONT::draw_text, look heavier).
MUTACOES = [
    ('sx10_13_last_advance',
     "total.wrapping_add(ink).wrapping_sub(1) // sx10_13: the last character's ink",
     "{ let _ = ink; total.wrapping_add(read_i32(glyph + font_glyph::ADVANCE)) } // mutation: the advance",
     'sx10_13 text width'),
    ('sx10_13_blur',
     'write_u8(out, (sum as i16 as i32 / 25) as u8);',
     'write_u8(out, (sum as i16 as i32 / 24) as u8); // mutation'),
]
