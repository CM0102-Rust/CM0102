# Mutations (sx10_1) by the save proof with the holiday save's load (CARGA; mutacoes.py --prova-save N). No 4th field:
# the base of the save proof runs with the routines off.
# Both give the holiday manager a fog of war map on load. The map makes fog_after_match (after each of his club's
# matches) call FOG_OF_WAR_MAP::raise_knowledge, which draws a random number per player of the lineup: the draws and
# the career after the load change.
# - the constructor goes on to the human managers' maps even when the load succeeded;
# - the load reports failure after reading the masking flag off, so the constructor builds the maps.
MUTACOES = [
    ('sx10_1_maps_after_load',
     'if load(this, save) != 0 { // sx10_1: the maps came from the save',
     'if load(this, save) == 0xdead { // mutation: the maps are built anyway'),
    ('sx10_1_load_fails',
     'return 1; // sx10_1: masking off, nothing more in fog.dat',
     'return 0; // mutation: a successful load reported as failed'),
]
