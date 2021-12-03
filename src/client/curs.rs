// James Nichols, jtn4@pdx.edu, CS494p Internetworking Protocols
// Fall 2021 Term Project: IRC client
// curs.rs - User Interface implementation for an IRC Client

use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
use cursive::traits::With;
use cursive::Cursive;
use cursive::event::{Key};
use irclib::{*};
use tokio::sync::mpsc;


//Creates a new tab holding an IRC chat room and adds it to the UI. 
pub fn make_room(name: String, initial_text: String, tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
    //We do want to be able to address the subregions of a tab later, however the field used for
    //the title of a tab in the cursive-tabs library is also the field used by the cursive library
    //for addressing views by name. As we want to set a tab's title to the name of a room, this
    //means we must ensure names we use internally cannot collide with room names users may
    //provide. Names are at most 32 codepoints long, so by using 32+ character suffixes on names of
    //tab subregions they are ensured to never collide with any user provided room name.
    let body = TextView::new(initial_text).with_name(format!("{}-------------------------content",name))
        .scrollable()
        .scroll_strategy(ScrollStrategy::StickToBottom) //TODO: figure out how to reapply on incoming message if user has scrolled
        .full_screen();
    let listing = TextView::new("").with_name(format!("{}--------------------------people",name)).scrollable().full_height().fixed_width(20);

    let input = EditView::new().on_submit(move |s,text| { 
           //Clone the channel inside the closure, then pass that clone to the function, otherwise
           //we're trying to Move a value out of a closure that was originally captured from the
           //surrounding scope, which isn't permitted. Only natively local objects can be moved
           //into a function call within a closure in Rust.
            let txi = tx_packet_out.clone();
            let _ =  accept_input(s,text,txi, false);
        }).with_name(format!("{}---------------------------input", name)).full_width();
    //TODO: we would prefer to put the userlist on the right, but this libraries interaction
    //between fixed_width and full_screen elements isn't perfect, allows the chat area to push the
    //user list off the screen. Put userlist to left of chat area for now.
    //let sideways = LinearLayout::horizontal().child(Panel::new(body)).child(Panel::new(listing).fixed_width(22));
    let sideways = LinearLayout::horizontal().child(Panel::new(listing)).child(Panel::new(body));

    //The outermost layout view that will be directly contained by a tabview gets the raw room
    //name, permitting that user chosen name to show up in the tabview's user interface.
    LinearLayout::vertical().child(sideways).child(Panel::new(input).min_height(3)).full_screen().with_name(name)
}

//Creates a new tab holding an IRC Direct Message session and adds it to the UI. 
pub fn make_dm_room(name: String, initial_text: String, tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
    let body = TextView::new(initial_text).with_name(format!("DM:{}-------------------------content",name))
        .scrollable()
        .scroll_strategy(ScrollStrategy::StickToBottom)
        .full_screen();

    let input = EditView::new().on_submit(move |s,text| { 
            let txi = tx_packet_out.clone();
            let _ =  accept_input(s,text,txi, true);
        }).with_name(format!("DM:{}---------------------------input", name)).full_width();

    let tab_contents = LinearLayout::vertical().child(Panel::new(body)).child(Panel::new(input).min_height(3)).full_screen().with_name(format!("DM:{}",name));
    tab_contents
}

//Parses user input typed into a chat room / DM tab. Either reacts to specific commands or sends
//the message to the IRC server.
pub fn accept_input<'a>(s: &mut Cursive, text: &str, tx_packet_out: mpsc::Sender<irclib::SyncSendPack>, is_dm: bool) -> Result<'a, ()>{
  
    //Which tab are we in.
    let current_tab = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab = tab_controller.active_tab();
        current_tab.map(|tab_name| tab_name.to_owned())
        /*match current_tab {
            Some(tab_name) => Some(tab_name.to_owned()),
            None => None,
        }*/
    });
    match current_tab.unwrap() {
        Some(tab_name) => {
            //Clear input line.
            s.call_on_name(format!("{}---------------------------input", tab_name).as_str(), |edit: &mut EditView| {
                edit.set_content("");
            });
            //Look for commands
            if text.starts_with('/')  {
                //Appears to be a command.
                let mut tokens = text.split_whitespace();
                match tokens.next() {
                    Some("/enter") | Some("/join") => {
                        if let Some(room_name) = tokens.next() {
                                let outgoing = EnterRoomPacket::new(room_name.to_string())?;
                                tx_packet_out.blocking_send(outgoing.into())?;
                        };
                    },
                    Some("/whisper") | Some("/tell") => {
                        if let Some(user_name) = tokens.next() {
                                let message_start = text.find(user_name).unwrap() + user_name.len();
                                let body_text = &text[message_start..].trim_start().to_string();
                                let outgoing = DirectMessagePacket::new(&user_name.to_string(), body_text)?;
                                tx_packet_out.blocking_send(outgoing.into())?;

                                //create the conversation tab if necessary, and echo display
                                //outgoing message.
                                match s.find_name::<TextView>(format!("DM:{}-------------------------content", user_name).as_str()) {
                                    Some(mut convo) =>{
                                        convo.append(format!("You: {}\n", body_text));
                                    },
                                    None =>{
                                        s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
                                            tab_controller.add_tab(make_dm_room(user_name.into(),format!("You: {}\n", body_text), tx_packet_out));
                                        });
                                    },
                                };
                        };
                    },
                    Some("/status") => {
                        if let Some(user_name) =  tokens.next() {
                                let outgoing = QueryUserPacket::new(&user_name.to_string())?;
                                tx_packet_out.blocking_send(outgoing.into())?;
                        };
                    },
                    Some("/leave") => {
                        if !is_dm {
                            //If this is a chat room tab, Tell the server we are leaving the room
                            let outgoing = LeaveRoomPacket::new(tab_name)?;
                            tx_packet_out.blocking_send(outgoing.into())?;
                        }
                        s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
                            //TODO: fix dirty code, ignores Err returned by remove_tab and unwrap    
                            let current_tab = tab_controller.active_tab().unwrap().to_owned();
                            tab_controller.prev();
                            let _ = tab_controller.remove_tab(current_tab.as_str());
                        });
                    },
                    Some("/offer") => {
                        if let Some(recipient) = tokens.next() {
                                //get file name
                                //read file if popsible
                                //send offer packet.
                                //register pending transfer...
                        }

                        s.call_on_name(format!("{}-------------------------content", tab_name).as_str(), |content: &mut TextView| {
                            content.append("OFFERCOMMAND");
                        });
                    },
                    Some("/yell") | Some("/broadcast") => {
                        let body_text;
                        if text.starts_with("/yell") {
                            body_text = text.split_at(5).1.trim_start(); //assigning tuple element #1
                        } else {
                            body_text = text.split_at(10).1.trim_start();
                        }
                        if !body_text.is_empty() {
                            let outgoing = BroadcastMessagePacket::new(&body_text.to_string())?;
                            tx_packet_out.blocking_send(outgoing.into())?;
                        }
                    },
                    Some(_) | None => (),
                };
            } else {
                //Not a command, send text to a room or DM conversation!

                let body_text = text.trim_start();
                if !body_text.is_empty() {
                    if is_dm {
                        //local echo of outgoing text:
                        let outgoing = DirectMessagePacket::new(&tab_name[3..].to_string(), &body_text.to_string())?;
                        s.call_on_name(format!("{}-------------------------content", tab_name).as_str(), |content: &mut TextView| {
                            content.append(format!("You: {}\n", body_text));
                        });
                        tx_packet_out.blocking_send(outgoing.into())?;
                    } else {
                        let outgoing = SendMessagePacket::new(&tab_name, &body_text.to_string())?;
                        tx_packet_out.blocking_send(outgoing.into())?;
                    }

                }
            }

        },
        None => (println!("command to nonexistant room!")), //error state, how did we send commands when there is no room tab? TODO: Handle error meaningfully
    };
    Ok(())
}

//Handles keyboard shortcut to swap to the previous tab
pub fn switch_prev(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.prev();
    });

    focus_input_line(s);
}

//Handles keyboard shortcut to swap to the next tab
pub fn switch_next(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.next();
    });

    focus_input_line(s);
}

//After switching tabs, the input focus moves to the tab in the tab bar, move the focus back down
//to the text input line on the tab to let the user type their messages immediately.
pub fn focus_input_line(s: &mut Cursive){
    
    let current_tab_opt_opt = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab_opt = tab_controller.active_tab();
        current_tab_opt.map(|tab_name| tab_name.to_owned())
    });

    if let Some(tab_name) = current_tab_opt_opt.unwrap() {
            /*match s.focus_name(format!("{}---------------------------input", tab_name).as_str()) {
                Ok(_) => {},
                Err(_) => {},
            };*/
            let _ = s.focus_name(format!("{}---------------------------input", tab_name).as_str());
    };

}

//Builds the First tab in the UI interface, which offers the list of known rooms and options to
//create new chat rooms.
pub fn make_rooms_page(tx_packet_out: mpsc::Sender<irclib::SyncSendPack>) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
    let mut tx1 = tx_packet_out.clone();
    let mut tx2 = tx_packet_out.clone();
    let select = SelectView::<String>::new()
        .on_submit(move |s,n| {let _ = choose_room(s,n, & tx1);})
        .with_name("Rooms----------------------select")
        .scrollable()
        .scroll_strategy(ScrollStrategy::StickToBottom)
        .fixed_width(24)
        .full_height();
    let spacer = DummyView
    .full_height();
    let buttons = LinearLayout::vertical()
        .child(spacer)
        .child(Button::new("New",move |s|{let txi = tx2.clone();let _ =  new_room_button(s, txi);}))
        .child(DummyView)
        .child(Button::new("Join", move |s| {let _ = choose_room_button(s,& tx_packet_out);}));

    LinearLayout::horizontal()
        .child(Panel::new(select))
        .child(buttons)
        .full_height()
        .with_name("<Rooms>")
}

//Opens a dialog window to enter a new chatroom name
pub fn new_room_button<'a>(s: &mut Cursive, tx_packet_out:  mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {
    let txf = tx_packet_out.clone();
    fn send_new_packet(s: &mut Cursive, new_room_name: &str, tx_packet_out: mpsc::Sender<irclib::SyncSendPack>){
        match EnterRoomPacket::new(new_room_name.to_string()) {
           Ok(out) => {
                let _ = tx_packet_out.blocking_send(out.into());
                s.pop_layer();
           },
           Err(_) => {
                s.add_layer(Dialog::around(TextView::new("Sorry, room names don't allow spaces or certain directional characters."))
                    .title("Invalid Room Name")
                    .dismiss_button("Darn")
                    .wrap_with(OnEventView::new)
                    .on_pre_event(Key::Esc, |s| {
                        s.pop_layer();
                    })
                );
           },
        };
    }

    s.add_layer(Dialog::around(EditView::new()
                .on_submit(move |s,t| {
                    let txi = txf.clone();
                    let _ = send_new_packet(s,t,txi);
                })
                .with_name("New_Room_Name_Field--------------")
                .fixed_width(24))
            .title("Enter a new room name")
            .dismiss_button("Cancel")
            .button("Enter",move |s: &mut Cursive|{
                let new_room_name = s.call_on_name("New_Room_Name_Field--------------", |input_field: &mut EditView| {
                    input_field.get_content()
                }).unwrap();
                let txi = tx_packet_out.clone();
                let _ = send_new_packet(s,&new_room_name.to_string(), txi);
            })
            .wrap_with(OnEventView::new)
            .on_pre_event(Key::Esc, |s| {
                s.pop_layer();
            })
        );
    Ok(())
}


//Handles when a user clicks on a room name in the Rooms tab list OR highlights a room with the
//arrow keys and presses Enter.
pub fn choose_room<'a>(s: &mut Cursive, name: &str, tx_packet_out: & mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {
    let outgoing = EnterRoomPacket::new(name.to_string())?;
    tx_packet_out.blocking_send(outgoing.into())?;
    Ok(())
}

//Handles when a user clicks the Join button on the Rooms tab.
pub fn choose_room_button<'a>(s: &mut Cursive, tx_packet_out: & mpsc::Sender<irclib::SyncSendPack>) -> Result<'a, ()> {
    let select_ref = s.find_name::<SelectView<String>>("Rooms----------------------select").unwrap();
    if let Some(n) = select_ref.selection() {
            let outgoing = EnterRoomPacket::new(n.to_string())?;
            tx_packet_out.blocking_send(outgoing.into())?;
    };
    Ok(())
}

