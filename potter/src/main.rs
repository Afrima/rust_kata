use std::collections::HashMap;

const BOOK_ID_ARRAY: [u8; 5] = [1, 2, 3, 4, 5];
const BOOK_PRICE: u16 = 800;

fn main() {
    print!("{}", buy(&[1, 1, 2, 3, 4, 3, 5, 2]));
}

fn buy(book_ids: &[u8]) -> f32 {
    let mut books = map_books(book_ids);
    let mut overall_price = 0;
    while !books.is_empty() {
        let price: u16 = get_book_price(&mut books);
        overall_price += get_discounted_price(price);
    }
    return overall_price as f32 / 100.0;
}

fn get_discounted_price(price: u16) -> u16 {
    let mut price_as_float: f32 = price as f32;
    if price / BOOK_PRICE == 2 {
        price_as_float *= 0.95;
    } else if price / BOOK_PRICE == 3 {
        price_as_float *= 0.90;
    } else if price / BOOK_PRICE == 4 {
        price_as_float *= 0.80;
    } else if price / BOOK_PRICE == 5 {
        price_as_float *= 0.75;
    }
    return price_as_float.round() as u16;
}

fn get_book_price(books: &mut HashMap<u8, u8>) -> u16 {
    let mut price: u16 = 0;
    for &i in &BOOK_ID_ARRAY {
        match books.get(&i) {
            Some(&book_amount) => {
                if book_amount > 0 {
                    books.insert(i, book_amount - 1);
                    price += BOOK_PRICE;
                } else {
                    books.remove(&i);
                }
            }
            None => {}
        }
    }
    return price;
}

fn map_books(book_ids: &[u8]) -> HashMap<u8, u8> {
    let mut books: HashMap<u8, u8> = HashMap::new();
    for &id in book_ids {
        match books.get(&id) {
            Some(&book_amount) => {
                books.insert(id, book_amount + 1);
            }
            None => {
                books.insert(id, 1);
            }
        }
    }
    return books;
}

#[cfg(test)]
mod tests {
    use crate::buy;

    #[test]
    fn test_buy_zero_books() {
        assert_eq!(buy(&mut []), 0.0);
    }

    #[test]
    fn test_buy_1_books() {
        assert_eq!(buy(&mut [1]), 8.0);
    }

    #[test]
    fn test_buy_2_books() {
        assert_eq!(buy(&mut [4, 2]), 15.2);
    }

    #[test]
    fn test_buy_3_books() {
        assert_eq!(buy(&mut [3, 5, 1]), 21.6);
    }

    #[test]
    fn test_buy_4_books() {
        assert_eq!(buy(&mut [2, 4, 5, 3]), 25.6);
    }

    #[test]
    fn test_buy_5_books() {
        assert_eq!(buy(&mut [5, 2, 4, 1, 3]), 30.0);
    }

    #[test]
    fn test_buy_complete_set_with_4_set_with_3_set_with_2_set_with_1_books() {
        assert_eq!(buy(&mut [
            1, 1, 1, 1, 1,
            2, 2, 2, 2,
            3, 3, 3,
            4, 4,
            5]), 100.4);
    }

    #[test]
    fn test_buy_4_complete_sets_and_3_different_books() {
        let price_one_series = 8.00 * 5.00 * 0.75;
        let price_three_books = 8.00 * 3.00 * 0.90;
        let expected_price = price_one_series * 4.00 + price_three_books;
        assert_eq!(buy(&mut [
            5, 5, 5, 5, 5,
            1, 1, 1, 1, 1,
            2, 2, 2, 2,
            3, 3, 3, 3, 3,
            4, 4, 4, 4]), expected_price);
    }
}
