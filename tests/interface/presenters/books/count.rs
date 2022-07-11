type Input = ();

type Expect = u32;

struct CountTableBooks;

impl CountTableBooks {
    fn expect(expect: Expect) -> CountTableBooksWE {
        CountTableBooksWE { expect }
    }
}

struct CountTableBooksWE {
    expect: Expect,
}

impl CountTableBooksWE {
    fn input(self, input: Input) -> CountTableBooksWI {
        CountTableBooksWI {
            expect: self.expect,
            input,
        }
    }
}

struct CountTableBooksWI {
    expect: Expect,
    input: Input,
}

impl CountTableBooksWI {
    fn run(self) {
        // Response header "x-total-count: 0"

        // 1. Launch a request with `Input`
        // TODO
        let _input = self.input;
        // reqwest.head(url) -> x_total_count

        // 2. Parse data from response
        let response_result: anyhow::Result<String> = Ok("0".into());
        assert!(response_result.is_ok());

        if let Ok(x_total_count_value) = response_result {
            let result_parse = x_total_count_value.parse::<Expect>();
            assert!(result_parse.is_ok());
            if let Ok(x_total_count) = result_parse {
                // 3. Compare parsed data from response against `Expect`
                assert_eq!(self.expect, x_total_count);
            }
        }
    }
}

#[test]
fn ok_on_count_void_database() {
    CountTableBooks::expect(0).input(()).run()
}

// #[test]
// fn ok_on_count_table_with_3_books() {
//     // 1. Populate table Books with 3 rows
//     CountTableBooks::expect(3).input(()).run()
// }
