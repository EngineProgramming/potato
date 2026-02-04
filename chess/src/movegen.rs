use crate::{
    Castling, Piece, Side,
    bitboard::Bitboard,
    mv::{Mv, PromoPiece},
    position::Position,
    square::Square,
};

fn push_promotions(movelist: &mut Vec<Mv>, from: Square, to: Square) {
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Queen),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Rook),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Bishop),
    });
    movelist.push(Mv {
        from,
        to,
        promo: Some(PromoPiece::Knight),
    });
}

/// Iterate over a piece mask
/// Generate moves for each piece
fn generate_moves(
    movelist: &mut Vec<Mv>,
    mut pieces: Bitboard,
    us: Bitboard,
    them: Bitboard,
    func: &dyn Fn(Square, Bitboard) -> Bitboard,
) {
    while pieces.is_occupied() {
        let from = pieces.pop_lsb();
        let mut moves = func(from, us | them) & !us;
        while moves.is_occupied() {
            let to = moves.pop_lsb();
            movelist.push(Mv {
                from,
                to,
                promo: None,
            });
        }
    }
}

impl Position {
    /// Generate pseudolegal moves
    /// This includes moves that leave the king in check
    #[must_use]
    pub fn pseudolegal_moves(&self) -> Vec<Mv> {
        let mut movelist = vec![];

        let us = self.colours[self.turn as usize];
        let them = self.colours[!self.turn as usize];
        let pawns = us & self.pieces[Piece::Pawn as usize];
        let knights = us & self.pieces[Piece::Knight as usize];
        let bishops = us & self.pieces[Piece::Bishop as usize];
        let rooks = us & self.pieces[Piece::Rook as usize];
        let queens = us & self.pieces[Piece::Queen as usize];
        let king = us & self.pieces[Piece::King as usize];
        let ep_bb = if let Some(ep) = self.ep {
            Bitboard::from_square(ep)
        } else {
            Bitboard::from_empty()
        };

        // Pawns
        if self.turn == Side::White {
            // Singles
            let mut north: Bitboard = pawns.north() & !(us | them);
            while north.is_occupied() {
                let to = north.pop_lsb();
                let from = Square::from_index(to.get_index() - 8);
                if to.y == 7 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }

            // Doubles
            let mut north: Bitboard =
                ((pawns & Bitboard(0xFF00)).north() & !(us | them)).north() & !(us | them);
            while north.is_occupied() {
                let to = north.pop_lsb();
                let from = Square::from_index(to.get_index() - 16);
                movelist.push(Mv {
                    from,
                    to,
                    promo: None,
                });
            }

            // Captures
            let mut ne = pawns.north().east() & (them | ep_bb);
            while ne.is_occupied() {
                let to = ne.pop_lsb();
                let from = Square::from_index(to.get_index() - 9);
                if to.y == 7 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
            let mut nw = pawns.north().west() & (them | ep_bb);
            while nw.is_occupied() {
                let to = nw.pop_lsb();
                let from = Square::from_index(to.get_index() - 7);
                if to.y == 7 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
        } else {
            // Singles
            let mut south: Bitboard = pawns.south() & !(us | them);
            while south.is_occupied() {
                let to = south.pop_lsb();
                let from = Square::from_index(to.get_index() + 8);
                if to.y == 0 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }

            // Doubles
            let mut south: Bitboard = ((pawns & Bitboard(0xFF000000000000)).south() & !(us | them))
                .south()
                & !(us | them);
            while south.is_occupied() {
                let to = south.pop_lsb();
                let from = Square::from_index(to.get_index() + 16);
                movelist.push(Mv {
                    from,
                    to,
                    promo: None,
                });
            }

            // Captures
            let mut se = pawns.south().east() & (them | ep_bb);
            while se.is_occupied() {
                let to = se.pop_lsb();
                let from = Square::from_index(to.get_index() + 7);
                if to.y == 0 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
            let mut sw = pawns.south().west() & (them | ep_bb);
            while sw.is_occupied() {
                let to = sw.pop_lsb();
                let from = Square::from_index(to.get_index() + 9);
                if to.y == 0 {
                    push_promotions(&mut movelist, from, to);
                } else {
                    movelist.push(Mv {
                        from,
                        to,
                        promo: None,
                    });
                }
            }
        }

        generate_moves(&mut movelist, knights, us, them, &Bitboard::mask_knight);
        generate_moves(&mut movelist, bishops, us, them, &Bitboard::mask_bishop);
        generate_moves(&mut movelist, rooks, us, them, &Bitboard::mask_rook);
        generate_moves(&mut movelist, queens, us, them, &Bitboard::mask_queen);
        generate_moves(&mut movelist, king, us, them, &Bitboard::mask_king);

        let ksq = (self.colours[self.turn as usize] & self.pieces[Piece::King as usize]).pop_lsb();
        let in_check = self.is_attacked(ksq, !self.turn);

        // Castling - white king side
        if self.turn == Side::White
            && self.castling[Castling::WKS as usize]
            && !in_check
            && self.is_empty(Square::from_index(5))
            && self.is_empty(Square::from_index(6))
            && !self.is_attacked(Square::from_index(5), !self.turn)
            && !self.is_attacked(Square::from_index(6), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(4),
                to: Square::from_index(6),
                promo: None,
            });
        }

        // Castling - white queen side
        if self.turn == Side::White
            && self.castling[Castling::WQS as usize]
            && !in_check
            && self.is_empty(Square::from_index(3))
            && self.is_empty(Square::from_index(2))
            && self.is_empty(Square::from_index(1))
            && !self.is_attacked(Square::from_index(3), !self.turn)
            && !self.is_attacked(Square::from_index(2), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(4),
                to: Square::from_index(2),
                promo: None,
            });
        }

        // Castling - black king side
        if self.turn == Side::Black
            && self.castling[Castling::BKS as usize]
            && !in_check
            && self.is_empty(Square::from_index(61))
            && self.is_empty(Square::from_index(62))
            && !self.is_attacked(Square::from_index(61), !self.turn)
            && !self.is_attacked(Square::from_index(62), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(60),
                to: Square::from_index(62),
                promo: None,
            });
        }

        // Castling - black queen side
        if self.turn == Side::Black
            && self.castling[Castling::BQS as usize]
            && !in_check
            && self.is_empty(Square::from_index(59))
            && self.is_empty(Square::from_index(58))
            && self.is_empty(Square::from_index(57))
            && !self.is_attacked(Square::from_index(59), !self.turn)
            && !self.is_attacked(Square::from_index(58), !self.turn)
        {
            movelist.push(Mv {
                from: Square::from_index(60),
                to: Square::from_index(58),
                promo: None,
            });
        }

        movelist
    }
}
