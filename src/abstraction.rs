use crate::color::Color;
use crate::data::Size;

pub trait Renderer {
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, fill: Color);
    fn translate(&mut self, x: f32, y: f32);
}
pub trait Visual {
    fn draw(&self, renderer: &mut dyn Renderer);
}

pub trait Component {
    /// # Description
    /// Measures the size in layout required to render this.
    ///
    /// # Parameters
    /// - **available**: The available size that this element can give to child elements. Infinity may
    ///                  be passed so that the element can size to whatever content is available.
    ///
    /// # Returns
    /// The size in layout this element needs, including its child elements.
    ///
    /// # Remarks
    /// This method is called first in the chain of methods to determine which size a component
    /// actually does require for rendering.
    /// To implement this behavior, you have to:
    /// 1. Iterate the elements, taking part in the layout process, and call measure on all of them
    /// 2. Compute the components desired size, based upon the measurements of the child components
    ///    and the components own needs.
    ///
    /// The return value of this method should always be the full, desired size of this component,
    /// which then becomes the measure input for a parent component of this component. This process
    /// will bubble up until the root element is reached.
    ///
    /// If a child component return a larger size than passed in as *available*, it indicates that
    /// the child component needs more space. This may be handled by your implementation (eg.
    /// by introducing a scroll bar).
    ///
    /// # Note to implementors
    /// Make sure to always call every *measure* of every child!
    fn measure(&self, available: Size<f32>) -> Size<f32>;

    /// # Description
    /// Positions child elements and determines a final size for this element.
    ///
    /// # Parameters
    /// - **given**: The final size this control is given to arrange itself and its children.
    ///              Infinity may be passed to indicate eg. a scroll area.
    ///
    /// # Returns
    /// The actual size used.
    /// This value may be less than *given* in which case the calling component has to arrange this
    /// as it desires.
    ///
    /// # Remarks
    /// This method may never exceed the *given* size in any way.
    /// A renderer may clip to the available size and, from layout perspective, the control has
    /// been assigned the size passed in *given*.
    ///
    /// # Note to implementors
    /// Make sure to always call every *arrange* of every child!
    fn arrange(&mut self, given: Size<f32>) -> Size<f32>;

    /// # Description
    /// Handles the rendering of the components visuals.
    ///
    /// # Parameters
    /// - **dispatch**: Callback for rendering a visual.
    ///
    /// # Remarks
    /// This is the final method called by the renderer. It may be called multiple times
    /// and independent of any other methods. Everything in here must be final and should be
    /// rendered from cached values, completing as fast as possible.
    /// No logic is to be put in here. Make sure to calculate everything in eg. *arrange*.
    ///
    /// # Note to implementors
    /// With this must be fast, i, the doc author, do mean that this must be fast.
    /// This is the "rendering loop".
    fn render(&self, renderer: &mut dyn Renderer);
}

pub trait ContainerComponent : Component {
    fn add_child(&mut self, child: Box<dyn Component>);
}