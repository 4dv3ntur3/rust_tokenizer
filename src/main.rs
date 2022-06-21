use tokenizers::tokenizer::{Result, Tokenizer};
use tokenizers::models::wordpiece::WordPiece;

fn main () -> Result<()> {

    let wp_builder = WordPiece::from_file("pretrained/vocab.txt"); // 이것도 parameter로 받아서 처리할 수 있도록 

    let wp = wp_builder
        .unk_token("[UNK]".into())
        .build()?;

    let mut tokenizer = Tokenizer::new(wp);

    // huggingface tokenizer의 경우에는 API 구현이 잘 안돼있네... 
    // split by 
    let mut v = Vec::new();
    v.push("서울시 노연로 80입니다.");
    let encoding = tokenizer.encode_batch(v, false); 

    // let id_1 = tokenizer.id_to_token(11655);

    // unwrap Result 
    let unwrapped = encoding.unwrap();

    // must use getter, setter
    // let input = unwrapped[0].get_ids();
    // let tokens = unwrapped[0].get_tokens();
    // let attention_mask = unwrapped.get_attention_mask();
    // println!("{} {} {}", unwrapped.ids, unwrapped.tokens, unwrapped.attention_mask);

    println!("{:?}", unwrapped);
    // {} {} {} 하려고 했더니 string only 형식인지... 
    // println!("{:?} {:?} {:?}", input, tokens, attention_mask);

    // return input, tokens, attention_mask, padding

    Ok(()) // 이 안에 변수 받을 수 있는 줄 알았는데 return type에 맞는 것만 가능한가... 
}