use crate::enums::{CropMode, Draw};
use iced::{mouse};
use iced::{Application, Command, Subscription, Element, Length, Settings, Theme, Size, Event, Rectangle};
use image::Rgba;
use imageproc::rect::Rect;
use image::{RgbaImage, SubImage, imageops};
use rusttype::{Font, Scale};
pub fn modify_image(crop: CropMode,
                    crop_start: (i32, i32),
                    crop_end: (i32, i32),
                    width: u32,
                    height: u32,
                    draw: Draw,
                    draw_mouse_pressed: bool,
                    draw_figure_press: (i32, i32),
                    draw_figure_released: (i32, i32),
                    draw_text_input: &str,
                    draw_color_slider_value: u32,
                    image_to_modify: &mut Option<DynamicImage>,
                    screenshot_bounds: Option<Rect>,
                    event: Option<Event>) {
    let color;
    match draw_color_slider_value.clone() {
        0..=9 => { color = Rgba([0u8, 0u8, 0u8, 255u8]); }
        10..=19 => { color = Rgba([255u8, 0u8, 0u8, 255u8]); }
        20..=29 => { color = Rgba([255u8, 165u8, 0u8, 255u8]); }
        30..=39 => { color = Rgba([255u8, 255u8, 51u8, 255u8]); }
        40..=49 => { color = Rgba([34u8, 139u8, 34u8, 255u8]); }
        50..=59 => { color = Rgba([0u8, 0u8, 255u8, 255u8]); }
        60..=69 => { color = Rgba([73u8, 0u8, 130u8, 255u8]); }
        70..=79 => { color = Rgba([218u8, 112u8, 238u8, 255u8]); }
        _ => { color = Rgba([255u8, 255u8, 255u8, 255u8]); }
    }
    let window_size = 2.0;
    match draw {
        Draw::FreeHand if crop != CropMode::CropConfirm => {
            let screen = image_to_modify.clone().unwrap();
            match event {
                Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() {
                        let position = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * window_size.clone()) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * window_size.clone()) as i32);
                        image_to_modify = Some(imageproc::drawing::draw_filled_circle(&screen, position, 5, color.clone()));
                    }
                }
                Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                    draw_mouse_pressed = true;
                }
                Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                    draw_mouse_pressed = false;
                }
                _ => {}
            };
        }
        Draw::Circle if crop != CropMode::CropConfirm => {
            let screen = image_to_modify.clone().unwrap();
            match event {
                Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && draw_figure_press == (0, 0) {
                        draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone()) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * window_size.clone()) as i32);
                    }
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && draw_figure_press != (0, 0) {
                        draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone()) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * window_size.clone()) as i32);
                    }
                }
                Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                    draw_mouse_pressed = true;
                }
                Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                    draw_mouse_pressed = false;
                    image_to_modify = Some(imageproc::drawing::draw_hollow_circle(&screen, draw_figure_press.clone(), (((draw_figure_released.0.clone() - draw_figure_press.0.clone()).pow(2) + (draw_figure_released.1.clone() - draw_figure_press.1.clone()).pow(2)) as f64).sqrt() as i32, color.clone()));
                    draw_figure_press = (0, 0);
                    draw_figure_released = (0, 0);
                }
                _ => {}
            };
        }
        Draw::Text if crop != CropMode::CropConfirm => {
            let screen = image_to_modify.clone().unwrap();
            match event {
                Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                    if screenshot_bounds.unwrap().contains(position) {
                        draw_figure_press = ((position.clone().x - screenshot_bounds.clone().unwrap().x) as i32, (position.clone().y - screenshot_bounds.clone().unwrap().y) as i32);
                    }
                }
                Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                    image_to_modify = Some(imageproc::drawing::draw_text(&screen, color.clone(), (draw_figure_press.0.clone() as f32 * window_size.clone()) as i32, (draw_figure_press.1.clone() as f32 * window_size.clone()) as i32, Scale { x: 24.8, y: 24.8 }, &Font::try_from_vec(Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8])).unwrap(), draw_text_input.clone().as_str()));
                    draw_figure_press = (0, 0);
                }
                _ => {}
            };
        }
        Draw::Arrow if crop != CropMode::CropConfirm => {
            let screen = image_to_modify.clone().unwrap();
            match event {
                Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && draw_figure_press == (0, 0) {
                        draw_figure_press = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone()) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y.clone()) * window_size.clone()) as i32);
                    }
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && draw_figure_press != (0, 0) {
                        draw_figure_released = (((position.x.clone() - screenshot_bounds.clone().unwrap().x) * window_size.clone()) as i32, ((position.y.clone() - screenshot_bounds.clone().unwrap().y) * window_size.clone()) as i32);
                    }
                }
                Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                    draw_mouse_pressed = true;
                }
                Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                    draw_mouse_pressed = false;
                    let slope = (draw_figure_released.clone().1 - draw_figure_press.clone().1) as f32 / (draw_figure_released.clone().0 - draw_figure_press.clone().0) as f32;
                    if draw_figure_press.1 > draw_figure_released.1 {
                        let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((draw_figure_released.clone().0 as f32 + (30.0 * slope.clone())), (draw_figure_released.clone().1 as f32 + (30.0 * slope.clone()))), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone());
                        let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((draw_figure_released.clone().0 as f32 + (30.0 * slope.clone())), (draw_figure_released.clone().1 as f32 - (30.0 * slope.clone()))), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone());
                        image_to_modify = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (draw_figure_press.clone().0 as f32, draw_figure_press.clone().1 as f32), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone()));
                    } else {
                        let image_tmp1 = imageproc::drawing::draw_line_segment(&screen, ((draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (draw_figure_released.clone().1 as f32 - (30.0 * slope.clone()))), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone());
                        let image_tmp2 = imageproc::drawing::draw_line_segment(&image_tmp1, ((draw_figure_released.clone().0 as f32 - (30.0 * slope.clone())), (draw_figure_released.clone().1 as f32 + (30.0 * slope.clone()))), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone());
                        image_to_modify = Some(imageproc::drawing::draw_line_segment(&image_tmp2, (draw_figure_press.clone().0 as f32, draw_figure_press.clone().1 as f32), (draw_figure_released.clone().0 as f32, draw_figure_released.clone().1 as f32), color.clone()));
                    }
                    draw_figure_press = (0, 0);
                    draw_figure_released = (0, 0);
                }
                _ => {}
            };
        }
        Draw::Crop => {
            let screen = image_to_modify.clone().unwrap();
            let color = Rgba([255u8, 0u8, 0u8, 255u8]);
            let mut rect = Rect::at(1, 1).of_size(1, 1);
            match event {
                Some(Event::Mouse(mouse::Event::CursorMoved { position })) => {
//println!("{} {}",position.x,position.y);
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && crop_start == (0, 0) {
                        crop_start = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * 1.575) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * 1.575) as i32);
                    }
                    if screenshot_bounds.unwrap().contains(position) && draw_mouse_pressed.clone() && crop_start != (0, 0) {
                        crop_end = (((position.x.clone() - screenshot_bounds.unwrap().x.clone()) * 1.575) as i32, ((position.y.clone() - screenshot_bounds.unwrap().y.clone()) * 1.575) as i32);
                    }
                }
                Some(Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                    draw_mouse_pressed = true;
                }
                Some(Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) => {
                    draw_mouse_pressed = false;
//println!("x1:{} y1:{} x2:{} y3:{}",crop_start.0,crop_start.1,crop_end.0,crop_end.1);
                    if crop != CropMode::CropConfirm {
//Da in alto a sinistra a destra
                        if crop_end.0.clone() - crop_start.0.clone() > 0 && crop_end.1.clone() - crop_start.1.clone() > 0 {
                            width = (crop_end.0.clone() - crop_start.0.clone()) as u32;
                            height = (crop_end.1.clone() - crop_start.1.clone()) as u32;
                            if width > 0 && height > 0 {
                                rect = Rect::at(crop_start.0.clone(), crop_start.1.clone()).of_size(width, height);
                            }
                        }
//Da in alto a destra a sinistra
                        if crop_end.0.clone() - crop_start.0.clone() <= 0 && crop_end.1.clone() - crop_start.1.clone() > 0 {
                            width = (crop_start.0.clone() - crop_end.0.clone()) as u32;
                            height = (crop_end.1.clone() - crop_start.1.clone()) as u32;
                            if width > 0 && height > 0 {
                                rect = Rect::at(crop_end.0.clone(), crop_start.1.clone()).of_size(width, height);
                            }
                        }
//Da in basso a destra a sinistra
                        if crop_end.0.clone() - crop_start.0.clone() <= 0 && crop_end.1.clone() - crop_start.1.clone() <= 0 {
                            width = (crop_start.0.clone() - crop_end.0.clone()) as u32;
                            height = (crop_start.1.clone() - crop_end.1.clone()) as u32;
                            if width > 0 && height > 0 {
                                rect = Rect::at(crop_end.0.clone(), crop_end.1.clone()).of_size(width, height);
                            }
                        }
//Da in basso a sinistra a destra
                        if crop_end.0.clone() - crop_start.0.clone() > 0 && crop_end.1.clone() - crop_start.1.clone() <= 0 {
                            width = (crop_end.0.clone() - crop_start.0.clone()) as u32;
                            height = (crop_start.1.clone() - crop_end.1.clone()) as u32;
                            if width > 0 && height > 0 {
                                rect = Rect::at(crop_start.0.clone(), crop_end.1.clone()).of_size(width, height);
                            }
                        }
                        if width > 0 && height > 0 {
                            image_to_modify = Some(imageproc::drawing::draw_hollow_rect(&screen, rect, color));
                        }
                    }
                    if width > 0 && height > 0 {
                        crop = CropMode::CropConfirm;
                    }
                }
                _ => {}
            };
        }
        _ => {}
    }
}