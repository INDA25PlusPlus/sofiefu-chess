# HUR MAN ANVÄNDER DETTA FANTASTISKA BIBLIOTEK

```
// vi representerar brädet med 2st 2d vektorer:
board: string // innehåller "empty", "pawn", "knight", "bishop", "rook", "queen", "king"
player: char // innehåller 'b', 'w', ' ' 

struct game_state // innehåller våra states: vems tur, brädet, spelare. samt några extra funktioner

// i main:
let mut game = game_state::new();
show_square(r, c, &mut game); // för att visa vilken pjäs det är på en viss ruta med koordinater (r, c)

// tre typer av drag
game.make_move(start_r, start_c, end_r, end_c);
game.promotion(start_r, start_c, new_piece);
game.castle(); 

// make_move och promotion returnerar olika typer av en enum:
enum Outcome{
    // valid moves
    Valid,
    Check,
    Checkmate,

    // invalid moves
    Bad_coordinates, 
    Wrong_player,
    Invalid,
    Checked
}

```