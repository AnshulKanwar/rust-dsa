/*
DESCRIPTION
    Given a value V, we want to make change for ₹V.
    We hav infinite supply of each of the denominations
    of indian currency.

    This solution uses Greedy Approach.

EXAMPLE
    Input: 70
    Output: 2
    We neet a ₹50 and a ₹20 note.
*/

static COINS: [u32; 9] = [1, 2, 5, 10, 20, 50, 100, 500, 1000];

pub fn min_coin(amount: u32) -> u32 {
    let mut change_left = amount;
    let mut n_coins = 0;

    while change_left > 0 {
        let largest = largest_denomination(change_left);
        change_left -= largest;
        n_coins += 1;
    }

    n_coins
}

fn largest_denomination(amount: u32) -> u32 {
    let mut largest = COINS[0];
    for coin in COINS {
        if coin <= amount {
            largest = coin;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_denomination_coin() {
        assert_eq!(largest_denomination(10), 10);
        assert_eq!(largest_denomination(40), 20);
        assert_eq!(largest_denomination(999), 500);
        assert_eq!(largest_denomination(1), 1);
        assert_eq!(largest_denomination(10000), 1000);
    }

    #[test]
    fn min_coins() {
        assert_eq!(coin_change(70), 2);
        assert_eq!(coin_change(121), 3);
        assert_eq!(coin_change(1), 1);
        assert_eq!(coin_change(4512), 7);
    }
}
