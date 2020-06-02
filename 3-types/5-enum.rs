#![allow(dead_code)]
#![allow(non_snake_case)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    PATCH,
}

enum Event {
    MouseEvent(i32, i32),
    KeyboardEvent {
        ctlKey: bool,
        altKey: bool,
        key: &'static str,
    },
    EscEvent,
    OtherEvent,
}

enum BitflagHttpMethod{
    GET = 0x01,
    POST = 0x02,
    PUT = 0x04,
    DELETE = 0x08,
    HEAD = 0x10,
    OPTIONS = 0x20,
    CONNECT = 0x40,
    TRACE = 0x80,
    PATCH = 0x100
}

fn check_event(evt: &Event) {
    use Event::*;
    match evt {
        MouseEvent(x, y) => {
            println!("it's a mouse event, clicked the ({},{})", x, y);
        }
        EscEvent => {
            println!("press the 'esc'!");
        }
        KeyboardEvent {
            ctlKey,
            altKey,
            key,
        } => {
            println!(
                "press the keyboard '{}', altKey:{}, ctlKey:{}",
                key, altKey, ctlKey
            );
        }
        _ => {
            println!("unkown event");
        }
    };
}

fn main() {
    // 经典形式的enum类型，每个enum元素都是一个单元结构体，同时会自动对应一个从0开始的索引值
    let method = HttpMethod::GET;
    match method {
        HttpMethod::GET => println!("it's get method, it has an index value {}", method as i32),
        _ => println!("it's not a get method"),
    };
    // 所有结构体形式的类型都可以在enum中使用
    let evt = Event::MouseEvent(0, 0);
    check_event(&evt);
    let evt = Event::KeyboardEvent{
        ctlKey: false,
        altKey: false,
        key: "Z"
    };
    check_event(&evt);
    // 针对上面的经典形式enum，可以使用=符号更改默认索引值，可以使用bitmask之类的crate来做一些位操作
    let allowMethods = BitflagHttpMethod::GET as u32 | BitflagHttpMethod::POST as u32 ;
    if allowMethods & (BitflagHttpMethod::GET as u32) != 0{
        println!("it accept a get method");
    }
    if allowMethods & (BitflagHttpMethod::POST as u32) != 0{
        println!("it accept a post method");
    }
}
