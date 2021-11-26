use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
use cursive::traits::*;
use cursive::Cursive;

fn main() {
	let mut siv = cursive::default();

    //Global callback to exit the program, will need to change this to pass a message to our core
    //program code to do a graceful disconnect.
	siv.add_global_callback(cursive::event::Event::CtrlChar('q'), |s| s.quit());

    let mut panel = TabPanel::new();
    panel.add_tab(make_room("First".into(),"First room".into()));
    panel.add_tab(make_room("Second".into(),"Wait one".into()));
    panel.add_tab(make_room("DM: Your_MOM".into(),"Hello deary".into()));
    panel.add_tab(make_room("R/Politics".into(),"Butts butts butts butts\nbutts butts\n\tbutts yeah?".into()));
    panel.add_tab(make_room("Third".into(),"That's just, like, your opinion man.".into()));

    let panelv = panel.with_name("TABS__________________________32+").full_screen();

    siv.add_fullscreen_layer(panelv);
    
    //Callbacks to capture ctrl-right and ctrl-left and switch tabs accordingly.
	siv.add_global_callback(cursive::event::Event::Ctrl(cursive::event::Key::Left),switch_prev);
        
	siv.add_global_callback(cursive::event::Event::Ctrl(cursive::event::Key::Right),switch_next);

	siv.run();
}

fn make_room(name: String, initial_text: String) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
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

fn accept_input(s: &mut Cursive, text: &str){

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

fn switch_next(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.next();
    });

    let current_tab_opt_opt = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab_opt = tab_controller.active_tab();
        match current_tab_opt {
            Some(tab_name) => Some(tab_name.to_owned()),
            None => None,
        }
    });

    //Call_on_name returns Option, as does the call to active_tab. We expect to always have a
    //tabview that call_on_name can access, so unwrap the outer Option before passing to match.
    match current_tab_opt_opt.unwrap() {
        Some(tab_name) => {
            s.focus_name(format!("{}---------------------------input", tab_name).as_str()).expect("Input couldn't be found");
        },
        None => (),
    };
}

fn switch_prev(s: &mut Cursive){
    s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
            tab_controller.prev();
    });

    let current_tab_opt_opt = s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel|  {
        let current_tab_opt = tab_controller.active_tab();
        match current_tab_opt {
            Some(tab_name) => Some(tab_name.to_owned()),
            None => None,
        }
    });

    //Call_on_name returns Option, as does the call to active_tab. We expect to always have a
    //tabview that call_on_name can access, so unwrap the outer Option before passing to match.
    match current_tab_opt_opt.unwrap() {
        Some(tab_name) => {
            s.focus_name(format!("{}---------------------------input", tab_name).as_str()).expect("Input couldn't be found");
        },
        None => (),
    };
}
