use fugazi::Client;

fn main() {
    let c = Client::new("https://api.binance.com/api/v3");
    let r = c.get_avg_price("XRPBTC");
    
    println!("{:?}", r);
}