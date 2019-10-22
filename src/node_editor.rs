use imgui::{im_str, ChildWindow, Condition, Id, ImStr, MouseButton, Ui, WindowFlags};

pub struct NodeEditor<'a> {
    id: Id<'a>,
}

impl<'a> NodeEditor<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Id<'a>>,
    {
        Self { id: id.into() }
    }

    pub fn build(self, ui: &Ui, viewport_pos: &mut [f32; 2]) {
        Canvas::new(self.id).build(ui, viewport_pos);
    }
}

pub struct Canvas<'a> {
    id: Id<'a>,
}

impl<'a> Canvas<'a> {
    pub fn new<T>(id: T) -> Self
    where
        T: Into<Id<'a>>,
    {
        Self { id: id.into() }
    }

    pub fn build(self, ui: &Ui, viewport_pos: &mut [f32; 2]) {
        ChildWindow::new(self.id)
            .flags(WindowFlags::NO_MOVE | WindowFlags::NO_SCROLL_WITH_MOUSE)
            .scroll_bar(false)
            .content_size([std::f32::MAX - 245_760.0, std::f32::MAX - 245_760.0])
            .build(ui, || {
                let draw_list = ui.get_window_draw_list();

                if ui.is_window_focused() {
                    let mouse_drag_delta =
                        ui.mouse_drag_delta_with_threshold(MouseButton::Left, 0.1);
                    viewport_pos[0] = viewport_pos[0] + mouse_drag_delta[0];
                    viewport_pos[1] = viewport_pos[1] + mouse_drag_delta[1];
                    ui.reset_mouse_drag_delta(MouseButton::Left);
                }
                let window_pos = ui.window_pos();
                let offset = [
                    window_pos[0] + viewport_pos[0],
                    window_pos[1] + viewport_pos[1],
                ];

                const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
                draw_list
                    .add_line(
                        [100.0 + offset[0], 100.0 + offset[1]],
                        [200.0 + offset[0], 200.0 + offset[1]],
                        WHITE,
                    )
                    .build();
            });
    }
}
