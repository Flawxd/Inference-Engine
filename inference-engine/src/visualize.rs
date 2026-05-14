use crate::types::{ProofTree, Term};
use ab_glyph::{FontRef, PxScale};
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use image::ImageBuffer;

const FONT_BYTES: &[u8] = include_bytes!("../assets/DejaVuSansMono.ttf");
const FONT_SCALE: f32 = 14.0;
const CHAR_WIDTH: u32 = 9;
const LINE_HEIGHT: u32 = 18;
const PAD_X: u32 = 12;
const PAD_Y: u32 = 8;
const H_GAP: u32 = 24;
const V_GAP: u32 = 48;
const MARGIN: u32 = 20;

const BG: Rgb<u8> = Rgb([255, 255, 255]);
const LEAF_COLOR: Rgb<u8> = Rgb([200, 240, 200]);
const NODE_COLOR: Rgb<u8> = Rgb([200, 220, 255]);
const BORDER_COLOR: Rgb<u8> = Rgb([80, 80, 80]);
const TEXT_COLOR: Rgb<u8> = Rgb([0, 0, 0]);
const LINE_COLOR: Rgb<u8> = Rgb([100, 100, 100]);

struct NodeLayout {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    lines: Vec<String>,
    children: Vec<NodeLayout>,
}

pub fn proof_tree_to_png(tree: &ProofTree, output_path: &str) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, String> {
    let layout = compute_layout(tree);

    let total_w = subtree_width(&layout) + MARGIN * 2;
    let total_h = subtree_height(&layout) + MARGIN * 2;

    let mut img = RgbImage::from_pixel(total_w, total_h, BG);
    let font = FontRef::try_from_slice(FONT_BYTES).map_err(|e| format!("Failed to load font: {}", e))?;
    let scale = PxScale::from(FONT_SCALE);

    draw_node(&mut img, &layout, MARGIN, MARGIN, &font, scale);

    img.save(output_path).map_err(|e| format!("Failed to save PNG: {}", e))?;
    Ok(img)
}

fn node_lines(tree: &ProofTree) -> Vec<String> {
    let goal = format_term(&tree.goal);
    let rule = match &tree.rule_used {
        Some(r) => format!("[{}]", r),
        None => "[fact]".into(),
    };
    vec![goal, rule]
}

fn node_size(lines: &[String]) -> (u32, u32) {
    let max_chars = lines.iter().map(|l| l.len() as u32).max().unwrap_or(0);
    let w = max_chars * CHAR_WIDTH + PAD_X * 2;
    let h = lines.len() as u32 * LINE_HEIGHT + PAD_Y * 2;
    (w, h)
}

fn compute_layout(tree: &ProofTree) -> NodeLayout {
    let lines = node_lines(tree);
    let (w, h) = node_size(&lines);

    let child_layouts: Vec<NodeLayout> = tree.subgoals.iter().map(compute_layout).collect();

    let children_total_w: u32 = if child_layouts.is_empty() {
        0
    } else {
        child_layouts.iter().map(|c| subtree_width(c)).sum::<u32>()
            + (child_layouts.len() as u32 - 1) * H_GAP
    };

    let node_x = if children_total_w > w {
        (children_total_w - w) / 2
    } else {
        0
    };

    let children_start_x = if w > children_total_w {
        (w - children_total_w) / 2
    } else {
        0
    };

    let mut positioned_children = Vec::new();
    let mut cx = children_start_x;
    for mut child in child_layouts {
        let child_sw = subtree_width(&child);
        shift_x(&mut child, cx);
        cx += child_sw + H_GAP;
        positioned_children.push(child);
    }

    NodeLayout {
        x: node_x,
        y: 0,
        width: w,
        height: h,
        lines,
        children: positioned_children,
    }
}

fn shift_x(layout: &mut NodeLayout, dx: u32) {
    layout.x += dx;
    for child in &mut layout.children {
        shift_x(child, dx);
    }
}

fn subtree_width(layout: &NodeLayout) -> u32 {
    if layout.children.is_empty() {
        return layout.width;
    }
    let children_total: u32 = layout.children.iter().map(|c| subtree_width(c)).sum::<u32>()
        + (layout.children.len() as u32 - 1) * H_GAP;
    layout.width.max(children_total)
}

fn subtree_height(layout: &NodeLayout) -> u32 {
    if layout.children.is_empty() {
        return layout.height;
    }
    layout.height
        + V_GAP
        + layout.children.iter().map(|c| subtree_height(c)).max().unwrap_or(0)
}

fn draw_node(
    img: &mut RgbImage,
    layout: &NodeLayout,
    ox: u32,
    oy: u32,
    font: &FontRef,
    scale: PxScale,
) {
    let nx = ox + layout.x;
    let ny = oy + layout.y;

    let fill = if layout.children.is_empty() { LEAF_COLOR } else { NODE_COLOR };

    draw_filled_rect_mut(
        img,
        Rect::at(nx as i32, ny as i32).of_size(layout.width, layout.height),
        fill,
    );
    draw_rect_border(img, nx, ny, layout.width, layout.height, BORDER_COLOR);

    for (i, line) in layout.lines.iter().enumerate() {
        let tx = nx + PAD_X;
        let ty = ny + PAD_Y + i as u32 * LINE_HEIGHT;
        draw_text_mut(img, TEXT_COLOR, tx as i32, ty as i32, scale, font, line);
    }

    let parent_cx = nx + layout.width / 2;
    let parent_bot = ny + layout.height;

    for child in &layout.children {
        let child_ox = ox;
        let child_oy = oy + layout.height + V_GAP;
        let child_nx = child_ox + child.x;
        let child_ny = child_oy + child.y;
        let child_cx = child_nx + child.width / 2;

        draw_line_segment_mut(
            img,
            (parent_cx as f32, parent_bot as f32),
            (child_cx as f32, child_ny as f32),
            LINE_COLOR,
        );

        draw_node(img, child, child_ox, child_oy, font, scale);
    }
}

fn draw_rect_border(img: &mut RgbImage, x: u32, y: u32, w: u32, h: u32, color: Rgb<u8>) {
    let x1 = x as f32;
    let y1 = y as f32;
    let x2 = (x + w - 1) as f32;
    let y2 = (y + h - 1) as f32;
    draw_line_segment_mut(img, (x1, y1), (x2, y1), color);
    draw_line_segment_mut(img, (x2, y1), (x2, y2), color);
    draw_line_segment_mut(img, (x2, y2), (x1, y2), color);
    draw_line_segment_mut(img, (x1, y2), (x1, y1), color);
}

fn format_term(term: &Term) -> String {
    match term {
        Term::Atom(c) => c.clone(),
        Term::Variable(v) => v.clone(),
        Term::Compound { functor, args } => {
            if args.is_empty() {
                functor.clone()
            } else {
                let args_str: Vec<String> = args.iter().map(|a| format_term(a)).collect();
                format!("{}({})", functor, args_str.join(", "))
            }
        }
    }
}
