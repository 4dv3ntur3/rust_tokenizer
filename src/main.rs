use tokenizers::tokenizer::{Result, Tokenizer, EncodeInput};
use tokenizers::models::wordpiece::{WordPiece, WordPieceBuilder};

fn tokenizer() -> Result<()> {
    let wp_builder:WordPieceBuilder = WordPiece::from_file("pretrained/vocab.txt");

    let wp = wp_builder
        .unk_token("[UNK]".into())
        .build()?;

    let tokenizer = Tokenizer::new(wp);

    let encoding = tokenizer.encode("안녕하세요!", true); 

    // let id_1 = tokenizer.id_to_token(11655);

    // unwrap Result 
    let unwrapped = encoding.unwrap();

    // must use getter, setter
    let input = unwrapped.get_ids();
    let tokens = unwrapped.get_tokens();
    let attention_mask = unwrapped.get_attention_mask();
    // println!("{} {} {}", unwrapped.ids, unwrapped.tokens, unwrapped.attention_mask);

    // println!("{:?}", unwrapped);
    // {} {} {} 하려고 했더니 string only 형식인지... 
    println!("{:?} {:?} {:?}", input, tokens, attention_mask);

    // return input, tokens, attention_mask, padding

    Ok(())
}

fn main () {
    tokenizer;
}