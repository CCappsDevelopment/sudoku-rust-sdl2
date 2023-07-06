use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct SdlContext {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

impl SdlContext {
    pub fn new() -> SdlContext {
        let (event_pump, canvas) = Self::init_sdl2().unwrap();

        SdlContext {
            event_pump,
            canvas,
        }
    }

    fn init_sdl2() -> Result<(EventPump, Canvas<Window>), String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
    
        // Get the current display mode so we can determine screen dimensions
        let display_mode = video_subsystem.current_display_mode(0)?;
    
        // Calculate window dimensions as percentages of screen dimensions
        let window_width: u32 = ((display_mode.w as f32) * 0.45) as u32;
        let window_height: u32 = ((display_mode.h as f32) * 0.95) as u32;
    
        let window = video_subsystem
            .window("Sudoku", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
    
        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
    
        let event_pump = sdl_context.event_pump()?;

        Ok((event_pump, canvas))
    }
}
