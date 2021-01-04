use super::{Processor, Status};

/// An IntCode binary Program that will ask for input and output it back. Perfect to test IO.
const ECHO_PROGRAM: [i64; 8] = [3, 3, 104, -1, 1106, 0, 0, 99];

#[test]
fn simple_operations() {
    fn execute_and_compare(start: &[i64], expected: &[i64]) {
        let mut program: Processor = start.into();
        let status = program.run();
        assert!(status.is_ok(), "Program should not have failed !");
        assert_eq!(&program.into_memory()[..], expected);
    }

    execute_and_compare(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    execute_and_compare(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    execute_and_compare(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    execute_and_compare(
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}

#[test]
fn io_operations() {
    let initial = [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    let mut program: Processor = initial.as_ref().into();
    assert_eq!(
        program.run(),
        Ok(Status::RequireInput),
        "The program should ask for input"
    );

    // When input < 8 => output == 999
    program = initial.as_ref().into();
    program.write_int(7);
    assert_eq!(
        program.run(),
        Ok(Status::WithOutput(999)),
        "Output should be 999"
    );

    // When input == 8 => output == 1000
    program = initial.as_ref().into();
    program.write_int(8);
    assert_eq!(
        program.run(),
        Ok(Status::WithOutput(1000)),
        "Output should be 1000"
    );

    // When input > 8 => output == 1001
    program = initial.as_ref().into();
    program.write_int(9);
    assert_eq!(
        program.run(),
        Ok(Status::WithOutput(1001)),
        "Output should be 1001"
    );
}

#[test]
fn relative_offset() {
    fn execute_and_check_outputs(start: &[i64], expected: &[i64]) {
        let mut program: Processor = start.into();
        let mut outputs = [0; 32];
        let (read, _) = program.read_next_array(&mut outputs[..], expected.len());
        assert_eq!(
            read,
            expected.len(),
            "Length of element written into {:?} should be {}",
            outputs,
            expected.len()
        );
        assert_eq!(&outputs[..expected.len()], expected);
    }

    const LARGE_RESULT: i64 = 1_219_070_632_396_864;
    const MIDDLE: i64 = 1_125_899_906_842_624;
    let quine: [i64; 16] = [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let large: [i64; 8] = [1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
    let middle_output: [i64; 3] = [104, MIDDLE, 99];

    execute_and_check_outputs(&quine, &quine);
    execute_and_check_outputs(&large, &[LARGE_RESULT]);
    execute_and_check_outputs(&middle_output, &[MIDDLE]);
}

#[test]
fn ascii() {
    let hello: &str = "Hello world\n";
    let mut program: Processor = ECHO_PROGRAM.as_ref().into();

    assert_eq!(
        program.run(),
        Ok(Status::RequireInput),
        "The program should ask for input"
    );

    program.write_text(hello);
    let (out, _) = program.read_next_line();
    assert_eq!(&out, hello);

    program.write_text("Hello there :");
    program.write_int(154_324);
    let (out, _) = program.read_next_line();
    assert_eq!(&out, "Hello there :\n154324\n");
}

#[test]
fn int_callback() {
    let input: [i64; 5] = [0, 1, 2, 3, 4];
    let mut program: Processor = ECHO_PROGRAM.as_ref().into();

    let mut data = input.iter().copied();
    let mut acc: Vec<i64> = Vec::with_capacity(input.len());
    program.run_with_callbacks(
        0,
        |_| data.next(),
        |_, int| {
            acc.push(int);
            Ok(())
        },
    );

    assert_eq!(input.as_ref(), acc.as_slice());
}

#[test]
fn ascii_callback() {
    let hello: &str = "Hello world\n";
    let mut program: Processor = ECHO_PROGRAM.as_ref().into();

    let mut data = std::iter::once(hello.to_string());
    let mut acc: String = String::with_capacity(hello.len());
    program.run_with_ascii_callbacks(
        0,
        |_| data.next(),
        |_, line| {
            acc.push_str(line);
            Ok(())
        },
    );

    assert_eq!(&acc, hello);
}
