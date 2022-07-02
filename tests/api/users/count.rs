type Input = ();

type Expect = u32;

struct CountTableUsers;

impl CountTableUsers {
    fn expect(expect: Expect) -> CountTableUsersWE {
        CountTableUsersWE { expect }
    }
}

struct CountTableUsersWE {
    expect: Expect,
}

impl CountTableUsersWE {
    fn input(self, input: Input) -> CountTableUsersWI {
        CountTableUsersWI {
            expect: self.expect,
            input,
        }
    }
}

struct CountTableUsersWI {
    expect: Expect,
    input: Input,
}

impl CountTableUsersWI {
    fn run(self) {
        // Response header "x-total-count: 0"

        // 1. Launch a request with `Input`
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
    CountTableUsers::expect(0).input(()).run()
}

#[test]
fn ok_on_count_table_with_3_users() {
    // 1. Populate table Users with 3 rows
    CountTableUsers::expect(3).input(()).run()
}