use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
//use cursive::traits::*;
use cursive::Cursive;
use irclib::{*};


pub fn make_room(name: String, initial_text: String) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
    //We do want to be able to address the subregions of a tab later, however the field used for
    //the title of a tab in the cursive-tabs library is also the field used by the cursive library
    //for addressing views by name. As we want to set a tab's title to the name of a room, this
    //means we must ensure names we use internally cannot collide with room names users may
    //provide. Names are at most 32 codepoints long, so by using 32+ character suffixes on names of
    //tab subregions they are ensured to never collide with any user provided room name.
    let body = TextView::new(initial_text).with_name(format!("{}-------------------------content",name)).full_screen();
    let listing = TextView::new("").with_name(format!("{}--------------------------people",name)).full_height().fixed_width(20);
    let input = EditView::new().on_submit(accept_input).with_name(format!("{}---------------------------input", name)).full_width();
    let sideways = LinearLayout::horizontal().child(Panel::new(body)).child(Panel::new(listing));

    //The outermost layout view that will be directly contained by a tabview gets the raw room
    //name, permitting that user chosen name to show up in the tabview's user interface.
    let tab_contents = LinearLayout::vertical().child(sideways).child(Panel::new(input)).full_screen().with_name(name);
    tab_contents
}

pub fn accept_input(s: &mut Cursive, text: &str){

    let current_tab = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab = tab_controller.active_tab();
        match current_tab {
            Some(tab_name) => Some(tab_name.to_owned()),
            None => None,
        }
    });

    match current_tab.unwrap() {
        Some(tab_name) => {
            s.call_on_name(format!("{}-------------------------content", tab_name).as_str(), |content: &mut TextView| {
                content.append(format!("\n{}", text));
            });
            s.call_on_name(format!("{}---------------------------input", tab_name).as_str(), |edit: &mut EditView| {
                edit.set_content("");
            });
        },
        None => (),
    };

}
pub fn switch_prev(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.prev();
    });

    focus_input_line(s);
}

pub fn switch_next(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.next();
    });

    focus_input_line(s);
}

pub fn focus_input_line(s: &mut Cursive){
    let current_tab_opt_opt = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab_opt = tab_controller.active_tab();
        match current_tab_opt {
            Some(tab_name) => Some(tab_name.to_owned()),
            None => None,
        }
    });

    match current_tab_opt_opt.unwrap() {
        Some(tab_name) => {
            match s.focus_name(format!("{}---------------------------input", tab_name).as_str()) {
                Ok(_) => {},
                Err(_) => {},
            };
        },
        None => (),
    };

}

pub fn make_rooms_page(tx_packet_out: tokio::sync::mpsc::Sender<irclib::SyncSendPack>) -> NamedView<ResizedView<cursive::views::LinearLayout>> {

    let mut tx1 = tx_packet_out.clone();
    let mut tx2 = tx_packet_out.clone();
    let select = SelectView::<String>::new()
        .on_submit(move |s,n| {let _ = choose_room(s,n, & tx2);})
        .with_name("Rooms----------------------select")
        .fixed_width(24)
        .full_height();
    let spacer = DummyView
    .full_height();
    let buttons = LinearLayout::vertical()
        .child(spacer)
        .child(Button::new("Join", move |s| {let _ = choose_room_button(s,& tx1);}));

    let pane = LinearLayout::horizontal()
        .child(Panel::new(select))
        .child(buttons)
        .full_height()
        .with_name("<Rooms>");

    pane

}

pub fn choose_room<'a>(s: &mut Cursive, name: &str, tx_packet_out: & tokio::sync::mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {
    let outgoing = EnterRoomPacket::new(&name.to_string())?;
    tx_packet_out.blocking_send(outgoing.into())?;
    Ok(())
}

pub fn choose_room_button<'a>(s: &mut Cursive, tx_packet_out: & tokio::sync::mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {

    let select_ref = s.find_name::<SelectView<String>>("Rooms----------------------select").unwrap();
    match select_ref.selection() {
        Some(n) =>{
            let outgoing = EnterRoomPacket::new(&n.to_string())?;
            tx_packet_out.blocking_send(outgoing.into())?;
        },
        None =>{}
    };
    Ok(())
}

