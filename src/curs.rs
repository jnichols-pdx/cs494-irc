use cursive::views::TextView;
use cursive_tabs::TabPanel;
use cursive::view::*;
use cursive::views::*;
use cursive::traits::*;

fn main() {
	let mut siv = cursive::default();

	//siv.add_global_callback('q', |s| s.quit());

	//siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
    
    let mut panel = TabPanel::new();
    panel.add_tab(TextView::new("First room").with_name("First"));
    panel.add_tab(TextView::new("Hello deary").with_name("DM: Your_Mom"));
    panel.add_tab(TextView::new("Hello deary").with_name("DM: Your_Mom"));
    panel.add_tab(TextView::new("Hello deary").with_name("DM: Your_Mom"));
    panel.add_tab(TextView::new("Hello deary").with_name("DM: Your_Mom"));
    let panelv = panel.full_screen();


    //let entry = TextArea::new().with_name("text_area").full_width().fixed_height(2);
    let entry = EditView::new().with_name("text_area").full_width();//.fixed_height(2);


    let my_root = LinearLayout::vertical()
        .child(panelv)
        .child(Panel::new(entry))
        .full_screen();

    //let my_content = Panel::new(my_root).full_screen();
    //siv.add_layer(my_content);
    siv.add_layer(my_root);

	siv.run();
}
