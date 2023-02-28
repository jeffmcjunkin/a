use bat::PrettyPrinter;
use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;

pub mod request;

fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("no prompt provided");
        std::process::exit(1);
    }

    let lang = args[0].clone();
    let prompt = args.join(" ");
    let url = String::from("https://api.openai.com/v1/completions");

    let response = request::make_request(url, prompt).expect("Could not make request to API");

    let mut response = String::from(response.strip_prefix("\n\n").unwrap());
    response.push_str("\n");

    #[cfg(feature = "clipboard")]
    {
        let mut ctx = ClipboardContext::new().unwrap();
        ctx.set_contents(response.clone()).unwrap();
    }

    PrettyPrinter::new()
        .input_from_bytes(response.as_bytes())
        .language(&lang)
        .print()
        .unwrap();
}
