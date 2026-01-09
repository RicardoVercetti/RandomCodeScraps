#[allow(unused)]
#[derive(Debug, PartialEq)]
struct Shoes {
    size: u32,
    style: Style
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Style {
    Sneaker,
    Sandal,
    Boot
}

#[allow(unused)]
fn shoes_in_size(shoes: Vec<Shoes>, size: u32) -> Vec<Shoes> {
    shoes.into_iter().filter(|i| i.size == size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoes {
                size: 10,
                style: Style::Sneaker
            },
            Shoes {
                size: 13,
                style: Style::Sandal
            },
            Shoes {
                size: 10, 
                style: Style::Boot
            }
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(in_my_size, vec![
            Shoes {
                size: 10,
                style: Style::Sneaker
            }, 
            Shoes {
                size: 10,
                style: Style::Boot
            }
        ]);
    }
}