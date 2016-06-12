/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use app_units::Au;
use display_list::ItemRange;
use euclid::{Point2D, Rect, Size2D};
use types::{BorderRadius, BoxShadowClipMode, ImageRendering};
use types::{ClipRegion, ColorF, FontKey, ImageKey, BorderSide};
use webgl::{WebGLContextId};

define_type! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct BorderDisplayItem {
        pub left: BorderSide,
        pub right: BorderSide,
        pub top: BorderSide,
        pub bottom: BorderSide,
        pub radius: BorderRadius,
    }
}

impl BorderDisplayItem {
    pub fn top_left_inner_radius(&self) -> Size2D<f32> {
        Size2D::new((self.radius.top_left.width - self.left.width).max(0.0),
                    (self.radius.top_left.height - self.top.width).max(0.0))
    }

    pub fn top_right_inner_radius(&self) -> Size2D<f32> {
        Size2D::new((self.radius.top_right.width - self.right.width).max(0.0),
                    (self.radius.top_right.height - self.top.width).max(0.0))
    }

    pub fn bottom_left_inner_radius(&self) -> Size2D<f32> {
        Size2D::new((self.radius.bottom_left.width - self.left.width).max(0.0),
                    (self.radius.bottom_left.height - self.bottom.width).max(0.0))
    }

    pub fn bottom_right_inner_radius(&self) -> Size2D<f32> {
        Size2D::new((self.radius.bottom_right.width - self.right.width).max(0.0),
                    (self.radius.bottom_right.height - self.bottom.width).max(0.0))
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct BoxShadowDisplayItem {
        pub box_bounds: Rect<f32>,
        pub offset: Point2D<f32>,
        pub color: ColorF,
        pub blur_radius: f32,
        pub spread_radius: f32,
        pub border_radius: f32,
        pub clip_mode: BoxShadowClipMode,
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct GradientDisplayItem {
        pub start_point: Point2D<f32>,
        pub end_point: Point2D<f32>,
        pub stops: ItemRange,
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ImageDisplayItem {
        pub image_key: ImageKey,
        pub stretch_size: Size2D<f32>,
        pub image_rendering: ImageRendering,
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct WebGLDisplayItem {
        pub context_id: WebGLContextId,
    }
}
define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct RectangleDisplayItem {
        pub color: ColorF,
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct TextDisplayItem {
        pub glyphs: ItemRange,
        pub font_key: FontKey,
        pub size: Au,
        pub color: ColorF,
        pub blur_radius: Au,
    }
}

define_type! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum SpecificDisplayItem {
        Rectangle(RectangleDisplayItem),
        Text(TextDisplayItem),
        Image(ImageDisplayItem),
        WebGL(WebGLDisplayItem),
        Border(BorderDisplayItem),
        BoxShadow(BoxShadowDisplayItem),
        Gradient(GradientDisplayItem),
    }
}

define_type! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct DisplayItem {
        pub item: SpecificDisplayItem,
        pub rect: Rect<f32>,
        pub clip: ClipRegion,
    }
}
