use show_image::{event::{MouseButton, VirtualKeyCode, WindowEvent, WindowKeyboardInputEvent}, winit::platform::windows::WindowExtWindows};

pub struct InputManager {
    pressed_keys: [bool;256],
    last_pressed_keys: [bool;256],

    pressed_buttons: [bool;3],
    last_pressed_buttons: [bool;3],
    
    mouse_position: [f32;2],
    last_mouse_position: [f32;2],
}

impl InputManager {
    pub const fn new() -> InputManager {
        InputManager {
            pressed_keys: [false;256],
            last_pressed_keys: [false;256],
            
            pressed_buttons: [false;3],
            last_pressed_buttons: [false;3],
            
            mouse_position: [0.0,0.0],
            last_mouse_position: [0.0,0.0],
        }
    }

    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput(WindowKeyboardInputEvent {input, ..}) => {
                if let Some(key_code) = input.key_code {
                    self.pressed_keys[key_code as usize] = input.state.is_pressed();
                } 
            }

            WindowEvent::MouseButton(input) => {
                match input.button {
                    MouseButton::Left => self.pressed_buttons[0] = input.state.is_pressed(),
                    MouseButton::Right => self.pressed_buttons[1] = input.state.is_pressed(),
                    MouseButton::Middle => self.pressed_buttons[2] = input.state.is_pressed(),
                    MouseButton::Other(_) => {}
                }
                
            }

            WindowEvent::MouseMove(input) => {
                self.mouse_position = input.position.into();
            }

            _ => {}
        }
    }

    pub fn end_of_frame(&mut self) {
        self.last_pressed_keys = self.pressed_keys.clone();
        self.last_pressed_buttons = self.pressed_buttons.clone();
        self.last_mouse_position = self.mouse_position.clone();
    }

    pub fn is_key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        self.pressed_keys[key_code as usize]
    }
    pub fn is_key_just_pressed(&self, key_code: VirtualKeyCode) -> bool {
        self.pressed_keys[key_code as usize] && !self.last_pressed_keys[key_code as usize]
    }

    pub fn is_button_pressed(&self, button_code: MouseButton) -> bool {
        match button_code {
            MouseButton::Left => self.pressed_buttons[0],
            MouseButton::Right => self.pressed_buttons[1],
            MouseButton::Middle => self.pressed_buttons[2],
            MouseButton::Other(_) => false
        }
    }
    pub fn is_button_just_pressed(&self, button_code: MouseButton) -> bool {
        let index = match button_code {
            MouseButton::Left => 0,
            MouseButton::Right => 1,
            MouseButton::Middle => 2,
            MouseButton::Other(_) => {return false}
        };

        self.pressed_buttons[index] && !self.last_pressed_buttons[index]
    }
    pub fn get_mouse_postion(&self) -> [f32; 2] {
        self.mouse_position
    }
    pub fn get_mouse_delta(&self) -> [f32; 2] {
        let [x,y] = self.mouse_position;
        let [lx,ly] = self.last_mouse_position;
        [x-lx, y-ly]
    }
}

