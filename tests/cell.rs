use text_grid::{Cell, CellStyle, Cells, CellsFormatter, HorizontalAlignment, RawCell};

#[test]
fn impl_cell() {
    struct X(String);

    impl RawCell for X {
        fn fmt(&self, s: &mut String) {
            s.push_str(&self.0);
        }
        fn style(&self) -> CellStyle {
            CellStyle::new().align_h(HorizontalAlignment::Right)
        }
    }
    impl Cells for X {
        fn fmt(f: &mut CellsFormatter<Self>) {
            f.content(Cell::new);
        }
    }
}
