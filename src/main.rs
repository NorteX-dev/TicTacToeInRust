use macroquad::prelude::*;

const PADDING: f32 = 10.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "TicTacToe".to_string(),
        window_width: 700,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut board = [["", "", ""], ["", "", ""], ["", "", ""]];
    let mut turn = "x";
    let mut game_on = true;

    let text_params = TextParams {
        font_size: 200,
        color: Color::from_rgba(0, 0, 0, 255),
        ..Default::default()
    };

    loop {
        clear_background(Color::from_rgba(235, 235, 235, 255));

        if !game_on {
            return;
        }


        for i in 0..3 {
            for j in 0..3 {
                let i = i as f32;
                let j = j as f32;
                let tile_size = screen_height() / 3.0;
                draw_line(0.0, tile_size * i + tile_size, screen_width(), tile_size * i + tile_size, 10.0, BLACK);
                draw_line(tile_size * j + tile_size, 0.0, tile_size * j + tile_size, screen_height(), 10.0, BLACK);
                let symbol = board[j as usize][i as usize];
                draw_text_ex(symbol, tile_size * i + tile_size / 2.0 - (text_params.font_size as f32 / 4.0), tile_size * j + (text_params.font_size as f32) - (text_params.font_size as f32 / 4.0), text_params);
            }
        }

        draw_text(&*format!("It's {}'s turn!", turn.to_uppercase()), 0.0, 30.0, 30.0, BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let tile_x = (mouse_x / (screen_width() / 3.0)).floor() as usize;
            let tile_y = (mouse_y / (screen_height() / 3.0)).floor() as usize;
            if board[tile_y][tile_x] == "" {
                // Set board field to turn
                board[tile_y][tile_x] = turn;
                // Swap turns
                if turn == "x" {
                    turn = "o";
                } else {
                    turn = "x";
                }
                check_winning_conditions(&board, &mut game_on);
            }
        }

        next_frame().await;
    }
}

fn check_winning_conditions(&board: &[[&str; 3]; 3], game_on: &mut bool) {
    // check verticals
    for i in 0..3 {
        if board[i][0] != "" && board[i][1] != "" && board[i][2] != "" {
            if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] == board[i][2] {
                println!("Won by horizontal match");
                *game_on = false;
            }
        }
    }
    for i in 0..3 {
        if board[0][i] != "" && board[1][i] != "" && board[2][i] != "" {
            if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] == board[2][i] {
                println!("Won by vertical match");
                *game_on = false;
            }
        }
    }
    // just bruteforce diagonals lol
    if board[0][0] != "" && board[1][1] != "" && board[2][2] != "" {
        if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] == board[2][2] {
            println!("Won by diagonal match");
            *game_on = false;
        }
    }
    if board[0][2] != "" && board[1][1] != "" && board[2][0] != "" {
        if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] == board[2][0] {
            println!("Won by reverse diagonal match");
            *game_on = false;
        }
    }
}