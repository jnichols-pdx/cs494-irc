use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
use cursive::traits::*;

fn main() {
	let mut siv = cursive::default();

	siv.add_global_callback(cursive::event::Event::CtrlChar('q'), |s| s.quit());
	//siv.add_global_callback(cursive::event::Key::Tab, |s| s.quit());
	//siv.add_global_callback('q', |s| s.quit());

/*    
    let mut panel = TabPanel::new();
    panel.add_tab(TextView::new("First room").with_name("First"));
    panel.add_tab(TextView::new("Wait one").with_name("Second"));
    panel.add_tab(TextView::new("Hello deary").with_name("DM: Your_Mom"));
    panel.add_tab(TextView::new("BUtts butts butts butts\n butts butts\n\tyeah?").with_name("R/Politics"));
    panel.add_tab(TextView::new("That's like.. just your opinion man").with_name("Third"));
    let panelv = panel.with_name("tabs").full_screen();


    //let entry = TextArea::new().with_name("text_area").full_width().fixed_height(2);
    let entry = EditView::new().with_name("text_area").full_width();//.fixed_height(2);


    let my_root = LinearLayout::vertical()
        .child(panelv)
        .child(Panel::new(entry))
        .full_screen();

    //let my_content = Panel::new(my_root).full_screen();
    //siv.add_layer(my_content);
    siv.add_fullscreen_layer(my_root);

	siv.run();
    */
    let mut panel = TabPanel::new();
    panel.add_tab(make_room("First".into(),"First room".into()));
    panel.add_tab(make_room("Second".into(),"Wait one".into()));
    panel.add_tab(make_room("DM: Your_MOM".into(),"Hello deary".into()));
    panel.add_tab(make_room("R/Politics".into(),"Butts butts butts butts\nbutts butts\n\tbutts yeah?".into()));
    panel.add_tab(make_room("Third".into(),"That's just, like, your opinion man.".into()));

    let panelv = panel.with_name("TABS__________________________32+").full_screen();


    siv.add_fullscreen_layer(panelv);
    
    //siv.add_layer(EditView::new().with_name("bruh").full_width());
    //
	siv.add_global_callback(cursive::event::Event::Ctrl(cursive::event::Key::Left), |s| {
        s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
                tab_controller.prev();
        });
    });
        
	siv.add_global_callback(cursive::event::Event::Ctrl(cursive::event::Key::Right), |s| {
        s.call_on_name("TABS__________________________32+", |tab_controller: &mut TabPanel| {
                tab_controller.next();
        });
    });

	siv.run();
}

fn make_room(name: String, initial_text: String) -> NamedView<ResizedView<cursive::views::LinearLayout>> {
    let body = TextView::new(initial_text).with_name(format!("{}-content",name)).full_screen();
    let listing = TextView::new("John\nMary\nJoseph").with_name(format!("{}-people",name)).full_height().fixed_width(20);
    let input = EditView::new().with_name(format!("{}-Input", name)).full_width();
    
    let sideways = LinearLayout::horizontal().child(Panel::new(body)).child(Panel::new(listing));

    let grouping = LinearLayout::vertical().child(sideways).child(Panel::new(input)).full_screen().with_name(name);
    grouping
}
