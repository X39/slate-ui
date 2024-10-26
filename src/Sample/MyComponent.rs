#[component("./MyComponent.xml")]
pub struct MyComponent {
    #[property]
    my_property: u8,
}

impl Component for MyComponent {

}

impl MyComponent {
    #[callback]
    fn on_button_clicked(&mut self, &mut sender: dyn Component) {
        self.my_property = self.my_property + 1;
    }
}
