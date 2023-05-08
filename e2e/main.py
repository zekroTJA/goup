import akane


@akane.test()
def install_sdk():
    akane.exec("goup use 1.20.4")
    res = akane.exec("go version")
    akane.assert_true(
        res.startswith("go version go1.20.4"),
        f"res was {res[:-1]}")


@akane.test()
def install_another_sdk():
    akane.exec("goup use 1.19.0")
    res = akane.exec("go version")
    akane.assert_true(
        res.startswith("go version go1.19"),
        f"res was {res[:-1]}")


@akane.test()
def current():
    res = akane.exec("goup current")
    akane.assert_eq("1.19\n", res)


@akane.test()
def check_for_update():
    res = akane.exec("goup check")
    lines = res.split("\n")[0:-1]
    akane.assert_true("New minor version is available!" in lines[-4],
                      f"line -4 was {lines[-4]}")
    akane.assert_true(lines[-3].startswith("1.19"), f"line -3 was {lines[-3]}")
    akane.assert_true("New patch version is available!" in lines[-2],
                      f"line -2 was {lines[-2]}")
    akane.assert_true(lines[-1].startswith("1.19"), f"line -1 was {lines[-1]}")


@ akane.test()
def switch_back():
    akane.exec("goup use 1.20.4")
    res = akane.exec("go version")
    akane.assert_true(
        res.startswith("go version go1.20.4"),
        f"res was {res[:-1]}")


@ akane.test()
def list_local():
    res = akane.exec("goup ls")
    exp = "  1.19\n" \
        "* 1.20.4\n"
    akane.assert_eq(exp, res)


@ akane.test()
def cleanup():
    akane.exec("goup clean --yes")


def main() -> int:
    return akane.run_all()


if __name__ == "__main__":
    exit(main())
