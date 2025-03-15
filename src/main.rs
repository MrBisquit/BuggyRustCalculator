use fltk::{app, button::Button, group::Pack, input::Input, enums::Color, frame::Frame, prelude::*, window::Window};
use fltk::group::PackType;
use fltk_theme::{color_themes, widget_themes, ThemeType, WidgetTheme};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    println!("Opening window...");

    let app = app::App::default();
    let widget_theme = WidgetTheme::new(ThemeType::Greybird);
    widget_theme.apply();
    let mut wind = Window::new(100, 100, 300, 405, "Calculator")
        .center_screen();
    wind.make_resizable(true);

    let mut inp = Input::new(5, 15, 290, 60, "");
    let inp_ref = Rc::new(RefCell::new(inp));
    inp_ref.borrow_mut().set_text_size(30);
    inp_ref.borrow_mut().set_readonly(true);
    inp_ref.borrow_mut().set_cursor_color(Color::from_rgba_tuple((0, 0, 0, 0)));

    let mut ind_plu = Rc::new(RefCell::new(Frame::new(5, 5, 5, 5, "+")));
    ind_plu.borrow_mut().set_label_color(Color::rgb_color(100, 100, 100));
    let mut ind_min = Rc::new(RefCell::new(Frame::new(15, 5, 5, 5, "-")));
    ind_min.borrow_mut().set_label_color(Color::rgb_color(155, 155, 155));
    let mut ind_las = Rc::new(RefCell::new(Frame::new(25, 5, 50, 5, "0")));

    let mut hpack = Pack::new(5, 35 + 45,295, 60, "");
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(5);
    let mut but_space_1 = Frame::default().with_size(69 * 2 + 5, 0); // Spacing
    let mut but_bac = Button::default().with_size(69, 0).with_label("DEL");
    but_bac.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_c = Button::default().with_size(69, 0).with_label("C");
    but_c.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    let mut hpack = Pack::new(5, 35 + 60 + 5 + 45,295, 60, "");
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(5);
    let mut but_7 = Button::default().with_size(69, 0).with_label("7");
    but_7.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_8 = Button::default().with_size(69, 0).with_label("8");
    but_8.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_9 = Button::default().with_size(69, 0).with_label("9");
    but_9.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_div = Button::default().with_size(69, 0).with_label("÷");
    but_div.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    let mut hpack = Pack::new(5, 35 + (60 * 2) + (5 * 2) + 45,295, 60, "");
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(5);
    let mut but_4 = Button::default().with_size(69, 0).with_label("4");
    but_4.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_5 = Button::default().with_size(69, 0).with_label("5");
    but_5.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_6 = Button::default().with_size(69, 0).with_label("6");
    but_6.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_tim = Button::default().with_size(69, 0).with_label("×");
    but_tim.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    let mut hpack = Pack::new(5, 35 + (60 * 3) + (5 * 3) + 45,295, 60, "");
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(5);
    let mut but_1 = Button::default().with_size(69, 0).with_label("1");
    but_1.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_2 = Button::default().with_size(69, 0).with_label("2");
    but_2.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_3 = Button::default().with_size(69, 0).with_label("3");
    but_3.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_min = Button::default().with_size(69, 0).with_label("-");
    but_min.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    let mut hpack = Pack::new(5, 35 + (60 * 4) + (5 * 4) + 45,295, 60, "");
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(5);
    let mut but_tog = Button::default().with_size(69, 0).with_label("+/-"); // Toggles + or -
    but_tog.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_0 = Button::default().with_size(69, 0).with_label("0");
    but_0.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_equ = Button::default().with_size(69, 0).with_label("=");//.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    but_equ.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    let mut but_add = Button::default().with_size(69, 0).with_label("+");
    but_add.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    hpack.end();

    wind.end();
    wind.show();

    let mut num = Rc::new(RefCell::new(0));
    let mut pom = Rc::new(RefCell::new(true)); // Plus = true, Minus = false
    let mut str = Rc::new(RefCell::new(String::from("0")));

    fn set_pom(pom: bool, ind_plu: &mut Frame, ind_min: &mut Frame) {
        if(pom) {
            ind_plu.set_label_color(Color::rgb_color(100, 100, 100));
            ind_min.set_label_color(Color::rgb_color(155, 155, 155));
        } else {
            ind_plu.set_label_color(Color::rgb_color(155, 155, 155));
            ind_min.set_label_color(Color::rgb_color(100, 100, 100));
        }
    }

    fn add_num(num: u32, str: &mut String, inp: &mut Input) -> bool {
        if (str == "0" && num == 0) {
            return false;
        }

        if(str == "0") {
            let mut result = format!("{}", num.to_string());
            *str = result;
            inp.set_value(str.as_mut_str());
        } else {
            let mut result = format!("{}{}", str, num.to_string());
            str.push_str(&num.to_string());
            inp.set_value(str.as_mut_str());
        }

        return true;
    }

    fn calc_add(pom: bool, num: &mut u32, str: &mut String, inp_last: &mut Frame, inp: &mut Input) {
        let last = inp_last.label().parse::<u32>().unwrap();
        if(pom) {
            let result = last + num.to_string().parse::<u32>().unwrap();
            inp_last.set_label(result.to_string().as_mut_str());
        } else {
            let result = last + (num.to_string().parse::<u32>().unwrap() * "-1".parse::<u32>().unwrap());
            inp_last.set_label(result.to_string().as_mut_str());
        }

        inp.set_value("0");
    }

    let str_clone_7 = Rc::clone(&str);
    let inp_clone_7 = Rc::clone(&inp_ref);
    but_7.set_callback(move |_| {
        add_num(7, &mut str_clone_7.borrow_mut(), &mut inp_clone_7.borrow_mut());
    });

    let str_clone_8 = Rc::clone(&str);
    let inp_clone_8 = Rc::clone(&inp_ref);
    but_8.set_callback(move |_| {
        add_num(8, &mut str_clone_8.borrow_mut(), &mut inp_clone_8.borrow_mut());
    });

    let str_clone_9 = Rc::clone(&str);
    let inp_clone_9 = Rc::clone(&inp_ref);
    but_9.set_callback(move |_| {
        add_num(9, &mut str_clone_9.borrow_mut(), &mut inp_clone_9.borrow_mut());
    });

    let str_clone_4 = Rc::clone(&str);
    let inp_clone_4 = Rc::clone(&inp_ref);
    but_4.set_callback(move |_| {
        add_num(4, &mut str_clone_4.borrow_mut(), &mut inp_clone_4.borrow_mut());
    });

    let str_clone_5 = Rc::clone(&str);
    let inp_clone_5 = Rc::clone(&inp_ref);
    but_5.set_callback(move |_| {
        add_num(5, &mut str_clone_5.borrow_mut(), &mut inp_clone_5.borrow_mut());
    });

    let str_clone_6 = Rc::clone(&str);
    let inp_clone_6 = Rc::clone(&inp_ref);
    but_6.set_callback(move |_| {
        add_num(6, &mut str_clone_6.borrow_mut(), &mut inp_clone_6.borrow_mut());
    });

    let str_clone_1 = Rc::clone(&str);
    let inp_clone_1 = Rc::clone(&inp_ref);
    but_1.set_callback(move |_| {
        add_num(1, &mut str_clone_1.borrow_mut(), &mut inp_clone_1.borrow_mut());
    });

    let str_clone_2 = Rc::clone(&str);
    let inp_clone_2 = Rc::clone(&inp_ref);
    but_2.set_callback(move |_| {
        add_num(2, &mut str_clone_2.borrow_mut(), &mut inp_clone_2.borrow_mut());
    });

    let str_clone_3 = Rc::clone(&str);
    let inp_clone_3 = Rc::clone(&inp_ref);
    but_3.set_callback(move |_| {
        add_num(3, &mut str_clone_3.borrow_mut(), &mut inp_clone_3.borrow_mut());
    });

    let pom_clone = Rc::clone(&pom);
    let ind_plu_clone = Rc::clone(&ind_plu);
    let ind_min_clone = Rc::clone(&ind_min);
    but_tog.set_callback(move |_| {
        let mut pom_mut = pom_clone.borrow_mut();
        let mut ind_plu_mut = ind_plu_clone.borrow_mut();
        let mut ind_min_mut = ind_min_clone.borrow_mut();
        *pom_mut = !*pom_mut;
        set_pom(*pom_mut, &mut ind_plu_mut, &mut ind_min_mut);
    });

    let str_clone_0 = Rc::clone(&str);
    let inp_clone_0 = Rc::clone(&inp_ref);
    but_0.set_callback(move |_| {
        add_num(0, &mut str_clone_0.borrow_mut(), &mut inp_clone_0.borrow_mut());
    });

    let pom_clone_add = Rc::clone(&pom);
    let num_clone_add = Rc::clone(&num);
    let str_clone_add = Rc::clone(&str);
    let ind_las_clone_add = Rc::clone(&ind_las);
    let inp_clone_add = Rc::clone(&inp_ref);
    but_add.set_callback(move |_| {
        let mut pom_mut = pom_clone_add.borrow_mut();
        let mut num_mut = num_clone_add.borrow_mut();
        let mut str_mut = str_clone_add.borrow_mut();
        let mut ind_las_mut = ind_las_clone_add.borrow_mut();
        let mut inp_mut = inp_clone_add.borrow_mut();

        calc_add(*pom_mut, &mut num_mut, &mut str_mut, &mut ind_las_mut, &mut inp_mut);
    });

    app.run().unwrap();
}