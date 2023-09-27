use freya::prelude::*;
pub fn app_window(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render!(
        text {
            font_size: "20",
            "Awesome rust gui -- Freya"
        }
        rect {
            height: "20%",
            width: "100%",
            background: "rgb(233, 196, 106)",
            padding: "12",
            color: "rgb(20, 33, 61)",
            label {
                font_size: "20",
                "Number is: {count}"
            }
        }
        rect {
            height: "40%",
            width: "100%",
            background: "rgb(168, 218, 220)",
            color: "black",
            padding: "12",
            onclick: move |_| count += 1,
            label { "Click to increase!" }
        }
        rect {
            height: "40%",
            width: "100%",
            background: "rgb(0, 218, 220)",
            color: "black",
            padding: "12",
            onclick: move |_| count -= 1,
            label { "Click to decrease!" }
        }
    )
}
