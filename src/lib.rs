use wasm_bindgen::prelude::*;

// bring in the tools needed to communicate between rust and javascript
// this is critical for the #[wasm_bindgen] attribute to work

// 1. simple conversion: decimal to binary
#[wasm_bindgen]
pub fn decimal_to_binary(num: i32) -> String {
    // using the {:b} flag automatically formats the integer as binary
    format!("{:b}", num)
}

// 2. simple conversion: binary to decimal
#[wasm_bindgen]
pub fn binary_to_decimal(input: &str) -> String {
    // try to parse the string as base-2. we use a match statement
    // because user input is unpredictable and might crash the app
    match i32::from_str_radix(input, 2) {
        Ok(num) => num.to_string(),
        Err(_) => "Invalid Binary".to_string(),
    }
}

// 3. visualization: binary -> decimal (bit cards + total sum)
#[wasm_bindgen]
pub fn render_explanation(input: &str) -> String {
    // 1. we create a temporary string just for the cards
    let mut cards_html = String::new();
    let mut total_sum = 0;
    
    // loop backwards (from 2^0 up to 2^N)
    for (index, char) in input.chars().rev().enumerate() {
        let power_val = 2_u32.pow(index as u32);
        let is_active = char == '1';
        
        // track the total for the final box
        if is_active {
            total_sum += power_val;
        }

        let card_class = if is_active { "card active" } else { "card dim" };
        let math_result = if is_active { power_val } else { 0 };

        let card_html = format!(
            r#"
            <div class='{}'>
                <div class='bit'>{}</div>
                <div class='math'>2<sup>{}</sup></div>
                <div class='result'>+ {}</div>
            </div>
            "#, 
            card_class, char, index, math_result
        );

        // insert at start so the big numbers appear on the left
        cards_html.insert_str(0, &card_html);
    }

    // 2. create the big equals sign and the final sum card
    let total_html = format!(
        r#"
        <div class='equals-item'>=</div>
        <div class='card total'>
            <div class='bit'>&Sigma;</div>
            <div class='math'>Sum</div>
            <div class='result'>{}</div>
        </div>
        "#, 
        total_sum
    );

    // 3. return the full container: cards + equals + sum
    return format!("<div class='viz-container'>{}{}</div>", cards_html, total_html);
}

// 4. visualization: decimal -> binary (repeated division)
#[wasm_bindgen]
pub fn render_decimal_viz(mut num: i32) -> String {
    // edge case: if they type 0, just show a static result
    if num == 0 {
        return String::from("<div class='step-card'>0 / 2 = 0 (Rem: <b>0</b>)</div>");
    }

    let mut html_output = String::from("<div class='steps-container'>");
    
    // keep dividing by 2 until we run out of numbers
    while num > 0 {
        let remainder = num % 2; // this is the bit (0 or 1)
        let quotient = num / 2;  // this is the result passed to the next step

        // create a card showing the math for this specific step
        let step_html = format!(
            r#"
            <div class='step-card'>
                <div class='calc'>{} ÷ 2 = {}</div>
                <div class='rem'>Rem: <span class='bit-highlight'>{}</span></div>
            </div>
            "#,
            num, quotient, remainder
        );

        html_output.push_str(&step_html);

        // update the number for the next loop iteration
        num = quotient;
    }

    html_output.push_str("<div class='instruction'>read remainders top to bottom to get binary</div></div>");
    
    return html_output;
}