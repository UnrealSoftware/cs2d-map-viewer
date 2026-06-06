use strum::FromRepr;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRepr)]
#[repr(u16)]
pub enum UiIcon {
    #[default]
    X = 0,
    New = 1,
    Load = 2,
    Save = 3,
    Options = 4,
    Draw = 5,

    Rect = 6,
    Fill = 7,
    Tiles = 8,
    Entity = 9,
    View = 10,
    DoubleArrow = 11,

    Circle = 12,
    Question = 13,
    QuestionDot = 14,
    ArrowUp = 15,
    ArrowRight = 16,
    ArrowDown = 17,

    ArrowLeft = 18,
    Star = 19,
    ArrowUpLeft = 20,
    ArrowUpRight = 21,
    ArrowDownRight = 22,
    ArrowDownLeft = 23
}