use iced::{
    alignment::{Horizontal, Vertical},
    Color, Element, Length, Padding, Sandbox, Settings,
};
use iced_grid::{grid, grid_row, Strategy};
use iced_widget::{checkbox, column, container, pick_list, radio, row, slider, text};

struct App {
    horizontal_alignment: Horizontal,
    vertical_alignemnt: Vertical,
    column_spacing: f32,
    row_spacing: f32,
    row_strategy: Strategy,
    column_strategy: Strategy,
    padding: Padding,
    debug_layout: bool,
}

#[derive(Debug, Clone)]
enum Message {
    HorizontalAlignment(Horizontal),
    VerticalAlignment(Vertical),
    ColumnSpacing(f32),
    RowSpacing(f32),
    RowStrategy(Strategy),
    ColumnStrategy(Strategy),
    Padding(Padding),
    DebugToggled(bool),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            horizontal_alignment: Horizontal::Left,
            vertical_alignemnt: Vertical::Center,
            column_spacing: 5.0,
            row_spacing: 5.0,
            row_strategy: Strategy::Minimum,
            column_strategy: Strategy::Minimum,
            padding: Padding::ZERO,
            debug_layout: false,
        }
    }

    fn title(&self) -> String {
        "Iced Grid widget example".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::HorizontalAlignment(align) => self.horizontal_alignment = align,
            Message::VerticalAlignment(align) => self.vertical_alignemnt = align,
            Message::ColumnSpacing(spacing) => self.column_spacing = spacing,
            Message::RowSpacing(spacing) => self.row_spacing = spacing,
            Message::RowStrategy(strategy) => self.row_strategy = strategy,
            Message::ColumnStrategy(strategy) => self.column_strategy = strategy,
            Message::Padding(value) => self.padding = value,
            Message::DebugToggled(enabled) => self.debug_layout = enabled,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let horizontal_align_pick = pick_list(
            HORIZONTAL_ALIGNMENTS
                .iter()
                .map(horizontal_align_to_string)
                .collect::<Vec<_>>(),
            Some(horizontal_align_to_string(&self.horizontal_alignment)),
            |selected| Message::HorizontalAlignment(string_to_horizontal_align(&selected)),
        );

        let vertical_align_pick = pick_list(
            VERTICAL_ALIGNMENTS
                .iter()
                .map(vertical_alignment_to_string)
                .collect::<Vec<_>>(),
            Some(vertical_alignment_to_string(&self.vertical_alignemnt)),
            |selected| Message::VerticalAlignment(string_to_vertical_align(&selected)),
        );

        let row_spacing_slider =
            slider(0.0..=100.0, self.row_spacing, Message::RowSpacing).width(200.0);
        let col_spacing_slider =
            slider(0.0..=100.0, self.column_spacing, Message::ColumnSpacing).width(200.0);

        let debug_mode_check = checkbox("", self.debug_layout, Message::DebugToggled);

        let row_height_radio = column(
            STRATEGIES
                .iter()
                .map(|strat| {
                    let name = strategy_to_string(&strat);
                    radio(name, strat, Some(&self.row_strategy), |click| {
                        Message::RowStrategy(click.clone())
                    })
                })
                .map(Element::from)
                .collect(),
        );

        let col_width_radio = row(STRATEGIES
            .iter()
            .map(|strat| {
                let name = strategy_to_string(&strat);
                radio(name, strat, Some(&self.column_strategy), |click| {
                    Message::ColumnStrategy(click.clone())
                })
            })
            .map(Element::from)
            .collect())
        .spacing(10);

        let padding_slider = slider(0.0..=50.0, self.padding.left, |value| {
            Message::Padding(Padding::new(value))
        })
        .width(100);

        let grid = grid!(
            grid_row!(text("Horizontal alignment"), horizontal_align_pick),
            grid_row!(text("Vertical alignment"), vertical_align_pick),
            grid_row!(text("Row spacing"), row_spacing_slider),
            grid_row!(text("Column spacing"), col_spacing_slider),
            grid_row!(text("Row height"), row_height_radio),
            grid_row!(text("Column width"), col_width_radio),
            grid_row!(text("Padding"), padding_slider),
            grid_row!(text("Debug mode"), debug_mode_check)
        )
        .horizontal_alignment(self.horizontal_alignment)
        .vertical_alignment(self.vertical_alignemnt)
        .row_spacing(self.row_spacing)
        .column_spacing(self.column_spacing)
        .row_height_strategy(self.row_strategy.clone())
        .column_width_strategy(self.column_strategy.clone());

        let mut contents = Element::from(container(grid).padding(self.padding));
        if self.debug_layout {
            contents = contents.explain(Color::BLACK);
        }
        container(contents)
            .padding(self.padding)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

const HORIZONTAL_ALIGNMENTS: [Horizontal; 3] =
    [Horizontal::Left, Horizontal::Center, Horizontal::Right];

const VERTICAL_ALIGNMENTS: [Vertical; 3] = [Vertical::Top, Vertical::Center, Vertical::Bottom];

const STRATEGIES: [Strategy; 2] = [
    Strategy::Minimum,
    Strategy::Equal,
    // Strategy::Fixed(Pixels(30.0)),
];

fn horizontal_align_to_string(alignment: &Horizontal) -> String {
    match alignment {
        Horizontal::Left => "Left",
        Horizontal::Center => "Center",
        Horizontal::Right => "Right",
    }
    .to_string()
}

fn string_to_horizontal_align(input: &str) -> Horizontal {
    match input {
        "Left" => Horizontal::Left,
        "Center" => Horizontal::Center,
        "Right" => Horizontal::Right,
        _ => panic!(),
    }
}

fn vertical_alignment_to_string(alignment: &Vertical) -> String {
    match alignment {
        Vertical::Top => "Top",
        Vertical::Center => "Center",
        Vertical::Bottom => "Bottom",
    }
    .to_string()
}

fn string_to_vertical_align(input: &str) -> Vertical {
    match input {
        "Top" => Vertical::Top,
        "Center" => Vertical::Center,
        "Bottom" => Vertical::Bottom,
        _ => panic!(),
    }
}

fn strategy_to_string(strategy: &Strategy) -> String {
    match strategy {
        Strategy::Minimum => "Minimum",
        Strategy::Equal => "Equal",
        // Strategy::Fixed(_) => "Fixed",
    }
    .to_string()
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
