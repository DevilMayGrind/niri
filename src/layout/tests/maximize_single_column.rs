use super::*;

fn make_options() -> Options {
    Options {
        layout: niri_config::Layout {
            gaps: 0.,
            maximize_single_column: true,
            default_column_width: Some(PresetSize::Proportion(0.5)),
            ..Default::default()
        },
        ..Options::default()
    }
}

fn add_window(layout: &mut Layout<TestWindow>, id: usize) {
    let win = TestWindow::new(TestWindowParams::new(id));
    layout.add_window(
        win,
        AddWindowTarget::Auto,
        Some(PresetSize::Proportion(0.5)),
        None,
        false,
        false,
        ActivateWindow::default(),
    );
}

/// Requested window widths, sorted by window id.
fn requested_widths(layout: &Layout<TestWindow>) -> Vec<(usize, i32)> {
    let mut rv: Vec<_> = layout
        .active_workspace()
        .unwrap()
        .windows()
        .map(|win| (win.0.id, win.requested_size().unwrap().w))
        .collect();
    rv.sort();
    rv
}

fn set_up(ops: impl IntoIterator<Item = Op>) -> Layout<TestWindow> {
    let mut layout = check_ops_with_options(make_options(), [Op::AddOutput(1)]);
    add_window(&mut layout, 1);
    check_ops_on_layout(&mut layout, ops);
    layout
}

#[test]
fn single_window_is_full_width() {
    let layout = set_up([Op::Communicate(1)]);
    assert_eq!(requested_widths(&layout), [(1, 1280)]);
}

#[test]
fn second_window_restores_default_width() {
    let mut layout = set_up([Op::Communicate(1)]);
    add_window(&mut layout, 2);
    check_ops_on_layout(&mut layout, [Op::Communicate(1), Op::Communicate(2)]);
    assert_eq!(requested_widths(&layout), [(1, 640), (2, 640)]);
}

#[test]
fn closing_second_window_maximizes_again() {
    let mut layout = set_up([Op::Communicate(1)]);
    add_window(&mut layout, 2);
    check_ops_on_layout(
        &mut layout,
        [Op::Communicate(1), Op::Communicate(2), Op::CloseWindow(2)],
    );
    check_ops_on_layout(&mut layout, [Op::Communicate(1)]);
    assert_eq!(requested_widths(&layout), [(1, 1280)]);
}

#[test]
fn two_windows_in_one_column_are_full_width() {
    let mut layout = set_up([Op::Communicate(1)]);
    add_window(&mut layout, 2);
    check_ops_on_layout(
        &mut layout,
        [
            Op::ConsumeOrExpelWindowLeft { id: None },
            Op::Communicate(1),
            Op::Communicate(2),
        ],
    );
    assert_eq!(requested_widths(&layout), [(1, 1280), (2, 1280)]);
}

#[test]
fn manual_resize_overrides_maximize() {
    let layout = set_up([
        Op::Communicate(1),
        Op::SetColumnWidth(SizeChange::SetProportion(75.)),
        Op::Communicate(1),
    ]);
    assert_eq!(requested_widths(&layout), [(1, 960)]);
}

#[test]
fn maximize_column_unmaximizes_sole_column() {
    let layout = set_up([Op::Communicate(1), Op::MaximizeColumn, Op::Communicate(1)]);
    assert_eq!(requested_widths(&layout), [(1, 640)]);
}

#[test]
fn manual_resize_opt_out_resets_after_another_window() {
    let mut layout = set_up([
        Op::Communicate(1),
        Op::SetColumnWidth(SizeChange::SetProportion(75.)),
        Op::Communicate(1),
    ]);
    add_window(&mut layout, 2);
    check_ops_on_layout(
        &mut layout,
        [Op::Communicate(1), Op::Communicate(2), Op::CloseWindow(2)],
    );
    check_ops_on_layout(&mut layout, [Op::Communicate(1)]);
    assert_eq!(requested_widths(&layout), [(1, 1280)]);
}

#[test]
fn option_disabled_keeps_default_width() {
    let mut options = make_options();
    options.layout.maximize_single_column = false;
    let mut layout = check_ops_with_options(options, [Op::AddOutput(1)]);
    add_window(&mut layout, 1);
    check_ops_on_layout(&mut layout, [Op::Communicate(1)]);
    assert_eq!(requested_widths(&layout), [(1, 640)]);
}
