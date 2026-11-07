use iced::advanced::Shell;

pub fn shell_merge_without_message<'a, 'b, MB, MO>(
    base: &mut Shell<'a, MB>,
    other: &Shell<'b, MO>,
) { 
    if other.is_layout_invalid() {
        base.invalidate_layout();
    }

    if other.are_widgets_invalid() {
        base.invalidate_widgets();
    }

    base.request_redraw_at(other.redraw_request());

    if other.is_event_captured() {
        base.capture_event();
    }

    base.request_input_method(other.input_method())
}
