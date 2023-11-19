use crate::{
    trains::{TrainConnection, TrainSchedule, TrainStopInfo},
    TriColor, TIMEZONE,
};
use alloc::string::{String, ToString};
use chrono::Duration;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    mono_font::{
        ascii::{FONT_8X13, FONT_9X15, FONT_9X18_BOLD},
        MonoTextStyle,
    },
    pixelcolor::Rgb888,
    primitives::{Circle, Line, Primitive, PrimitiveStyle, Rectangle},
    text::{renderer::CharacterStyle, Alignment, Text, TextStyle},
    Drawable,
};

pub struct TrainScheduleDrawer<'d, 's, Display>
where
    Display: DrawTarget<Color = TriColor>,
{
    pub display: &'d mut Display,
    pub schedule: &'s TrainSchedule,
    pub offset: Point,
    pub size: Size,
    pub cell_size: Size,
    pub padding: Size,
    pub margin: Size,
    pub header_size: Size,
    pub duration_width: u32,
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds() % 60;
    let minutes = (duration.num_seconds() / 60) % 60;
    let hours = (duration.num_seconds() / 60) / 60;
    if hours > 0 {
        format!("{: >2}:{:0>2}", hours, minutes)
    } else {
        format!("{}", minutes)
    }
}

impl<'d, 't, Display> TrainScheduleDrawer<'d, 't, Display>
where
    Display: DrawTarget<Color = TriColor>,
{
    pub fn new(
        display: &'d mut Display,
        schedule: &'t TrainSchedule,
        offset: Point,
        size: Size,
        padding: Size,
    ) -> Self {
        let duration_width = 50;

        let cell_size = Size::new(
            ((size.width - duration_width) / (schedule.stations.len() as u32)),
            150,
        );
        let margin = padding;

        Self {
            display,
            schedule,
            offset,
            size,
            cell_size,
            padding,
            margin,
            header_size: Size::new(size.width, 30),
            duration_width,
        }
    }

    pub fn display_header(&mut self) -> Result<(), Display::Error> {
        let ts_with_tz = self.schedule.timestamp.with_timezone(TIMEZONE);

        let date = ts_with_tz.format("%d/%m/%Y").to_string();
        let time = ts_with_tz.format("%H:%M").to_string();

        let title_character_style = MonoTextStyle::new(&FONT_8X13, TriColor::Black);

        Text::with_text_style(
            &date,
            Point::new(13, 20),
            title_character_style,
            TextStyle::with_alignment(Alignment::Left),
        )
        .draw(self.display)?;

        Text::with_text_style(
            &time,
            Point::new(self.size.width as i32 / 2, 20),
            title_character_style,
            TextStyle::with_alignment(Alignment::Center),
        )
        .draw(self.display)?;

        // TODO: system error icons

        // Draw separator line
        let separator_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);
        Line::new(
            Point::new(0, self.header_size.height as i32),
            Point::new(
                self.header_size.width as i32,
                self.header_size.height as i32,
            ),
        )
        .into_styled(separator_style)
        .draw(self.display)?;

        Ok(())
    }

    pub fn display_station_names(&mut self) -> Result<(), Display::Error> {
        let station_name_style = MonoTextStyle::new(&FONT_9X15, TriColor::Black);

        let last_station_index = self.schedule.stations.len() - 1;
        let stop_spacing = (self.size.width - self.duration_width - self.padding.width * 2)
            / (self.schedule.stations.len() as u32 - 1);

        let mut offset = Point::new(
            self.padding.width as i32,
            self.header_size.height as i32 + 2 * self.padding.height as i32,
        );

        for (index, station_name) in self.schedule.stations.iter().enumerate() {
            let alignment = match index {
                index if index == 0 => Alignment::Left,
                index if index == last_station_index => Alignment::Right,
                _ => Alignment::Center,
            };

            Text::with_text_style(
                &station_name,
                Point::new(offset.x, offset.y),
                station_name_style,
                TextStyle::with_alignment(alignment),
            )
            .draw(self.display)?;

            offset = Point::new(offset.x + stop_spacing as i32, offset.y);
        }

        Ok(())
    }

    fn display_stop(
        &mut self,
        stop: &Option<TrainStopInfo>,
        offset: Point,
        alignment: Alignment,
    ) -> Result<(), Display::Error> {
        let time_style = MonoTextStyle::new(&FONT_9X18_BOLD, TriColor::Black);

        let time_txt = stop.as_ref().map_or("-".to_string(), |info| {
            info.time
                .with_timezone(TIMEZONE)
                .format("%H:%M")
                .to_string()
        });

        Text::with_text_style(
            &time_txt,
            Point::new(offset.x, offset.y),
            time_style,
            TextStyle::with_alignment(alignment),
        )
        .draw(self.display)?;

        if let Some(info) = &stop {
            if !info.delay.is_zero() {
                let mut delay_style = time_style.clone();
                delay_style.set_text_color(Some(TriColor::Chromatic));

                let delay_txt = format_duration(info.delay);

                Text::with_text_style(
                    &delay_txt,
                    Point::new(offset.x, offset.y + 30),
                    delay_style,
                    TextStyle::with_alignment(Alignment::Left),
                )
                .draw(self.display)?;
            }
        }
        Ok(())
    }

    fn display_connection(
        &mut self,
        connection: &TrainConnection,
        offset: Point,
    ) -> Result<(), Display::Error> {
        let mut offset = Point::new((self.padding.width * 2) as i32, offset.y);

        // Draw track line
        let track_style = PrimitiveStyle::with_stroke(TriColor::Black, 2);
        let line_voffset = 10;
        Line::new(
            Point::new((self.padding.width * 2) as i32, offset.y + line_voffset),
            Point::new(
                (self.size.width - self.duration_width - self.padding.width * 2) as i32,
                offset.y + line_voffset,
            ),
        )
        .into_styled(track_style)
        .draw(self.display)?;

        let duration_style = MonoTextStyle::new(&FONT_9X18_BOLD, TriColor::Black);

        Text::with_text_style(
            &format_duration(connection.duration),
            Point::new(
                (self.size.width - self.padding.width) as i32,
                offset.y + line_voffset + 4,
            ),
            duration_style,
            TextStyle::with_alignment(Alignment::Right),
        )
        .draw(self.display)?;

        let stop_spacing = (self.size.width - self.duration_width - self.padding.width * 4)
            / (connection.stops.len() as u32 - 1);

        for (index, stop) in connection.stops.iter().enumerate() {
            self.display_stop(&stop, offset, Alignment::Center)?;

            // Draw stop dot
            if let Some(info) = &stop {
                let fill_color = if info.delay.is_zero() && !info.canceled {
                    TriColor::Black
                } else {
                    TriColor::Chromatic
                };

                let stop_style = PrimitiveStyle::with_fill(fill_color);

                if !info.canceled {
                    Circle::with_center(Point::new(offset.x, offset.y + line_voffset - 1), 6).into_styled(stop_style).draw(self.display)?;
                } else {
                    Rectangle::with_center(
                        Point::new(offset.x, offset.y + line_voffset - 1),
                        Size::new(6, 6),
                    ).into_styled(stop_style).draw(self.display)?;
                }

            }

            offset = Point::new(offset.x + stop_spacing as i32, offset.y);
        }

        Ok(())
    }

    pub fn display_connections(&mut self, vertical_offset: i32) -> Result<(), Display::Error> {
        if self.schedule.connections.len() == 0 {
            return Ok(());
        }

        let mut offset = Point::new(self.padding.width as i32, 100);

        for (index, connection) in self.schedule.connections.iter().enumerate() {
            self.display_connection(connection, offset)?;
            offset = Point::new(offset.x, offset.y + 100);
        }

        Ok(())
    }
}
