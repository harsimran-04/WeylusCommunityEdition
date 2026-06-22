use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::{HWND, POINT};
use winapi::um::winuser::*;

use tracing::warn;

use crate::input::autopilot_device::AutoPilotDevice;
use crate::input::device::{InputDevice, InputDeviceType};
use crate::protocol::{
    Button, KeyboardEvent, KeyboardEventType, PointerEvent, PointerEventType, PointerType,
    WheelEvent,
};

use crate::capturable::{Capturable, Geometry};

pub struct WindowsInput {
    capturable: Box<dyn Capturable>,
    autopilot_device: AutoPilotDevice,
    pointer_device_handle: *mut HSYNTHETICPOINTERDEVICE__,
    touch_device_handle: *mut HSYNTHETICPOINTERDEVICE__,
    multitouch_map: std::collections::HashMap<i64, POINTER_TYPE_INFO>,
}

impl WindowsInput {
    pub fn new(capturable: Box<dyn Capturable>) -> Self {
        unsafe {
            InitializeTouchInjection(5, TOUCH_FEEDBACK_DEFAULT);
            Self {
                capturable: capturable.clone(),
                autopilot_device: AutoPilotDevice::new(capturable),
                pointer_device_handle: CreateSyntheticPointerDevice(PT_PEN, 1, 1),
                touch_device_handle: CreateSyntheticPointerDevice(PT_TOUCH, 5, 1),
                multitouch_map: std::collections::HashMap::new(),
            }
        }
    }
}

fn map_keyboard_event_code(code: &str) -> Option<u8> {
    let key = match code {
        "KeyA" => b'A',
        "KeyB" => b'B',
        "KeyC" => b'C',
        "KeyD" => b'D',
        "KeyE" => b'E',
        "KeyF" => b'F',
        "KeyG" => b'G',
        "KeyH" => b'H',
        "KeyI" => b'I',
        "KeyJ" => b'J',
        "KeyK" => b'K',
        "KeyL" => b'L',
        "KeyM" => b'M',
        "KeyN" => b'N',
        "KeyO" => b'O',
        "KeyP" => b'P',
        "KeyQ" => b'Q',
        "KeyR" => b'R',
        "KeyS" => b'S',
        "KeyT" => b'T',
        "KeyU" => b'U',
        "KeyV" => b'V',
        "KeyW" => b'W',
        "KeyX" => b'X',
        "KeyY" => b'Y',
        "KeyZ" => b'Z',
        "Digit0" => b'0',
        "Digit1" => b'1',
        "Digit2" => b'2',
        "Digit3" => b'3',
        "Digit4" => b'4',
        "Digit5" => b'5',
        "Digit6" => b'6',
        "Digit7" => b'7',
        "Digit8" => b'8',
        "Digit9" => b'9',
        "Escape" => VK_ESCAPE as u8,
        "Enter" => VK_RETURN as u8,
        "Backspace" => VK_BACK as u8,
        "Tab" => VK_TAB as u8,
        "Space" => VK_SPACE as u8,
        "CapsLock" => VK_CAPITAL as u8,
        "F1" => VK_F1 as u8,
        "F2" => VK_F2 as u8,
        "F3" => VK_F3 as u8,
        "F4" => VK_F4 as u8,
        "F5" => VK_F5 as u8,
        "F6" => VK_F6 as u8,
        "F7" => VK_F7 as u8,
        "F8" => VK_F8 as u8,
        "F9" => VK_F9 as u8,
        "F10" => VK_F10 as u8,
        "F11" => VK_F11 as u8,
        "F12" => VK_F12 as u8,
        "F13" => VK_F13 as u8,
        "F14" => VK_F14 as u8,
        "F15" => VK_F15 as u8,
        "F16" => VK_F16 as u8,
        "F17" => VK_F17 as u8,
        "F18" => VK_F18 as u8,
        "F19" => VK_F19 as u8,
        "F20" => VK_F20 as u8,
        "F21" => VK_F21 as u8,
        "F22" => VK_F22 as u8,
        "F23" => VK_F23 as u8,
        "F24" => VK_F24 as u8,
        "Home" => VK_HOME as u8,
        "ArrowUp" => VK_UP as u8,
        "PageUp" => VK_PRIOR as u8,
        "ArrowLeft" => VK_LEFT as u8,
        "ArrowRight" => VK_RIGHT as u8,
        "End" => VK_END as u8,
        "ArrowDown" => VK_DOWN as u8,
        "PageDown" => VK_NEXT as u8,
        "Insert" => VK_INSERT as u8,
        "Delete" => VK_DELETE as u8,
        "PrintScreen" => VK_SNAPSHOT as u8,
        "ScrollLock" => VK_SCROLL as u8,
        "Pause" => VK_PAUSE as u8,
        "ControlLeft" => VK_LCONTROL as u8,
        "ControlRight" => VK_RCONTROL as u8,
        "AltLeft" => VK_LMENU as u8,
        "AltRight" => VK_RMENU as u8,
        "MetaLeft" => VK_LWIN as u8,
        "MetaRight" => VK_RWIN as u8,
        "ShiftLeft" => VK_LSHIFT as u8,
        "ShiftRight" => VK_RSHIFT as u8,
        "Minus" => VK_OEM_MINUS as u8,
        "Equal" => VK_OEM_PLUS as u8,
        "BracketLeft" => VK_OEM_4 as u8,
        "BracketRight" => VK_OEM_6 as u8,
        "Semicolon" => VK_OEM_1 as u8,
        "Quote" => VK_OEM_7 as u8,
        "Backquote" => VK_OEM_3 as u8,
        "Backslash" => VK_OEM_5 as u8,
        "Comma" => VK_OEM_COMMA as u8,
        "Period" => VK_OEM_PERIOD as u8,
        "Slash" => VK_OEM_2 as u8,
        "Numpad0" => VK_NUMPAD0 as u8,
        "Numpad1" => VK_NUMPAD1 as u8,
        "Numpad2" => VK_NUMPAD2 as u8,
        "Numpad3" => VK_NUMPAD3 as u8,
        "Numpad4" => VK_NUMPAD4 as u8,
        "Numpad5" => VK_NUMPAD5 as u8,
        "Numpad6" => VK_NUMPAD6 as u8,
        "Numpad7" => VK_NUMPAD7 as u8,
        "Numpad8" => VK_NUMPAD8 as u8,
        "Numpad9" => VK_NUMPAD9 as u8,
        "NumpadDecimal" => VK_DECIMAL as u8,
        "NumLock" => VK_NUMLOCK as u8,
        "NumpadDivide" => VK_DIVIDE as u8,
        "NumpadMultiply" => VK_MULTIPLY as u8,
        "NumpadSubtract" => VK_SUBTRACT as u8,
        "NumpadAdd" => VK_ADD as u8,
        "NumpadEnter" => VK_RETURN as u8,
        _ => return None,
    };

    Some(key)
}

fn toggle_virtual_key(vk: u8, down: bool) {
    let flags = if down { 0 } else { KEYEVENTF_KEYUP };
    unsafe { keybd_event(vk, 0, flags, 0) };
}

impl InputDevice for WindowsInput {
    fn send_wheel_event(&mut self, event: &WheelEvent) {
        unsafe { mouse_event(MOUSEEVENTF_WHEEL, 0, 0, event.dy as DWORD, 0) };
    }

    fn send_pointer_event(&mut self, event: &PointerEvent) {
        if let Err(err) = self.capturable.before_input() {
            warn!("Failed to activate window, sending no input ({})", err);
            return;
        }
        let Geometry::VirtualScreen(offset_x, offset_y, width, height, left, top) =
            self.capturable.geometry().unwrap()
        else {
            unreachable!()
        };

        let (x, y) = (
            (event.x * width as f64) as i32 + offset_x,
            (event.y * height as f64) as i32 + offset_y,
        );
        let mut pointer_flags = match event.event_type {
            PointerEventType::DOWN => {
                POINTER_FLAG_INRANGE | POINTER_FLAG_INCONTACT | POINTER_FLAG_DOWN
            }
            PointerEventType::MOVE | PointerEventType::OVER | PointerEventType::ENTER => {
                POINTER_FLAG_INRANGE | POINTER_FLAG_UPDATE
            }
            PointerEventType::UP => POINTER_FLAG_UP,
            PointerEventType::CANCEL | PointerEventType::LEAVE | PointerEventType::OUT => {
                POINTER_FLAG_INRANGE | POINTER_FLAG_UPDATE | POINTER_FLAG_CANCELED
            }
        };
        let button_change_type = match event.buttons {
            Button::PRIMARY => {
                pointer_flags |= POINTER_FLAG_INCONTACT;
                POINTER_CHANGE_FIRSTBUTTON_DOWN
            }
            Button::SECONDARY => POINTER_CHANGE_SECONDBUTTON_DOWN,
            Button::AUXILARY => POINTER_CHANGE_THIRDBUTTON_DOWN,
            Button::NONE => POINTER_CHANGE_NONE,
            _ => POINTER_CHANGE_NONE,
        };
        if event.is_primary {
            pointer_flags |= POINTER_FLAG_PRIMARY;
        }
        match event.pointer_type {
            PointerType::Pen => {
                unsafe {
                    let mut pointer_type_info = POINTER_TYPE_INFO {
                        type_: PT_PEN,
                        u: std::mem::zeroed(),
                    };
                    *pointer_type_info.u.penInfo_mut() = POINTER_PEN_INFO {
                        pointerInfo: POINTER_INFO {
                            pointerType: PT_PEN,
                            pointerId: event.pointer_id as u32,
                            frameId: 0,
                            pointerFlags: pointer_flags,
                            sourceDevice: 0 as *mut winapi::ctypes::c_void, //maybe use syntheticPointerDeviceHandle here but works with 0
                            hwndTarget: 0 as HWND,
                            ptPixelLocation: POINT { x: x, y: y },
                            ptHimetricLocation: POINT { x: 0, y: 0 },
                            ptPixelLocationRaw: POINT { x: x, y: y },
                            ptHimetricLocationRaw: POINT { x: 0, y: 0 },
                            dwTime: 0,
                            historyCount: 1,
                            InputData: 0,
                            dwKeyStates: 0,
                            PerformanceCount: 0,
                            ButtonChangeType: button_change_type,
                        },
                        penFlags: PEN_FLAG_NONE,
                        penMask: PEN_MASK_PRESSURE
                            | PEN_MASK_ROTATION
                            | PEN_MASK_TILT_X
                            | PEN_MASK_TILT_Y,
                        pressure: (event.pressure * 1024f64) as u32,
                        rotation: event.twist as u32,
                        tiltX: event.tilt_x,
                        tiltY: event.tilt_y,
                    };
                    InjectSyntheticPointerInput(self.pointer_device_handle, &pointer_type_info, 1);
                }
            }
            PointerType::Touch => {
                unsafe {
                    let mut pointer_type_info = POINTER_TYPE_INFO {
                        type_: PT_TOUCH,
                        u: std::mem::zeroed(),
                    };

                    let mut pointer_touch_info: POINTER_TOUCH_INFO = std::mem::zeroed();
                    pointer_touch_info.pointerInfo = std::mem::zeroed();
                    pointer_touch_info.pointerInfo.pointerType = PT_TOUCH;
                    pointer_touch_info.pointerInfo.pointerFlags = pointer_flags;
                    pointer_touch_info.pointerInfo.pointerId = event.pointer_id as u32; //event.pointer_id as u32; Using the actual pointer id causes errors in the touch injection
                    pointer_touch_info.pointerInfo.ptPixelLocation = POINT { x, y };
                    pointer_touch_info.touchFlags = TOUCH_FLAG_NONE;
                    pointer_touch_info.touchMask = TOUCH_MASK_PRESSURE;
                    pointer_touch_info.pressure = (event.pressure * 1024f64) as u32;

                    pointer_touch_info.pointerInfo.ButtonChangeType = button_change_type;

                    *pointer_type_info.u.touchInfo_mut() = pointer_touch_info;
                    self.multitouch_map
                        .insert(event.pointer_id, pointer_type_info);
                    let len = self.multitouch_map.len();

                    let mut pointer_type_info_vec: Vec<POINTER_TYPE_INFO> = Vec::new();
                    for (_i, info) in self.multitouch_map.iter().enumerate() {
                        pointer_type_info_vec.push(*info.1);
                    }
                    let b: Box<[POINTER_TYPE_INFO]> = pointer_type_info_vec.into_boxed_slice();
                    let m: *mut POINTER_TYPE_INFO = Box::into_raw(b) as _;

                    InjectSyntheticPointerInput(self.touch_device_handle, m, len as u32);

                    match event.event_type {
                        PointerEventType::DOWN
                        | PointerEventType::MOVE
                        | PointerEventType::OVER
                        | PointerEventType::ENTER => {}

                        PointerEventType::UP
                        | PointerEventType::CANCEL
                        | PointerEventType::LEAVE
                        | PointerEventType::OUT => {
                            self.multitouch_map.remove(&event.pointer_id);
                        }
                    }
                }
            }
            PointerType::Mouse => {
                let mut dw_flags = 0;

                let (screen_x, screen_y) = (
                    (event.x * width as f64) as i32 + left,
                    (event.y * height as f64) as i32 + top,
                );

                match event.event_type {
                    PointerEventType::DOWN => match event.buttons {
                        Button::PRIMARY => {
                            dw_flags |= MOUSEEVENTF_LEFTDOWN;
                        }
                        Button::SECONDARY => {
                            dw_flags |= MOUSEEVENTF_RIGHTDOWN;
                        }
                        Button::AUXILARY => {
                            dw_flags |= MOUSEEVENTF_MIDDLEDOWN;
                        }
                        _ => {}
                    },
                    PointerEventType::MOVE | PointerEventType::OVER | PointerEventType::ENTER => {
                        unsafe { SetCursorPos(screen_x, screen_y) };
                    }
                    PointerEventType::UP => match event.button {
                        Button::PRIMARY => {
                            dw_flags |= MOUSEEVENTF_LEFTUP;
                        }
                        Button::SECONDARY => {
                            dw_flags |= MOUSEEVENTF_RIGHTUP;
                        }
                        Button::AUXILARY => {
                            dw_flags |= MOUSEEVENTF_MIDDLEUP;
                        }
                        _ => {}
                    },
                    PointerEventType::CANCEL | PointerEventType::LEAVE | PointerEventType::OUT => {
                        dw_flags |= MOUSEEVENTF_LEFTUP;
                    }
                }
                unsafe { mouse_event(dw_flags, 0 as u32, 0 as u32, 0, 0) };
            }
            PointerType::Unknown => todo!(),
        }
    }

    fn send_keyboard_event(&mut self, event: &KeyboardEvent) {
        if let Some(key) = map_keyboard_event_code(&event.code) {
            let down = match event.event_type {
                KeyboardEventType::UP => false,
                KeyboardEventType::DOWN | KeyboardEventType::REPEAT => true,
            };
            toggle_virtual_key(key, down);
        } else {
            self.autopilot_device.send_keyboard_event(event);
        }
    }

    fn set_capturable(&mut self, capturable: Box<dyn Capturable>) {
        self.capturable = capturable;
    }

    fn device_type(&self) -> InputDeviceType {
        InputDeviceType::WindowsInput
    }
}
