# Mutations (sx10_14), screens: the draw_text comparer (side by side on the surface) catches them in the game comparing,
# on any text drawn without a callback:
# - the red and blue of the text colour swapped (any coloured text);
# - one more pixel between letters (every text of two or more characters).
MUTACOES = [
    ('sx10_14_colour',
     'font_pack(font_mix(r, colour.0, alpha), font_mix(g, colour.1, alpha), font_mix(b, colour.2, alpha))',
     'font_pack(font_mix(r, colour.2, alpha), font_mix(g, colour.1, alpha), font_mix(b, colour.0, alpha)) /* mutation */',
     'sx10_14 text drawn'),
    ('sx10_14_spacing',
     'pen = pen.wrapping_add(advance);',
     'pen = pen.wrapping_add(advance).wrapping_add(0x10000); // mutation',
     'sx10_14 text drawn'),
]
