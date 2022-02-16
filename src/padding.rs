use crate::*;

pub struct Padding<V: View> {
    child: V,
}

impl<V> View for Padding<V>
where
    V: View,
{
    fn print(&self, id: ViewID, cx: &mut Context) {
        println!("Padding {{");
        (self.child).print(id.child(0), cx);
        println!("}}");
    }

    fn process(&self, event: &Event, id: ViewID, cx: &mut Context) {
        self.child.process(event, id.child(0), cx);
    }

    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER) {
        self.child.draw(id, cx, vger);
    }
}

impl<V> Padding<V>
where
    V: View + 'static,
{
    pub fn new(child: V) -> Self {
        Self { child: child }
    }
}