# Mutations (tel26_2): none. These input routines (hit test, popup hover, the two drags, the text entry key, the
# scroll bar click, the network keepalive) draw, open popups, consume the key or send a packet, so their comparers
# run only the original (like the match screens' input handlers): no comparer can catch a mutation. The proof is
# Lucas playing in Rust mode (clicks, menus that open on top, scrolling a list, typing a name in a text box).
MUTACOES = []
