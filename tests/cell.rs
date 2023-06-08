use text_grid::{Cell, CellSource, CellStyle, CellsFormatter, CellsSource, HorizontalAlignment};

#[test]
fn impl_cell() {
    struct X(String);

    impl CellSource for X {
        fn fmt(&self, s: &mut String) {
            s.push_str(&self.0);
        }
        fn style(&self) -> CellStyle {
            CellStyle::new().align_h(HorizontalAlignment::Right)
        }
    }
    impl CellsSource for X {
        fn fmt(f: &mut CellsFormatter<&Self>) {
            f.content(|x| Cell::new(*x));
        }
    }
}
