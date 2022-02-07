use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Button, Dialog, OnEventView, DummyView, SelectView, TextView, EditView, LinearLayout};
use cursive::Cursive;

// We'll use a SelectView here.
//
// A SelectView is a scrollable list of items, from which the user can select
// one.

fn main() {
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    let content = include_str!("cities.txt");
    select.add_all_str(content.lines());

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(show_next_window);

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    let buttons = LinearLayout::vertical()
        .child(DummyView.fixed_height(4))
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(Button::new("Quit", Cursive::quit));

    let buttons2 = LinearLayout::vertical()
        .child(DummyView.fixed_height(4))
        .child(DummyView.fixed_width(15))
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(Button::new("Quit", Cursive::quit));



    let mut siv = cursive::default();

    // Let's add a ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).


    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(TextView::new("Title").h_align(HAlign::Center))
            .child(buttons2)
            .child(
                // Popup-mode SelectView are small enough to fit here
                SelectView::new()
                    .item_str("0-18")
                    .item_str("19-30")
                    .item_str("31-40")
                    .item_str("41+")
            )

            //.child(DummyView.fixed_width(4))
            .child(select.scrollable()))
        .title("Select a profile"));

    siv.run();
}

// Let's put the callback in a separate function to keep it clean,
// but it's not required.
fn show_next_window(siv: &mut Cursive, city: &str) {
    siv.pop_layer();
    let text = format!("{} is a great city!", city);
    siv.add_layer(
        Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()),
    );
}

fn add_name(s: &mut Cursive) {

}

fn on_submit(s: &mut Cursive, name: &str) {

}

fn delete_name(s: &mut Cursive) {

}