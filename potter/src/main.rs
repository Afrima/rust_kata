use std::collections::HashMap;

const BOOK_ID_ARRAY: &[u8] = &[1, 2, 3, 4, 5];
const BOOK_PRICE: f32 = 8.0;

fn main() {
    print!("{}", buy(&[1, 1, 2, 3, 4, 3, 5, 2]));
}

fn buy(book_ids: &[u8]) -> f32 {
    let mut books = map_books(book_ids);
    let mut overall_price = 0.0;
    loop {
        let price: f32 = get_book_price(&mut books);
        if price == 0.0 {
            break;
        }
        overall_price += get_discounted_price(price);
    }
    return (overall_price * 100.0).round() / 100.0;
}

fn get_discounted_price(price: f32) -> f32 {
    if price / BOOK_PRICE == 2.0 {
        return price * 0.95;
    } else if price / BOOK_PRICE == 3.0 {
        return price * 0.90;
    } else if price / BOOK_PRICE == 4.0 {
        return price * 0.80;
    } else if price / BOOK_PRICE == 5.0 {
        return price * 0.75;
    }
    return price;
}

fn get_book_price(books: &mut HashMap<&u8, u8>) -> f32 {
    let mut price: f32 = 0.0;
    for i in BOOK_ID_ARRAY {
        let book = books.get(i);
        if book != None {
            let num = book.expect("no number").clone();
            if num > 0 {
                books.insert(i, num - 1);
                price += BOOK_PRICE;
            }
        }
    }
    return price;
}

fn map_books(book_ids: &[u8]) -> HashMap<&u8, u8> {
    let mut books: HashMap<&u8, u8> = HashMap::new();
    for id in book_ids {
        let count: u8;
        if books.get(id) == None {
            count = 1;
        } else {
            count = books.get(id).expect("no number") + 1;
        }
        books.insert(id, count);
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
