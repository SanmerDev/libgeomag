from pathlib import Path


def main():
    root_folder = Path(__file__).resolve().parent
    cof_file = root_folder.joinpath("WMM.COF")
    cof = cof_file.read_text()

    c = []
    for line in cof.splitlines():
        v = line.strip().split()[2:]
        v = [float(v) for v in v]
        c.append(v)

    l0 = len(c[0])
    l1 = len(c)
    print(f"const WMM_COEFFICIENTS: [[f64; {l0}]; {l1}] = {c};")


if __name__ == "__main__":
    main()
