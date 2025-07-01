
/* 
pub const STYLE_ROBOT_LABEL: &str = "
    ...
"; 
*/

pub fn my_style(style1: &str, style2: &str) -> String {
    let s =  format!("{} {}", style1, style2);
    return s;
}

pub const STYLE_ROBOT_LABEL: &str = "
    mx-auto my-10 w-1/3 text-center bg-green-600 text-black text-3xl rounded-xl border-2 border-solid border-amber-400
";
pub const STYLE_CARD_BOX1: &str = "
    w-2/3 flex justify-center items-center bg-slate-800 mx-auto my-5
";

pub const STYLE_CARD_BOX2: &str = "
    border-solid justify-center items-center border-2 border-slate-700 rounded-lg px-3 py-5 w-full
"; 

pub const STYLE_BOX_LABEL_3XL: &str = "
    text-center text-3xl text-gray-200
";

pub const STYLE_ERROR_MESSAGE: &str = "
    bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3
";

pub const STYLE_INPUT_DIV: &str = "
    flex my-5 justify-center mx-auto flex-col w-3/5
";

pub const STYLE_INPUT_LABEL: &str = "
    flex text-lg text-slate-300 text-center
"; 

pub const STYLE_INPUT: &str = "
    flex w-full rounded-lg mx-auto px-2 py-1 bg-slate-100
"; 

pub const STYLE_BUTTON_SKY: &str = "
    flex bg-sky-500 text-slate-200 px-3 py-2 text-bold text-2xl rounded-lg w-1/2 mx-auto my-5 hover:bg-sky-600
"; 

pub const STYLE_BUTTON_NO_COLOR: &str = "
    flex text-slate-200 px-3 py-2 text-bold text-2xl rounded-lg w-1/2 mx-auto my-5 hover:bg-sky-600
"; 