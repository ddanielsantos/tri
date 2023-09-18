use std::process::Command;

#[test]
fn it_correctly_displays_folder_structure() {
    let expected = "fixtures
├── arq.txt
├── emp
├── not_emp
    ├── arq.txt
    ├── arq2.txt
    └── really
        └── nested
            └── folder
                └── like
                    └── java
                        └── folks
                            └── do
└── not_not_emp
".to_string();

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests/fixtures")
        .output()
        .expect("failed to execute process");

    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
}