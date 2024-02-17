use std::process::Command;

#[test]
fn test_main_with_args() {
  let output = Command::new("cargo")
    .arg("run")
    .arg("--")
    .arg("-d")
    .arg("./tests/inputsForTest/")
    .output()
    .expect("");

  assert!(output.status.success());
  let stdout = String::from_utf8(output.stdout).expect("Saída não válida UTF-8");
  let expected_output = "Total of 1 file with the repeated name\n\nTotal of 4 files with '.png' extension\nTotal of 1 files with '.mp4' extension\n\n\nTotal of 5 files organized\n"; // Substitua esta string pela saída esperada do seu programa
  assert!(stdout.contains(expected_output));
}