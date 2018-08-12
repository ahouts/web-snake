#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
extern crate brotli;
extern crate bincode;
extern crate chrono;
extern crate time;

use stdweb::{initialize, event_loop};
use stdweb::web::{self, document, IParentNode, IEventTarget, INode, Element, IElement};
use stdweb::web::html_element::{CanvasElement, InputElement};
use stdweb::web::event::{KeyDownEvent, ClickEvent};
use stdweb::traits::IKeyboardEvent;
use stdweb::unstable::TryInto;
use std::cell::RefCell;
use std::rc::Rc;

mod canvas;
mod snake;
mod graphics_data;
mod js_utils;

struct Cfg {
    canvas: CanvasElement,
    width: u32,
    height: u32,
    game_frame_rate: u32,
    frame_rate: u32,
}

fn run_snake_game<F>(cfg_cell: &Rc<RefCell<Cfg>>, res: F)
    where F: FnOnce(Result<snake::GameResult, String>) + 'static {
    let cfg = cfg_cell.borrow_mut();

    let snake_game = Rc::new(
        RefCell::new(snake::SnakeGameLogic::new(cfg.width, cfg.height, cfg.game_frame_rate))
    );

    web::window().add_event_listener({
        let snake = snake_game.clone();
        move |event: KeyDownEvent| {
            let mut snake = snake.borrow_mut();
            match event.key().as_ref() {
                "w" | "W" | "ArrowUp" => snake.press_key(snake::MoveDirection::Up),
                "s" | "S" | "ArrowDown" => snake.press_key(snake::MoveDirection::Down),
                "a" | "A" | "ArrowLeft" => snake.press_key(snake::MoveDirection::Left),
                "d" | "D" | "ArrowRight" => snake.press_key(snake::MoveDirection::Right),
                _ => {}
            }
        }
    });

    let snake_canvas = match canvas::Canvas::new(cfg.canvas.clone(), cfg.frame_rate) {
        Ok(c) => c,
        Err(e) => {
            res(Err(e));
            return;
        }
    };
    let snake_canvas = Rc::new(RefCell::new(snake_canvas));

    fn main_loop<F>(c: Rc<RefCell<canvas::Canvas>>, s: Rc<RefCell<snake::SnakeGameLogic>>, res: F)
        where F: FnOnce(Result<snake::GameResult, String>) + 'static {
        let snake_ref = s.clone();
        let wait_time = {
            let mut snake_game = snake_ref.borrow_mut();
            match snake_game.advance() {
                Ok(d) => {
                    let canvas_ref = c.clone();
                    let mut canvas = canvas_ref.borrow_mut();
                    Some(canvas.render(&d))
                }
                Err(d) => {
                    let canvas_ref = c.clone();
                    let mut canvas = canvas_ref.borrow_mut();
                    canvas.render(&d);
                    None
                }
            }
        };
        if let Some(t) = wait_time {
            web::set_timeout(move || {
                main_loop(c.clone(), s.clone(), res);
            }, t.abs() as u32);
        } else {
            let mut snake_game = snake_ref.borrow_mut();
            res(Ok(snake_game.get_results()));
        }
    };

    main_loop(snake_canvas, snake_game, res);
}

fn toggle_display(n: &Element) {
    if n.get_attribute("style").unwrap() == "display: none;" {
        n.set_attribute("style", "display: block;").expect("failed to set css attribute");
    } else {
        n.set_attribute("style", "display: none;").expect("failed to set css attribute");
    }
}

fn get_value(n: &InputElement) -> u32 {
    n.raw_value().parse().unwrap()
}

fn main() {
    initialize();

    let cfg = Rc::new(RefCell::new(Cfg {
        width: 20,
        height: 20,
        game_frame_rate: 10,
        frame_rate: 60,
        canvas: document()
            .query_selector("#snake-window")
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap(),
    }));

    let button = document().query_selector("#start-button").unwrap().unwrap();
    button.add_event_listener({
        let cfg = cfg.clone();
        move |_: ClickEvent| {
            run_snake_game(&cfg, |res| {
                match res {
                    Err(e) => {
                        web::window().alert(e.as_ref());
                    }
                    Ok(r) => {
                        let new_div = document().create_element("p").unwrap();
                        new_div.set_text_content(format!("score: {}", r.apples_eaten).as_ref());
                        web::document().query_selector("#scores").unwrap().unwrap().append_child(&new_div);
                    }
                }
            });
        }
    });

    let options = document().query_selector("#options").unwrap().unwrap();
    let show_options = document().query_selector("#options-button").unwrap().unwrap();
    show_options.add_event_listener({
        move |_: ClickEvent| {
            toggle_display(&options);
        }
    });
    let option_frame_rate: InputElement = document().query_selector("#frame-rate").unwrap().unwrap().try_into().unwrap();
    let option_game_frame_rate: InputElement = document().query_selector("#game-frame-rate").unwrap().unwrap().try_into().unwrap();
    let option_width: InputElement = document().query_selector("#width").unwrap().unwrap().try_into().unwrap();
    let option_height: InputElement = document().query_selector("#height").unwrap().unwrap().try_into().unwrap();
    let submit_options = document().query_selector("#submit-options").unwrap().unwrap();
    submit_options.add_event_listener({
        let cfg = cfg.clone();
        move |_: ClickEvent| {
            let mut cfg = cfg.borrow_mut();
            cfg.frame_rate = get_value(&option_frame_rate);
            cfg.game_frame_rate = get_value(&option_game_frame_rate);
            cfg.width = get_value(&option_width);
            cfg.height = get_value(&option_height);
        }
    });

    let scores = document().query_selector("#scores").unwrap().unwrap();
    let show_scores = document().query_selector("#scores-button").unwrap().unwrap();
    show_scores.add_event_listener({
        move |_: ClickEvent| {
            toggle_display(&scores);
        }
    });

    event_loop();
}
