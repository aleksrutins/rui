use crate::*;

#[derive(Clone)]
struct TextEditorState {
    cursor: usize,
    glyph_rects: Vec<LocalRect>,
    lines: Vec<LineMetrics>,
}

impl TextEditorState {
    fn fwd(&mut self, len: usize) {
        self.cursor += 1;
        if self.cursor >= len {
            self.cursor = len - 1;
        }
    }
    fn back(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn find_line(&self) -> usize {
        let mut i = 0;
        for line in &self.lines {
            if self.cursor >= line.glyph_start && self.cursor < line.glyph_end {
                break;
            }
            i += 1;
        }
        i
    }

    fn closest_in_range(
        &self,
        p: LocalPoint,
        range: std::ops::Range<usize>,
        rects: &[LocalRect],
    ) -> usize {
        let mut d = std::f32::MAX;
        let mut closest = 0;
        for i in range {
            let dp = rects[i].center().distance_to(p);
            if dp < d {
                closest = i;
                d = dp;
            }
        }
        closest
    }

    fn down(&mut self) {
        let p = self.glyph_rects[self.cursor].center();

        let line = self.find_line() + 1;
        if line < self.lines.len() {
            let metrics = self.lines[line];
            self.cursor =
                self.closest_in_range(p, metrics.glyph_start..metrics.glyph_end, &self.glyph_rects);
        }
    }

    fn up(&mut self) {
        let p = self.glyph_rects[self.cursor].center();

        let line = self.find_line();
        if line > 0 {
            let metrics = self.lines[line - 1];
            self.cursor =
                self.closest_in_range(p, metrics.glyph_start..metrics.glyph_end, &self.glyph_rects);
        }
    }

    fn key(&mut self, k: &KeyPress, text: String) -> String {
        match k {
            KeyPress::ArrowLeft => { self.back(); text }
            KeyPress::ArrowRight => { self.fwd(text.len()); text }
            KeyPress::ArrowUp => { self.up(); text }
            KeyPress::ArrowDown => { self.down(); text }
            KeyPress::Backspace => {
                if self.cursor > 0 {
                    let mut t = text.clone();
                    t.remove(self.cursor-1);
                    self.back();
                    t
                } else {
                    text
                }
            }
            KeyPress::Character(c) => {
                let mut t = text.clone();
                t.insert_str(self.cursor, c);
                self.cursor += c.len();
                t
            }
            KeyPress::Space => {
                let mut t = text.clone();
                t.insert(self.cursor, ' ');
                self.cursor += 1;
                t
            }
            KeyPress::Home => { self.cursor = 0; text },
            KeyPress::End => { self.cursor = text.len(); text },
            _ => text,
        }
    }
}

impl TextEditorState {
    fn new() -> Self {
        Self {
            cursor: 0,
            glyph_rects: vec![],
            lines: vec![],
        }
    }
}

/// Struct for `text_editor`.
pub struct TextEditor<F> {
    text: String,
    set_text: F,
}

impl<F> TextEditor<F>
where
    F: Fn(&mut Context, String) + 'static + Copy
{
    fn body(&self) -> impl View {
        let text = self.text.clone();
        let set_text = self.set_text;
        focus(move |has_focus| {
            let text = text.clone();
            state(TextEditorState::new, move |state, cx| {
                let cursor = cx[state].cursor;
                let text = text.clone();
                let text2 = text.clone();
                canvas(move |cx, rect, vger| {
                    vger.translate([0.0, rect.height()]);
                    let font_size = 18;
                    let break_width = Some(rect.width());

                    let rects = vger.glyph_positions(&text, font_size, break_width);
                    let lines = vger.line_metrics(&text, font_size, break_width);

                    vger.text(&text, font_size, TEXT_COLOR, break_width);

                    if has_focus {
                        let glyph_rect_paint = vger.color_paint(vger::Color::MAGENTA);
                        let r = rects[cursor];
                        vger.fill_rect(
                            LocalRect::new(r.origin, [2.0, 20.0].into()),
                            0.0,
                            glyph_rect_paint,
                        );
                    }

                    cx[state].glyph_rects = rects;
                    cx[state].lines = lines;
                })
                .key(move |cx, k| {
                    if has_focus {
                        let new_text = cx[state].key(&k, text2.clone());
                        set_text(cx, new_text);
                    }
                })
            })
        })
    }
}

impl<F> View for TextEditor<F>
where
    F: Fn(&mut Context, String) + 'static + Copy,
{
    body_view!();
}

impl<B> private::Sealed for TextEditor<B> {}

pub fn text_editor(text: String, set_text: impl Fn(&mut Context, String) + 'static + Copy) -> impl View {
    TextEditor { text, set_text }
}
